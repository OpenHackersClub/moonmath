#!/usr/bin/env bash
#
# scripts/acceptance.sh
#
# Minimal HTTP-level acceptance suite that runs against a deployed MoonMath
# Worker. Walks the same route list the prerender step generates, asserts
# every route returns 200, and spot-checks key markers (branding, KaTeX,
# hydration script).
#
# This is the CI-side smoke gate — the full chromiumoxide-driven suite from
# PRD v0.2.6 lands separately. The contract here is "if this fails, the
# deploy is broken end-to-end and should be rolled back".
#
# Usage:
#   scripts/acceptance.sh https://moonmath.example.workers.dev
#   BASE_URL=https://... scripts/acceptance.sh
#
# Env:
#   BASE_URL                 fallback if $1 omitted
#   ACCEPTANCE_MAX_WAIT      seconds to wait for the URL to become reachable (default 60)
#   ACCEPTANCE_SSG_DATA_DIR  override path to SSG data (default target/ssg-data)

set -euo pipefail

BASE_URL="${1:-${BASE_URL:-}}"
SSG_DATA_DIR="${ACCEPTANCE_SSG_DATA_DIR:-target/ssg-data}"
MAX_WAIT="${ACCEPTANCE_MAX_WAIT:-60}"

log()  { printf '[acceptance] %s\n' "$*" >&2; }
fail() { printf '[acceptance] FAIL: %s\n' "$*" >&2; exit 1; }

[[ -n "$BASE_URL" ]] || fail "BASE_URL not set (pass as \$1 or env var)"
BASE_URL="${BASE_URL%/}"  # strip trailing slash

command -v curl >/dev/null || fail "curl is required"
command -v jq   >/dev/null || fail "jq is required"

# ─── Wait for deployment to propagate ───────────────────────────────────────
# Cloudflare Workers can take a few seconds to be reachable globally after
# `wrangler deploy` returns. Poll until we get a 200 on / or hit MAX_WAIT.

log "waiting for $BASE_URL/ (max ${MAX_WAIT}s)"
ready=0
for _ in $(seq 1 "$MAX_WAIT"); do
  if curl -sfL -o /dev/null --max-time 5 "$BASE_URL/"; then
    ready=1
    break
  fi
  sleep 1
done
[[ "$ready" -eq 1 ]] || fail "$BASE_URL/ never became reachable"
log "deployment reachable"

# ─── Enumerate routes ───────────────────────────────────────────────────────
# Same shape as scripts/prerender.sh — single source of truth is the SSG data.

declare -a routes=("/" "/inspirations" "/showcase")

categories_json="$SSG_DATA_DIR/showcase/categories.json"
[[ -f "$categories_json" ]] || fail "$categories_json missing — run 'cargo run -p moonmath-ssg' first"

while IFS= read -r cat_slug; do
  routes+=("/showcase/${cat_slug}")
  pages_json="$SSG_DATA_DIR/showcase/${cat_slug}/pages.json"
  if [[ -f "$pages_json" ]]; then
    while IFS= read -r page_slug; do
      routes+=("/showcase/${cat_slug}/${page_slug}")
    done < <(jq -r '.[].slug' "$pages_json")
  fi
done < <(jq -r '.[].slug' "$categories_json")

log "checking ${#routes[@]} routes"

# ─── Per-route status check ─────────────────────────────────────────────────
# Workers Static Assets propagation isn't atomic — a fresh deploy can have `/`
# serving 200 while child routes still 404 on some edge POPs for ~10–30s.
# Retry 404s a handful of times before declaring failure, since it's cheap and
# almost always a propagation race rather than a real bug.

retry_get() {
  local url="$1" body_path="$2" code
  for attempt in 1 2 3 4 5; do
    code=$(curl -sSL -o "$body_path" -w "%{http_code}" --max-time 15 "$url" || echo "000")
    case "$code" in
      404|000|5*) sleep "$attempt"; continue ;;  # likely propagation/transient — backoff
      *) printf '%s' "$code"; return 0 ;;
    esac
  done
  printf '%s' "$code"
}

failed=0
tmp_body="$(mktemp)"
trap 'rm -f "$tmp_body"' EXIT

for route in "${routes[@]}"; do
  # -L follows redirects; the Worker uses html_handling = "auto-trailing-slash"
  # which 307s `/foo` → `/foo/`, so we need to chase the final response.
  http_code=$(retry_get "${BASE_URL}${route}" "$tmp_body")
  if [[ "$http_code" != "200" ]]; then
    log "  FAIL $route → HTTP $http_code (after retries)"
    failed=$((failed + 1))
    continue
  fi
  # Every page must carry the brand and the hydration script.
  if ! grep -q "MoonMath" "$tmp_body"; then
    log "  FAIL $route → missing brand marker 'MoonMath'"
    failed=$((failed + 1))
    continue
  fi
  if ! grep -q '/pkg/' "$tmp_body"; then
    log "  FAIL $route → missing hydration assets reference '/pkg/'"
    failed=$((failed + 1))
    continue
  fi
  log "  ok   $route"
done

# ─── Content-specific assertions ────────────────────────────────────────────
# Spot-check a known page to confirm KaTeX server-side rendering survived
# the deploy. We pick the first showcase page from the first category.

first_cat=$(jq -r '.[0].slug' "$categories_json")
first_page_slug=$(jq -r '.[0].slug' "$SSG_DATA_DIR/showcase/${first_cat}/pages.json" 2>/dev/null || echo "")
if [[ -n "$first_page_slug" ]]; then
  spot_url="${BASE_URL}/showcase/${first_cat}/${first_page_slug}"
  log "spot-checking KaTeX on $spot_url"
  spot_code=$(retry_get "$spot_url" "$tmp_body")
  if [[ "$spot_code" != "200" ]]; then
    log "  FAIL katex spot-check → HTTP $spot_code (after retries)"
    failed=$((failed + 1))
  elif ! grep -q 'class="katex' "$tmp_body"; then
    log "  FAIL katex spot-check → no .katex span found"
    failed=$((failed + 1))
  else
    log "  ok   katex spot-check"
  fi
fi

# ─── 404 handling ───────────────────────────────────────────────────────────

log "checking 404 handling"
nf_code=$(curl -sS -o "$tmp_body" -w "%{http_code}" --max-time 15 "${BASE_URL}/__moonmath_does_not_exist__" || echo "000")
if [[ "$nf_code" != "404" ]]; then
  log "  FAIL 404 handling → got HTTP $nf_code (expected 404)"
  failed=$((failed + 1))
else
  log "  ok   404 handling"
fi

# ─── Lean compile endpoint ──────────────────────────────────────────────────
# Skips by default — Cloudflare Containers cold-start can take 5+ seconds
# and we don't want to gate every PR preview on the container build.
# Opt-in by setting ACCEPTANCE_CHECK_LEAN_COMPILE=1.

if [[ "${ACCEPTANCE_CHECK_LEAN_COMPILE:-0}" == "1" ]]; then
  log "checking POST /api/CompileLean"
  # Server-fn `PostUrl` input format: URL-encoded form, field name `code`.
  # We pick a trivial proof so this works with the no-Mathlib v0 image.
  compile_code='theorem t : 1 = 1 := rfl'
  compile_body="code=$(jq -rn --arg c "$compile_code" '$c|@uri')"
  compile_status=$(curl -sS -o "$tmp_body" -w "%{http_code}" --max-time 30 \
    -X POST "${BASE_URL}/api/CompileLean" \
    -H 'Content-Type: application/x-www-form-urlencoded' \
    --data "$compile_body" || echo "000")
  if [[ "$compile_status" != "200" ]]; then
    log "  FAIL compile → HTTP $compile_status, body: $(head -c 200 "$tmp_body")"
    failed=$((failed + 1))
  elif ! jq -e '.success == true' "$tmp_body" >/dev/null 2>&1; then
    log "  FAIL compile → success != true, body: $(head -c 200 "$tmp_body")"
    failed=$((failed + 1))
  else
    log "  ok   compile (trivial proof verified)"
  fi
fi

# ─── Result ─────────────────────────────────────────────────────────────────

if (( failed > 0 )); then
  fail "$failed acceptance check(s) failed against $BASE_URL"
fi
log "all acceptance checks passed against $BASE_URL"
