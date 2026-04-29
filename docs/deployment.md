# MoonMath — Cloudflare Worker Deployment

Implements the deployment target defined in `specs/prd.md` → "Deployment (Cloudflare Workers)" (PRD milestone v0.2.5).

## What gets deployed

A single Cloudflare Worker that

1. serves the prerendered SSG output from Workers Static Assets (`dist/`),
2. dispatches `POST /api/CompileLean` (the `compile_lean` server fn) to a Cloudflare Container running Lean — see [`docs/lean-service.md`](./lean-service.md), and
3. exposes a raw `POST /api/lean/compile` proxy for non-Leptos callers (legacy of v0.2.5; same container).

```
repo/
├── wrangler.toml              # Worker config (Static Assets + container + envs)
├── worker/
│   ├── src/index.ts           # Worker entrypoint (routing, KV cache)
│   ├── src/lean-compiler.ts   # Container Durable Object class
│   ├── package.json           # wrangler + @cloudflare/containers
│   └── tsconfig.json
├── services/lean/
│   ├── Dockerfile             # Lean toolchain + Bun HTTP wrapper
│   └── server.ts              # POST /compile { code } → CompileResponse JSON
├── scripts/prerender.sh       # SSR → static HTML stop-gap (until v0.1.7)
├── .github/workflows/deploy.yml
└── dist/                      # Built locally or in CI; gitignored
```

## Stop-gap: prerender script

The PRD assumes the v0.1.7 SSG migration has landed, after which `cargo leptos build` writes static HTML directly. Until then, `scripts/prerender.sh` boots the existing SSR Axum server, walks every route, and dumps HTML into `dist/` in the layout Workers Static Assets expects.

Routes are enumerated from `target/ssg-data/*.json` — adding a new `content/showcase/*` page automatically extends coverage with no script changes.

The script will be deleted once v0.1.7 lands.

## First-time setup

1. **Cloudflare account.** Get an API token with `Workers Scripts:Edit` and `Account:Read` scopes, plus the account ID.
2. **Repo secrets.** Add `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` to GitHub Actions secrets.
3. **Worker deps.** `cd worker && npm install`
4. **(Optional) Rate-limit KV.** `npx wrangler kv namespace create LEAN_RATE_LIMIT` and paste the printed `id` / `preview_id` into `wrangler.toml` (uncomment the `[[kv_namespaces]]` block).
5. **(Optional) Compile-cache KV.** `npx wrangler kv namespace create LEAN_CACHE` and `... --preview`; paste both ids into the `LEAN_CACHE` block in `wrangler.toml`. Without it `/api/CompileLean` still works but every request hits the container.
6. **(Optional) Custom domain.** Uncomment the `routes = [...]` line under `[env.production]` in `wrangler.toml` once the zone is set up. Until then deploys land on the `*.workers.dev` subdomain.

The `LeanCompiler` Durable Object + container image are provisioned automatically by `wrangler deploy` — no separate registry push.

## Local commands

```sh
# 1. Generate SSG data + WASM bundle.
cargo run -p moonmath-ssg
cargo leptos build --release

# 2. Prerender routes to ./dist (boots the SSR binary, walks routes via curl).
./scripts/prerender.sh

# 3. Local Worker preview against ./dist on :8787.
cd worker && npm run dev

# 4. Deploy.
cd worker && npm run deploy            # production
cd worker && npm run deploy:staging    # staging
```

## CI/CD

`.github/workflows/deploy.yml`:

| Trigger | Action |
| --- | --- |
| `push` to default branch | builds + prerenders + deploys `--env production` |
| `pull_request` | preview deploy at `moonmath-preview-pr-<n>.workers.dev` and a sticky comment with the URL |
| `workflow_dispatch` | choose `staging` or `production` |

The workflow caches Cargo builds via `Swatinem/rust-cache` and uses `cloudflare/wrangler-action@v3`.

## Cache policy

Set in `worker/src/index.ts` per the PRD:

| Path | Cache-Control |
| --- | --- |
| `/pkg/*` (content-hashed WASM/JS/CSS) | `public, max-age=31536000, immutable` |
| `/data/*` (SSG JSON) | `public, max-age=60, must-revalidate` |
| `/...` (HTML pages) | `public, max-age=60, must-revalidate` + security headers |

## Observability

`wrangler.toml` enables `[observability] enabled = true`, which surfaces request logs and basic metrics (count, error rate, p95 latency) in the Cloudflare dashboard. For long-form analysis, hook Logpush from the dashboard — there's nothing to configure in this repo.

## Rollback

```sh
cd worker && npx wrangler rollback --env production
```

`wrangler rollback` reverts the latest deployment; `wrangler deployments list` shows what's live.

## Related milestones

- **v0.1.7 — SSG migration** (TODO). Removes the need for `scripts/prerender.sh`.
- **v0.2.6 — Acceptance test gate** (TODO). Adds a CDP-driven required job between build and deploy.
- **v0.3.5 — Immersive Mode**. Ships a per-page `immersive.json` artifact alongside the static HTML; no Worker changes expected.
