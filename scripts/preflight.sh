#!/usr/bin/env bash
#
# scripts/preflight.sh
#
# Fast-fail local checks for the Cloudflare Worker deploy stack. Catches the
# class of bugs CI would otherwise discover after a 10+ minute Rust build:
#
#   - wrangler.toml schema (env split, [[containers]] format, instance_type)
#   - Dockerfile build for services/lean (apt deps, Bun installer, Lean toolchain)
#   - Worker TypeScript typecheck
#
# What this does NOT cover (Cloudflare's local tooling can't):
#   - Cloudflare Containers don't run under `wrangler dev` — only the docker
#     build is verified here. Container runtime behavior still needs a deploy.
#   - Workers Static Assets propagation race (see scripts/acceptance.sh).
#
# Usage:
#   ./scripts/preflight.sh                 # all checks
#   PREFLIGHT_SKIP_DOCKER=1 ./scripts/preflight.sh   # skip the docker build
#   PREFLIGHT_SKIP_WRANGLER=1 ./scripts/preflight.sh # skip wrangler dry-run
#
# Exits non-zero on the first failing check; prints a one-line summary at the end.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

log()  { printf '[preflight] %s\n' "$*" >&2; }
fail() { printf '[preflight] FAIL: %s\n' "$*" >&2; exit 1; }
warn() { printf '[preflight] WARN: %s\n' "$*" >&2; }

ran=()
skipped=()

# Docker availability — tested once and reused. Wrangler 4.x requires Docker
# to be running even for `--dry-run` when wrangler.toml has [[containers]],
# because it walks the container build plan.
docker_state="down"
if command -v docker >/dev/null && docker info >/dev/null 2>&1; then
  docker_state="up"
fi

# ─── 1. Worker TypeScript typecheck ─────────────────────────────────────────

log "(1/3) typechecking worker/"
if [[ ! -d worker/node_modules ]]; then
  log "      installing worker deps (one-time)"
  (cd worker && npm install --no-audit --no-fund >/dev/null)
fi
(cd worker && npm run --silent typecheck) || fail "worker typecheck failed"
ran+=("worker typecheck")

# ─── 2. wrangler.toml dry-run ───────────────────────────────────────────────
# Catches: env split inheritance bugs, [[containers]] schema regressions,
# instance_type renames, missing bindings, wrong [vars] keys.
#
# `wrangler deploy --dry-run` does NOT need network or auth — it parses
# wrangler.toml and walks the deploy plan offline.

if [[ "${PREFLIGHT_SKIP_WRANGLER:-0}" == "1" ]]; then
  log "(2/3) skipping wrangler dry-run (PREFLIGHT_SKIP_WRANGLER=1)"
  skipped+=("wrangler dry-run")
elif [[ "$docker_state" == "down" ]]; then
  warn "(2/3) docker daemon down — skipping wrangler dry-run (it requires docker for [[containers]] validation)"
  skipped+=("wrangler dry-run (docker down)")
else
  log "(2/3) wrangler deploy --dry-run"
  # `--dry-run` errors if dist/ doesn't exist; create an empty placeholder
  # so we exercise wrangler.toml parsing without needing a real build.
  if [[ ! -d dist ]]; then
    mkdir -p dist
    : >dist/index.html
    PREFLIGHT_DIST_PLACEHOLDER=1
  fi
  trap '[[ -n "${PREFLIGHT_DIST_PLACEHOLDER:-}" && -f dist/index.html && ! -s dist/index.html ]] && rm -rf dist' EXIT

  out=$(cd worker && npx --no-install wrangler deploy --dry-run --outdir /tmp/preflight-wrangler 2>&1) || {
    printf '%s\n' "$out" >&2
    fail "wrangler dry-run rejected wrangler.toml"
  }
  # Surface warnings even on success — they often foreshadow CI errors.
  if grep -qE 'WARNING' <<<"$out"; then
    printf '%s\n' "$out" >&2
    warn "wrangler emitted warnings (above) — investigate before deploy"
  fi
  ran+=("wrangler dry-run")
fi

# ─── 3. Docker build for services/lean ──────────────────────────────────────
# Catches: missing apt packages, broken curl|sh installers, Lean toolchain
# version drift, COPY paths. Skipped automatically when Docker isn't running
# (developer machine without daemon up).

if [[ "${PREFLIGHT_SKIP_DOCKER:-0}" == "1" ]]; then
  log "(3/3) skipping docker build (PREFLIGHT_SKIP_DOCKER=1)"
  skipped+=("docker build")
elif [[ "$docker_state" == "down" ]]; then
  warn "(3/3) docker not available — skipping container build check"
  skipped+=("docker build (docker down)")
else
  log "(3/3) docker build services/lean (this is the slow check; ~30s warm, ~5min cold)"
  if ! docker build --quiet services/lean -t moonmath-lean:preflight >/dev/null; then
    fail "docker build failed for services/lean"
  fi
  ran+=("docker build")
fi

# ─── Summary ────────────────────────────────────────────────────────────────

log "ran:     ${ran[*]:-(none)}"
if (( ${#skipped[@]} > 0 )); then
  log "skipped: ${skipped[*]}"
fi
log "all preflight checks passed"
