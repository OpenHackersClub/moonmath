# MoonMath — Lean Compile Service (Cloudflare Container)

How `POST /api/CompileLean` is served on the Cloudflare deployment.

## Why a container

Lean ships as a native toolchain — there's no in-process Worker option. Three places "the rest" can live on Cloudflare:

| Option | Verdict |
| --- | --- |
| **Cloudflare Containers** ✅ | Worker spawns a long-lived container, fetches into it. Pay only while serving. Used here. |
| Workers + WASM in-process | ❌ 10 MB code-size cap. Lean.wasm is 80 MB+, won't fit. |
| External host (Fly.io / Hetzner) behind the Worker | Works but moves the deploy story off Cloudflare. We can fall back here later via `LEAN_FALLBACK_URL`. |

## Request path

```
client (compile_lean(code))
  │  POST /api/CompileLean
  │  Content-Type: application/x-www-form-urlencoded
  │  body: code=<urlencoded>
  ▼
Cloudflare Worker (worker/src/index.ts)
  │  1. Decode form, extract `code`
  │  2. Rate limit per IP (LEAN_RATE_LIMIT KV)
  │  3. Cache check: sha256(code) → LEAN_CACHE KV
  │  4. On miss, getContainer(env.LEAN_COMPILER).fetch(...)
  ▼
LeanCompiler container (services/lean/)
  │  Bun HTTP server :8080
  │  POST /compile { code }
  │  → spawn `lean scratch.lean`
  │  → parse stdout, return CompileResponse JSON
  ▼
Worker writes upstream body into LEAN_CACHE (24h TTL),
returns 200 application/json to the client.

Errors → text body `ServerError|<message>` so the Leptos
client renders a real message instead of "Could not deserialize error".
```

## What the Worker owns

The Worker is the edge: routing, auth, rate-limit, cache, error shaping. **It does not run Lean.** Specifically:

- **Edge cache** — `LEAN_CACHE` KV keyed by `sha256(code)`, 24 h TTL. Most demoed proofs hit the cache and return in ~30 ms.
- **Rate-limit** — per-IP via `LEAN_RATE_LIMIT` KV (20 req/min, same window the legacy `/api/lean/compile` route uses).
- **Body cap** — `LEAN_REQUEST_MAX_BYTES` (32 KB default).
- **Wire-format translation** — Leptos server-fn `PostUrl` form body → JSON `{code}` for the container; container JSON response is forwarded as-is.
- **Error shaping** — non-2xx responses use the `ServerError|<msg>` body format that `server_fn::ServerFnError::de` parses. This is what makes user-visible errors readable.

## What the container owns

A minimal Lean runner. See `services/lean/`:

- `Dockerfile` — Ubuntu 24.04 + elan + `leanprover/lean4:v4.28.0` (mirrors `lean-project/lean-toolchain`) + Bun for the HTTP wrapper. ~350 MB compressed.
- `server.ts` — Bun HTTP server on `:8080`. `POST /compile { code }` writes the source to a tmp file, runs `lean`, parses diagnostics into the `CompileResponse` shape from `crates/moonmath-types`. Error parsing mirrors `crates/moonmath-lean/src/compiler.rs::parse_lean_errors` so local SSR and the Cloudflare path produce identical records.

## Known limitation: no Mathlib in v0

The container ships **standalone Lean only**. Showcase snippets that use Mathlib (`Real.log`, `ℝ`, `norm_num`, `linarith`, …) will return Lean import errors until the Mathlib follow-up.

Why: a full Mathlib build runs ~5–7 GB compressed; Cloudflare Containers cap images at 2 GB. Path forward, in order of effort:

1. **Subset Mathlib** — pre-build only the modules needed for current showcase pages (`Mathlib.Analysis.Real`, `Mathlib.Data.Nat.Prime`, …). Likely fits under 2 GB.
2. **Pre-built `.olean` cache as a separate volume** — once Cloudflare Containers ships persistent volumes (announced, not GA at time of writing), mount Mathlib oleans separately from the image.
3. **External Mathlib host** — keep the container for non-Mathlib snippets, fall back to a Fly.io Lean+Mathlib service via `LEAN_FALLBACK_URL` when imports require it.

Tracked separately. Do not block this PR on Mathlib.

## Cost model (rough)

Cloudflare Containers bill on memory × time + CPU × time, with no idle charge once the container sleeps.

| Knob | Value |
| --- | --- |
| Instance type | `standard` (4 GB RAM, 0.5 vCPU) |
| `sleepAfter` | `10m` |
| `max_instances` | `5` |
| Cold start | ~2–5 s (image pull on first hit per region) |
| Warm compile | ~1–3 s for v0 snippets (no Mathlib) |
| Cache hit | ~30 ms (KV roundtrip only) |

A single demo session (10 button clicks, 1 cold start, 9 warm/cached) costs roughly $0.001. Pricing model assumes a small number of unique proofs cached aggressively; if that breaks down (open-ended editor, unique proofs every request) raise the rate-limit and revisit `instance_type`.

## Local development

`wrangler dev` will not spin up real containers — it currently runs the Worker against a stub. To exercise the full path locally, run the container and the Worker side-by-side:

```sh
# Build + run the Lean container directly.
docker build -t moonmath-lean services/lean
docker run --rm -p 8080:8080 moonmath-lean

# In another shell, start the Worker pointed at the local container
# (uses LEAN_FALLBACK_URL in dev — the [[durable_objects.bindings]] only
# works against a deployed Worker today).
LEAN_FALLBACK_URL=http://127.0.0.1:8080/compile \
  cd worker && npm run dev

# Smoke test:
curl -s -X POST http://127.0.0.1:8787/api/lean/compile \
  -H 'Content-Type: application/json' \
  -d '{"code":"theorem t : 1 = 1 := rfl"}' | jq .
```

For the full Cloudflare-side path including the Container DO, deploy a preview (`wrangler deploy --name moonmath-preview-pr-<n>`) and hit `/api/CompileLean` against the preview URL.

## Operational notes

- **Bumping Lean version**: edit both `services/lean/Dockerfile` (`LEAN_VERSION` build arg) and `lean-project/lean-toolchain` so local SSR and the container stay in sync. Mismatched versions silently produce different error messages.
- **Watching usage**: Cloudflare's "Containers" dashboard surfaces concurrent instance count and CPU-seconds. The `X-Lean-Cache: hit|miss` response header from the Worker is a quick proxy for cache pressure; tail it with `wrangler tail`.
- **Disabling**: comment out the `[[durable_objects.bindings]]` and `[[containers]]` blocks in `wrangler.toml` and redeploy. The Worker returns `ServerError|Lean compile container is not bound on this deployment.` — the page stays usable, just no compile button.
