# MoonMath

Interactive, animated explanations of mathematical formulas, algorithm visualizations, and machine-checked Lean4 proofs — built in Rust, compiled to WebAssembly, served from a single Cloudflare Worker.

**Live:** https://moonmath.openhackers.club

## What it is

MoonMath is a static site (Leptos SSR + hydration today, full SSG in v0.1.7) where every showcase page pairs a human-readable proof with:

- **Animated formula display** — step-through derivations with term highlighting (KaTeX server-rendered, animated client-side)
- **Algorithm visualizations** — Rust/WASM-powered, interactive with custom inputs
- **Lean4 formalization** — every showcase carries a Lean4 snippet; `lean4_status` tracks completeness (`complete` / `partial` / `sorry` / `planned`)
- **Immersive Mode** — full-screen `eframe`/egui playback of the same content as a "reader's theatre" animation (`/showcase/:category/:slug/immersive`)

See [`specs/prd.md`](specs/prd.md) for the full product spec.

## Tech stack

- **Leptos 0.7.8** + Axum 0.7 (SSR + hydration)
- **cargo-leptos** v0.3.4 build tooling
- **katex-rs** for server-side math rendering
- **pulldown-cmark** + **gray_matter** for Markdown + TOML frontmatter
- **egui / eframe** for the immersive renderer and Lean4 editor
- **Cloudflare Workers** + Workers Static Assets for deployment

## Workspace

| Crate | Role |
|---|---|
| `moonmath-app` | Leptos app — routes, components, server functions, Axum entrypoint |
| `moonmath-types` | Shared types (`Frontmatter`, `ContentPage`, `CompileRequest`, …) |
| `moonmath-content` | Content pipeline (markdown + TOML frontmatter, KaTeX preprocess) — SSR-only |
| `moonmath-math` | KaTeX server-side rendering — SSR-only |
| `moonmath-lean` | Lean4 compiler integration (shells out to `lean`) — SSR-only |
| `moonmath-ssg` | Build-time data generator |
| `moonmath-egui` / `moonmath-viz` | Immersive renderer + visualization stubs (v0.2+) |

## Quickstart

Requires `cargo-leptos` (`cargo install cargo-leptos`).

```sh
# Dev server with hot reload (SSG first, then cargo leptos watch)
cargo dev

# Regenerate showcase data
cargo ssg

# Type-check both targets
cargo check --features ssr     -p moonmath-app
cargo check --features hydrate -p moonmath-app --target wasm32-unknown-unknown

# Run tests
cargo test
```

Dev server runs at http://127.0.0.1:3000 (reload on 3001).

## Deployment (Cloudflare Worker)

Production target is a single Worker fronting Workers Static Assets. Full guide in [`docs/deployment.md`](docs/deployment.md).

```sh
# 1. Build everything
cargo run -p moonmath-ssg
cargo leptos build --release

# 2. Prerender all routes to ./dist (stop-gap until v0.1.7 SSG)
./scripts/prerender.sh

# 3. Local Worker preview on :8787
cd worker && npm install && npm run dev

# 4. Deploy
cd worker && npm run deploy            # production
cd worker && npm run deploy:staging    # staging
```

CI (`.github/workflows/deploy.yml`) builds, prerenders, and runs `wrangler deploy`. PRs get preview Workers at `moonmath-preview-pr-<n>.workers.dev`.

## Roadmap

- **v0.1** ✅ Leptos shell, content pipeline, KaTeX, cargo-leptos build
- **v0.1.7** SSG migration — `cargo leptos build` writes static HTML directly
- **v0.2** egui canvas + Leptos↔egui bridge (Immersive Mode foundation)
- **v0.2.5** ✅ Cloudflare Worker deploy + acceptance gate
- **v0.3** Algorithm visualizations
- **v0.4** Lean4 IDE (egui-based editor, in-browser compile via WASM)
- **v0.5** LeanTeX → LaTeX → KaTeX + Typst PDF export

## License

[MIT](LICENSE) © Open Hackers Club
