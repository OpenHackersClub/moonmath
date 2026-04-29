# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This project uses **cargo-leptos** (v0.3.4+). Install with `cargo install cargo-leptos`.

```sh
# Dev server with hot reload (runs SSG first, then cargo leptos watch)
cargo dev

# Or run SSG alone to regenerate showcase data
cargo ssg

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

## Deployment (Cloudflare Worker)

Production target is a single Cloudflare Worker fronting Workers Static Assets. See `docs/deployment.md` for full details.

```sh
# 1. Build everything.
cargo run -p moonmath-ssg
cargo leptos build --release

# 2. Prerender all routes to ./dist (boots SSR, walks routes via curl).
./scripts/prerender.sh

# 3. Local Worker preview against ./dist on :8787.
cd worker && npm install && npm run dev

# 4. Deploy.
cd worker && npm run deploy            # → moonmath.<account>.workers.dev
```

CI: `.github/workflows/deploy.yml` runs steps 1–3 then `wrangler deploy`. PRs get a preview Worker at `moonmath-preview-pr-<n>.workers.dev`.

`scripts/prerender.sh` is a stop-gap until milestone v0.1.7 (SSG migration) lands and `cargo leptos build` writes static HTML directly.

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

## Development Workflow — Test-Driven & Verify-First

**Every change must be verified before it is considered done.** Follow this checklist after any code change:

### Mandatory verification steps

1. **Both build targets compile:**
   ```sh
   cargo check --features ssr -p moonmath-app
   cargo check --features hydrate -p moonmath-app --target wasm32-unknown-unknown
   ```
2. **All tests pass:**
   ```sh
   cargo test --features ssr -p moonmath-app   # smoke tests
   cargo test                                   # full workspace
   ```
3. **SSG data is current** (if content/types changed):
   ```sh
   cargo run -p moonmath-ssg
   ```
4. **Server starts and pages render** (for UI/routing changes):
   ```sh
   LEPTOS_OUTPUT_NAME=moonmath-app target/debug/moonmath-app &
   curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/
   curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/showcase/number-theory/prime-theorem
   ```
5. **Cloudflare deploy preflight** (for changes touching `wrangler.toml`, `worker/`, `services/lean/`, or `.github/workflows/deploy.yml`):
   ```sh
   ./scripts/preflight.sh    # worker typecheck + wrangler dry-run + docker build
   ```
   Catches the class of bugs CI would otherwise discover after a 10+ minute Rust build (env-split inheritance, `[[containers]]` schema, missing apt deps in the Lean container, etc.). Wrangler 4.x's dry-run requires the Docker daemon to be running; the script auto-skips wrangler/docker checks when Docker is down (with a warning) and still runs the typecheck.

### Test-driven approach

- **Write tests first** when adding new functionality — or immediately after implementation
- **Use the test-writer agent** (`.claude/agents/test-writer.md`) after completing feature work
- **Never assume compilation = correctness** — Leptos SSR builds can compile fine while WASM hydration panics at runtime. Always verify BOTH targets
- **Hydration hazards**: `use_location()`, `use_params_map()`, and other router hooks MUST be called inside a `<Router>` context. Calling them before `<Router>` renders will panic on hydration and silently break ALL client-side interactivity (compile buttons, fractal canvases, etc.)
- **`#[cfg(feature = "hydrate")]` blocks** that use router hooks should be in components rendered INSIDE `<Router>`, not in the `App` component before `<Router>`

### When using team agents

After parallel agents finish, **always run the full verification checklist** in the main context. Agents edit files independently and may introduce cross-file issues (e.g., one agent's change causes another's code to break at hydration time).

## File Creation Rules

When creating or writing `.lean`, `.md`, `.rs`, or any other content files, always use the **Write** tool (or **Edit** tool for modifications) instead of `cat`, `echo`, or heredoc via Bash. This avoids unnecessary permission prompts and keeps file operations trackable.

## Team Agents

For multi-file or cross-concern tasks, prefer spawning a team of agents to work in parallel. Suggested roles mapped to this codebase:

| Agent name | Scope | Typical files |
|---|---|---|
| **rust-components** | Leptos pages, components, server functions | `crates/moonmath-app/src/pages/`, `crates/moonmath-app/src/components/` |
| **styling** | CSS, layout, responsive design, theming | `crates/moonmath-app/style/main.css` |
| **content-pipeline** | Markdown processing, frontmatter, KaTeX math | `crates/moonmath-content/`, `crates/moonmath-math/`, `content/` |
| **lean-tooling** | Lean4 compiler integration, LeanTeX | `crates/moonmath-lean/` |
| **build-verify** | Run `cargo check --features ssr`, WASM check, `cargo test` | (read-only, Bash only) |
| **test-writer** | Write and run tests for any new/changed code | All `#[cfg(test)]` modules, `smoke_tests.rs` |
| **ux-reviewer** | Design consistency, theme compliance, responsive layout | `crates/moonmath-app/style/main.css`, all pages + components |

### When to use a team

- Any task touching **both** Rust components and CSS (e.g., new page, redesign)
- Feature work spanning **3+ crates** (e.g., adding a new showcase with content + rendering + UI)
- Large refactors where build verification should run continuously alongside edits

### How to structure

1. Create tasks with `TaskCreate` for each concern (Rust, CSS, content, etc.)
2. Spawn agents with `Task` tool using `team_name` — assign `subagent_type="general-purpose"` for edit work, `subagent_type="Bash"` for build-verify
3. The **build-verify** agent should run `cargo check --features ssr -p moonmath-app` and `cargo check --features hydrate -p moonmath-app --target wasm32-unknown-unknown` after teammates finish edits
4. Keep CSS and Rust agents independent — they touch different files and can always run in parallel
