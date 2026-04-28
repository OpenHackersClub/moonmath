#!/usr/bin/env bash
#
# scripts/prerender.sh
#
# Stop-gap static-site exporter for MoonMath. Boots the existing SSR Axum
# server, walks every route, and writes the rendered HTML into ./dist/ in the
# layout Workers Static Assets expects:
#
#   dist/
#   ├── index.html
#   ├── 404.html
#   ├── inspirations/index.html
#   ├── showcase/
#   │   ├── index.html
#   │   ├── number-theory/index.html
#   │   └── number-theory/prime-theorem/index.html
#   ├── pkg/...                (cargo-leptos hashed WASM/JS/CSS)
#   └── data/showcase/...      (SSG JSON consumed by hydrated pages)
#
# Routes are enumerated from target/ssg-data/showcase/categories.json + each
# category's pages.json — adding a new content/showcase/* page automatically
# extends coverage with no script changes.
#
# This script will be deleted once milestone v0.1.7 (SSG migration) lands and
# `cargo leptos build` produces static HTML directly. Until then, deploys to
# Cloudflare run this in CI before `wrangler deploy`.

set -euo pipefail

PORT="${MOONMATH_PRERENDER_PORT:-3030}"
HOST="127.0.0.1"
BASE_URL="http://${HOST}:${PORT}"
DIST_DIR="${MOONMATH_DIST_DIR:-dist}"
SSR_BIN="${MOONMATH_SSR_BIN:-target/release/moonmath-app}"
SSG_DATA_DIR="target/ssg-data"
SITE_PKG_DIR="target/site/pkg"

log() { printf '[prerender] %s\n' "$*" >&2; }
die() { printf '[prerender] error: %s\n' "$*" >&2; exit 1; }

# ─── Pre-flight ──────────────────────────────────────────────────────────────

command -v curl >/dev/null || die "curl is required"
command -v jq >/dev/null || die "jq is required (brew install jq)"

[[ -x "$SSR_BIN" ]] || die "SSR binary not found at $SSR_BIN — run 'cargo leptos build --release' first"
[[ -d "$SSG_DATA_DIR" ]] || die "$SSG_DATA_DIR missing — run 'cargo run -p moonmath-ssg' first"
[[ -d "$SITE_PKG_DIR" ]] || die "$SITE_PKG_DIR missing — run 'cargo leptos build --release' first"

# ─── Boot SSR server ─────────────────────────────────────────────────────────

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

log "starting SSR binary on :$PORT"
LEPTOS_OUTPUT_NAME="moonmath-app" \
  LEPTOS_SITE_ADDR="${HOST}:${PORT}" \
  LEPTOS_SITE_ROOT="target/site" \
  LEPTOS_SITE_PKG_DIR="pkg" \
  "$SSR_BIN" >/tmp/moonmath-prerender-ssr.log 2>&1 &
SSR_PID=$!
trap 'log "stopping SSR (pid $SSR_PID)"; kill "$SSR_PID" 2>/dev/null || true; wait "$SSR_PID" 2>/dev/null || true' EXIT

# Poll until ready — give it up to 30 seconds.
for _ in $(seq 1 60); do
  if curl -sf "$BASE_URL/" -o /dev/null; then
    log "SSR ready"
    break
  fi
  sleep 0.5
done
if ! curl -sf "$BASE_URL/" -o /dev/null; then
  log "SSR never came up — last 50 lines of log:"
  tail -n 50 /tmp/moonmath-prerender-ssr.log >&2 || true
  die "SSR boot timed out"
fi

# ─── Enumerate routes ────────────────────────────────────────────────────────

declare -a routes=("/" "/inspirations" "/showcase")

categories_json="$SSG_DATA_DIR/showcase/categories.json"
[[ -f "$categories_json" ]] || die "$categories_json missing"

while IFS= read -r cat_slug; do
  routes+=("/showcase/${cat_slug}")
  pages_json="$SSG_DATA_DIR/showcase/${cat_slug}/pages.json"
  if [[ -f "$pages_json" ]]; then
    while IFS= read -r page_slug; do
      routes+=("/showcase/${cat_slug}/${page_slug}")
    done < <(jq -r '.[].slug' "$pages_json")
  fi
done < <(jq -r '.[].slug' "$categories_json")

log "prerendering ${#routes[@]} routes"

# ─── Fetch each route ────────────────────────────────────────────────────────

failed=0
for route in "${routes[@]}"; do
  # `/foo/bar` → `dist/foo/bar/index.html`; `/` → `dist/index.html`.
  if [[ "$route" == "/" ]]; then
    out="$DIST_DIR/index.html"
  else
    out="$DIST_DIR${route}/index.html"
  fi
  mkdir -p "$(dirname "$out")"

  http_code=$(curl -sS -o "$out" -w "%{http_code}" "${BASE_URL}${route}" || echo "000")
  if [[ "$http_code" != "200" ]]; then
    log "FAIL $route → HTTP $http_code"
    failed=$((failed + 1))
  else
    log "  ok  $route"
  fi
done

# 404 page — fetch a deliberately-missing route. Leptos returns the shell with
# the in-app fallback rendered, which is what we want for the static 404.
mkdir -p "$DIST_DIR"
curl -sS "${BASE_URL}/__moonmath_does_not_exist__" -o "$DIST_DIR/404.html" || true

# ─── Copy assets ─────────────────────────────────────────────────────────────

log "copying $SITE_PKG_DIR → $DIST_DIR/pkg"
mkdir -p "$DIST_DIR/pkg"
cp -R "$SITE_PKG_DIR/." "$DIST_DIR/pkg/"

log "copying $SSG_DATA_DIR → $DIST_DIR/data"
mkdir -p "$DIST_DIR/data"
cp -R "$SSG_DATA_DIR/." "$DIST_DIR/data/"

# Public assets bundled by cargo-leptos (favicon, etc.) live alongside pkg/ in
# target/site — copy anything there that isn't pkg/.
if [[ -d target/site ]]; then
  while IFS= read -r -d '' entry; do
    name=$(basename "$entry")
    [[ "$name" == "pkg" ]] && continue
    cp -R "$entry" "$DIST_DIR/"
  done < <(find target/site -mindepth 1 -maxdepth 1 -print0)
fi

if (( failed > 0 )); then
  die "$failed route(s) failed to render"
fi

log "wrote $DIST_DIR/ — ready for 'wrangler deploy'"
