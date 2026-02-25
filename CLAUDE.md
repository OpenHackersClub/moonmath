# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This project uses **cargo-leptos** (v0.3.4+). Install with `cargo install cargo-leptos`.

```sh
# Dev server with hot reload (primary development command)
cargo leptos watch

# Build for release
cargo leptos build --release

# Check SSR build (server)
cargo check --features ssr -p moonmath-app

# Check WASM build (client)
cargo check --features hydrate -p moonmath-app --target wasm32-unknown-unknown

# Run tests (inline #[cfg(test)] modules in moonmath-content, moonmath-math)
cargo test

# Run tests for a single crate
cargo test -p moonmath-content

# Check a non-app crate
cargo check -p moonmath-lean
```

The dev server runs at `http://127.0.0.1:3000` (reload on port 3001).

## Architecture

**Leptos 0.7.8 SSR + Hydration** — The server renders full HTML with Axum, then the client hydrates with a WASM bundle. The `ssr` and `hydrate` features are mutually exclusive and control which code paths compile.

### Workspace Crates

- **moonmath-app** — Leptos application. Has two compilation targets: `cdylib` (WASM) and `rlib` (server). Contains routes, components, server functions, and the Axum main entry point.
- **moonmath-types** — Shared types (`Frontmatter`, `ContentPage`, `CompileRequest`, `CompileResponse`, etc.). No feature gates, compiles everywhere.
- **moonmath-content** — Content pipeline: walks `content/` dir, parses TOML frontmatter (`+++...+++` delimiters via `gray_matter`), renders markdown with LaTeX math preservation. **SSR-only** (doesn't compile to WASM).
- **moonmath-math** — KaTeX server-side rendering via `katex-rs`. **SSR-only**.
- **moonmath-lean** — Lean4 compiler integration (shells out to `lean` binary). **SSR-only**.
- **moonmath-egui** / **moonmath-viz** — Stubs for future egui visualization (v0.2+).

### Feature Gate Pattern

SSR-only crates (`moonmath-content`, `moonmath-math`, `moonmath-lean`) are declared `optional = true` in moonmath-app's Cargo.toml and listed under the `ssr` feature. Code that uses them must be gated:

```rust
#[cfg(feature = "ssr")]
fn server_only_code() { ... }
```

Server functions use `#[server(Name, "/api")]` and are automatically gated. The `gray_matter` dep is also SSR-only.

### Content Pipeline

Content lives in `content/` with Zola-like TOML frontmatter:
```
content/formulas/calculus/chain-rule.md  →  /formulas/calculus/chain-rule
content/algorithms/sorting/quicksort.md  →  /content/algorithms/sorting/quicksort
```

`_index.md` files define sections (title, description, child ordering). The `loader.rs` walks the directory tree; `markdown.rs` preprocesses `$...$` and `$$...$$` into `data-latex` HTML attributes for KaTeX rendering.

### Routing

Routes use Leptos `FlatRoutes` with `StaticSegment` / `ParamSegment`:
- `/` → `HomePage`
- `/formulas/:section/:slug` → `FormulaPage` (server function loads from disk)
- `/content/:path` → `ContentPage`
- `/showcase/prime-theorem` → `PrimeShowcasePage`

### Math Rendering

Server-side: `katex-rs` (Duktape engine) pre-renders LaTeX to HTML. Client-side: KaTeX CSS (loaded from CDN) styles the pre-rendered output. No client-side JS KaTeX needed for v0.1.

## Leptos 0.7 API Notes

- Imports: `use leptos::prelude::*;`
- `<A>` component: use `attr:class="..."` not `class="..."`
- Shell function pattern for SSR entry, `hydrate_body(App)` for client
- Leptos 0.7 pins to **Axum 0.7** (not 0.8) and **Tower 0.4** (not 0.5)
- Server functions: `#[server(FnName, "/api")]`
- Async in components: use `Resource::new` + `<Suspense>` + `Suspend::new`

## CSS

Single stylesheet at `crates/moonmath-app/style/main.css`. Dark theme with CSS variables (`--color-accent`, `--color-surface`, `--color-border`, etc.). Font: `--font-mono` for code, `--font-sans` for prose.

## Milestones

- v0.1: **Done** — Leptos shell, content pipeline, KaTeX, cargo-leptos build
- v0.2: TODO — egui canvas + Leptos↔egui bridge
- v0.3: TODO — Algorithm visualization
- v0.4: In progress — Lean4 compilation (showcase page done, full IDE pending)
- v0.5: TODO — LeanTeX→LaTeX→KaTeX + Typst PDF export

Full PRD at `specs/prd.md`.

## File Creation Rules

When creating or writing `.lean`, `.md`, `.rs`, or any other content files, always use the **Write** tool (or **Edit** tool for modifications) instead of `cat`, `echo`, or heredoc via Bash. This avoids unnecessary permission prompts and keeps file operations trackable.
