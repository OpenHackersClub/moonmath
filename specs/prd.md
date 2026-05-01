# MoonMath — Product Requirements Document

## Overview

MoonMath is a Rust-based **static site** compiled to WebAssembly (WASM) that provides interactive, animated explanations of mathematical formulas and algorithm visualizations. It also supports compiling Lean4 proof code and rendering it in LaTeX or Typst. The site is statically generated at build time (SSG) with all computation happening client-side in WASM — no server required at runtime.

## Goals

- Make mathematical concepts accessible through animated, interactive visualizations
- Leverage Rust + WASM for high-performance rendering in the browser
- Provide a seamless workflow for writing, compiling, and rendering Lean4 proofs
- Support LaTeX and Typst as output formats for formal math content

## Core Features

### 1. Animated Math Formula Display

- Render mathematical formulas with step-by-step animated breakdowns
- Support standard math notation (LaTeX-compatible input)
- Allow users to control animation speed, pause, rewind, and step through derivations
- Highlight individual terms, substitutions, and transformations as they occur

### 2. Algorithm Visualization (WASM)

- Interactive visualizations of common algorithms (sorting, graph traversal, cryptographic primitives, etc.)
- Rust-powered computation compiled to WASM for near-native performance
- Users can supply custom inputs and observe execution in real time
- Visual elements: data structure diagrams, state transitions, complexity annotations

### 3. Lean4 Proof Snippets (Every Showcase Page)

Every showcase page includes a Lean4 formalization of the theorem or a key lemma. This is a core differentiator: machine-checked proofs alongside human-readable explanations.

- Lean4 code blocks with syntax highlighting (build-time rendered)
- `lean4_status` field tracks completeness: `complete` (fully proven), `partial` (key lemmas proven), `sorry` (statement formalized, proof WIP), `planned`
- Client-side compilation via lean4web WASM port — users can modify and re-check proofs in the browser
- egui-based code editor (pure Rust, no JS) for editing Lean4 code
- Render compiled Lean4 output as:
  - **LaTeX** — downloadable `.tex` files and in-browser preview
  - **Typst** — downloadable `.typ` files and in-browser preview

### 4. Immersive Mode

Every showcase page links to a full-screen **Immersive Mode** that narrates the page's markdown content as a pure animation — no scrolling, no chrome, no clickable HTML widgets. The intent is a "reader's theatre" for mathematics: the proof unfolds on a canvas the way a teacher would draw it on a board.

- **Entry point:** Each showcase page header carries an "Open in Immersive Mode" action (icon button, also bound to the `i` keyboard shortcut). Category and home pages may link directly to a category-level immersive playlist that runs through every premier page in order.
- **URL pattern:** `/showcase/:category/:slug/immersive` (deep-linkable, shareable). A query string controls playback (`?t=<step>&speed=<rate>&reduced-motion=1`).
- **Renderer:** `eframe` (egui via WASM) takes over the viewport. It is the *sole* renderer — no DOM math, no HTML overlays. KaTeX-rendered HTML from the static page is *not* reused; LaTeX is re-rendered into the egui scene as glyphs/paths so it can be animated, transformed, and highlighted as first-class scene objects.
- **Source of truth:** The animation script is derived from the same markdown that produces the static page. Headings, prose paragraphs, math blocks, and Lean4 code blocks become a sequence of typed scene nodes (`Heading`, `Prose`, `Formula`, `LeanBlock`, `Diagram`, `Pause`). A build-time pass in `moonmath-content` emits a deterministic `immersive.json` (or compact binary) per page next to the rendered HTML. No new authoring file is required.
- **Animation primitives:** entrance/exit transitions, term highlighting and re-substitution within a formula, step-by-step proof reveal, camera focus, and timed pacing. Diagrams and visualizations declared on a page (e.g. Mandelbrot canvas, IFS sliders) appear as embedded scenes inside the same egui frame.
- **Controls:** spacebar play/pause, arrow keys for prev/next step, `[`/`]` for speed, `Esc` to return to the article view. A thin progress bar overlays the bottom edge. Controls auto-hide after 2s of inactivity.
- **Accessibility:** respects `prefers-reduced-motion` (transitions become instant cuts), provides a "View transcript" toggle that overlays the original markdown text, and exposes step boundaries to keyboard users. All narration text is selectable as plain text via an off-screen DOM mirror so screen readers are not locked out.
- **Performance budget:** the immersive scene must hit 60fps on a 2020-class laptop for the full proof of any premier page; `moonmath-viz` owns the render loop and shares the canvas with `moonmath-egui`.

## Architecture

### Static Site Generation (SSG)

MoonMath is a **statically generated site**. All pages are pre-rendered at build time with no runtime server:

- **Build time:** Content is processed (markdown → HTML, KaTeX rendering, wiki-link resolution, backlink indexing, immersive scene compilation, card image rendering) during `cargo leptos build`. The output is a set of static HTML files + a WASM bundle + per-page assets (`immersive.json`, `card.png`).
- **Runtime:** The WASM bundle hydrates the static HTML and handles all interactivity client-side: navigation, tooltips, Lean4 compilation (via WASM or external API), visualizations, immersive playback.
- **No server functions on the hot path:** All data that pages need is either baked into the HTML at build time or fetched client-side. The only optional runtime endpoint is the Lean4 compile fallback (see below).

This eliminates SSR hydration mismatches and enables deployment to a Cloudflare Worker (see [Deployment](#deployment-cloudflare-workers)) or any compatible static host.

### Frontend

- **Rust → WASM** core for rendering, animation, and computation
- Leptos with SSG mode for static page generation
- Canvas or WebGL for high-fidelity visualizations
- egui-based code editor for Lean4 input (pure Rust, no JS)

### Build-Time Processing

- Content pipeline: markdown parsing, KaTeX math rendering, wiki-link resolution, backlink index computation — all happen at build time
- Output: static HTML files per route + WASM bundle + CSS/assets
- Deployed to any static file host

### Lean4 Compilation

- **Primary:** Client-side via Lean4 WASM port (lean4web) — no server needed
- **Fallback:** Optional external API endpoint for compilation if WASM port is not available
- LaTeX and Typst rendering pipeline (client-side or build-time)

### Build & Deployment

- `cargo-leptos` for SSG build producing static output
- Cloudflare Worker as the primary deployment target (see below)
- No Docker containers needed for production

### Deployment (Cloudflare Workers)

MoonMath ships as a single Cloudflare Worker that serves the static SSG output and, optionally, fronts a Lean4 compile fallback. This is the only supported production target — everything is shaped around it.

- **Asset serving:** `target/site/` (the cargo-leptos SSG output) is uploaded to Workers Static Assets. The Worker entrypoint is a thin Rust module compiled with `worker-rs` (or, equivalently, a TypeScript shim) that delegates `GET` requests to the static assets binding and applies HTTP caching headers (immutable for hashed assets, short TTL for HTML).
- **Routing:** Pretty URLs (`/showcase/:category/:slug`, `/showcase/:category/:slug/immersive`) are served by mapping each route to its pre-rendered `index.html`. 404 falls back to a static `404.html`.
- **Lean4 compile fallback:** A single Worker route `POST /api/lean/compile` proxies to a managed Lean4 service (Cloudflare Containers or an external host) when the client-side WASM compiler is unavailable. Bodies are size-capped (32 KB) and rate-limited per IP via Workers KV.
- **Configuration:** `wrangler.toml` at the repo root pins the Worker name, compatibility date, static assets binding, and environment-specific routes (`moonmath.dev`, `staging.moonmath.dev`). Secrets (Lean fallback URL, signing key) are set via `wrangler secret put`.
- **CI deploy:** GitHub Actions builds the SSG output, runs the [acceptance test suite](#acceptance-testing), and publishes via `wrangler deploy` on green. Preview deployments are created per PR via Cloudflare's `--name` per-branch convention.
- **Observability:** Worker tail logs go to Cloudflare's Logpush; basic metrics (req count, error rate, p95 latency) are scraped from the Cloudflare API into the project's existing dashboards.

### Acceptance Testing

Every build runs an acceptance suite that drives a real Chrome instance via the **Chrome DevTools Protocol (CDP)** and refuses to deploy if any check fails. The suite is the canonical answer to "did this change break the site?" — `cargo check` and unit tests cannot, by themselves, detect KaTeX layout breakage, eframe canvas regressions, or a Lean4 proof that no longer compiles in the browser.

- **Runner:** A Rust crate `crates/moonmath-acceptance` using [`chromiumoxide`](https://github.com/mattsse/chromiumoxide) (CDP client over async-tungstenite). No Node, no Playwright. The runner boots the SSG output via `wrangler dev` (or `python -m http.server` for fast local runs), launches headless Chrome, and walks a fixed list of routes.
- **Trigger:**
  - Local: `cargo acceptance` (alias defined in `.cargo/config.toml`) — runs on demand and is wired into the `cargo dev` pre-deploy step described in `CLAUDE.md`.
  - CI: a required GitHub Actions job between `cargo leptos build` and `wrangler deploy`. A red acceptance run blocks merge and blocks deploy.
- **Coverage (must all pass):**
  1. **Lean4 proofs compile.** For each showcase page with `lean4_status = "complete"` or `"partial"`, the runner navigates to the page, clicks "Compile", and asserts the compiler reports success within 30 s. Failures dump the editor contents and compiler diagnostics into the run artifacts. Pages with `sorry`/`planned` are asserted to *render* the snippet but skipped from the compile gate.
  2. **Formulas render properly.** For every `$...$` and `$$...$$` block in source, the runner asserts the corresponding `.katex` element exists, has non-zero bounding box, and contains no `.katex-error` span. A pixel-diff baseline is captured for the front page hero formula and each premier page's primary `latex` formula; >1% pixel delta fails the run.
  3. **Interactions and animations work.** Per-page checks include:
     - The "Open in Immersive Mode" link navigates to `/...:slug/immersive`, the eframe canvas mounts, and the first animation step advances within 2 s.
     - Spacebar pause/resume changes the playback state (verified via a debug attribute on the canvas root).
     - Showcase pages with `tags = ["interactive", ...]` boot their canvas (Mandelbrot, IFS) and respond to a synthetic mouse drag with a CDP `Input.dispatchMouseEvent` sequence.
     - `prefers-reduced-motion` is honored: with the media feature emulated on, no transitions exceed 50 ms.
- **Test data and fixtures:** The list of routes and per-page assertions is generated at build time from the same frontmatter that produces the site (`acceptance.json`), so adding a new showcase page automatically extends coverage. There is no separate test inventory to keep in sync.
- **Artifacts:** On failure, the runner writes screenshots, HAR, console logs, and Lean diagnostics into `target/acceptance/<run-id>/` and uploads them as a CI artifact.
- **Why CDP, not Playwright/WebDriver:** CDP is the lowest-friction Rust-native option, integrates cleanly with the existing Cargo workflow, and keeps the toolchain in one language. It also gives direct access to performance traces for the 60fps immersive-mode budget.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust |
| WASM tooling | wasm-bindgen, cargo-leptos |
| Frontend framework | Leptos 0.7 (SSG mode) |
| Visualization | egui (via eframe WASM), Canvas 2D / WebGL via web-sys |
| Math rendering | katex-rs (build-time) + KaTeX CSS (client) |
| Code editor | egui-based editor (pure Rust, no JS) |
| Lean4 | lean4web WASM port (client-side) or Cloudflare Worker fallback |
| LaTeX rendering | katex-rs (build-time) |
| Typst rendering | typst CLI (build-time) |
| Immersive mode | eframe + egui (WASM canvas, pure animation) |
| Card images | Headless Chrome (CDP) snapshot of build-time card component |
| Hosting | Cloudflare Worker (Workers Static Assets) |
| Acceptance tests | Rust + chromiumoxide (Chrome DevTools Protocol) |

## Navigation & Information Architecture

The app is organized around **categories** of mathematics, each containing multiple **showcase pages** for individual theorems, proofs, or findings.

### URL Structure

```
/                                          → Home (lists all categories)
/showcase                                  → Category index (grid/list of all categories)
/showcase/:category                        → Category page (lists all showcase pages in that category)
/showcase/:category/:slug                  → Individual showcase page (theorem/finding)
```

Examples:
- `/showcase/fractal-geometry`             → Fractal Geometry category page
- `/showcase/fractal-geometry/mandelbrot`  → Mandelbrot Set showcase
- `/showcase/galois-theory/quintic`        → Impossibility of Quintic Formula showcase
- `/showcase/number-theory/prime-theorem`  → Infinitude of Primes showcase

### Category Page

Each category page displays:
- Category title and description
- A card grid of all showcase pages within the category, each showing:
  - **A generated card image** (see [Card Images](#card-images) below) — never a text-only card
  - Theorem/finding title
  - Brief summary (1–2 sentences)
  - Tags (e.g. "interactive", "lean4-proof", "visualization")
  - A small **"Premier"** badge if the page is marked premier (assumes limited prerequisite knowledge of the topic). Pages without the badge make no claim about prerequisite difficulty.
- Breadcrumb navigation: Home → Showcase → Category

#### Card Images

Every showcase card displays a generated image — text-only cards are not permitted on `/showcase`, category pages, or the home page.

- **Source:** Each showcase page renders a dedicated `<ShowcaseCard>` component at build time (Leptos SSG route `/_card/:category/:slug`), styled to a fixed 1200×630 canvas with the page title, primary `latex` formula (KaTeX-rendered), category accent, and category icon.
- **Generation:** A build step (`cargo run -p moonmath-ssg -- --cards`) launches headless Chrome via CDP, navigates to each `/_card/...` route, and captures a screenshot to `target/site/cards/<category>/<slug>.png` (and `.webp`). The same toolchain that powers acceptance tests is reused — no extra image library.
- **Override:** A page may declare `card_image = "path/to/custom.png"` in frontmatter to bypass generation (useful for fractal/visualization showcases that prefer a real render of their attractor).
- **Cache:** Cards are content-hashed and only regenerated when the page's frontmatter, primary formula, or category metadata changes.
- **Fallback:** If generation fails for a page, the build fails — there is no text-only fallback in production. The acceptance suite asserts every visible card on `/showcase` and category pages has a non-empty rendered image.

### Individual Showcase Page

Each showcase page presents a single theorem or finding with:
- Title, author attribution (where applicable), and a "Premier" badge if applicable (no introductory/intermediate/advanced rating; see [Premier Tag](#premier-tag))
- An **"Open in Immersive Mode"** action in the page header (icon button, `i` shortcut) — links to `/showcase/:category/:slug/immersive`
- KaTeX-rendered mathematical statement
- Step-by-step proof walkthrough with animated explanations
- **Lean4 proof snippet** — every showcase page MUST include a Lean4 formalization of the theorem/finding, rendered with syntax highlighting and compilable client-side. This is a core differentiator: MoonMath provides machine-checked proofs alongside human-readable explanations.
- Interactive visualizations (where applicable)
- Proof dependency graph (prerequisites and related results)
- Breadcrumb navigation: Home → Showcase → Category → This Page
- Prev/Next navigation within the same category
- **Backlinks and concept tooltips** (see below)

#### Premier Tag

MoonMath does **not** rate pages by difficulty (introductory / intermediate / advanced). Difficulty levels were removed because they were both subjective and discouraging — readers would self-select away from "advanced" content even when the page was approachable, and authors would over- or under-rate their own work.

Instead, a single optional boolean tag, `premier`, marks pages that **assume limited prerequisite knowledge of the topic**. A premier page is the recommended starting point for that topic — it doesn't require the reader to have seen related theorems first.

- `premier = true` — Reader can land here cold and follow along. Earns the "Premier" badge on cards and the page header.
- (omitted / `premier = false`) — Page assumes the reader has worked through prerequisite material (typically declared via `prerequisites = [...]`).

A category may have any number of premier pages (including zero). The home page and category pages surface premier pages first in the card grid.

#### Lean4 Proof Requirement

Every showcase page must contain at least one Lean4 code block that formalizes the page's theorem or a key lemma. The Lean4 snippet must:
- Be self-contained or import only from Mathlib
- Include the theorem statement and a **complete proof — no `sorry`, no `admit`, no `axiom`-based shortcuts**. If the full theorem is out of reach, scope the snippet down to a key lemma that *can* be discharged in full rather than leaving a placeholder.
- Compile under the project's pinned `lean-toolchain` (Mathlib included), verified by `lake build` in `lean-project/`
- Be compilable via the client-side Lean4 WASM compiler at the same toolchain
- Include doc-comments explaining the proof strategy
- Be rendered with syntax highlighting using the Lean4 highlighter

**Acceptance gate.** CI runs `lake build` over every Lean snippet extracted from showcase pages and fails the build if any snippet contains the strings `sorry`, `admit`, or an unjustified `axiom`. The `lean4_status` frontmatter field consequently only takes the values `complete` (full theorem proven) or `partial` (a scoped lemma is proven in full); the legacy `sorry` and `planned` values are no longer permitted on merged content.

### Backlinks & Concept Tooltips (Wikipedia-style)

Mathematical content is deeply interconnected — a proof in Galois Theory may reference prime factorization from Number Theory, or a fractal dimension formula may depend on logarithmic identities. The app provides a rich cross-referencing system inspired by Wikipedia's internal links and preview tooltips.

#### Concept Links

Within any showcase page's markdown content, authors can link to other showcase pages or to specific concepts using wiki-style syntax:

```markdown
This follows from the [[Fundamental Theorem of Arithmetic]] and [[Fermat's Little Theorem]].
The key insight uses [[Galois group|the Galois group]] of the splitting field.
```

- `[[Page Title]]` — links to the showcase page with that title, auto-resolved across all categories
- `[[Page Title|display text]]` — links with custom display text
- `[[#concept-id]]` — links to a specific heading/concept anchor within the current page
- `[[Page Title#section]]` — deep link to a section in another page

Concept links are rendered as styled internal links (`<a class="concept-link">`) visually distinct from external links.

#### Hover Tooltips (Preview Popups)

When a user hovers over (or long-presses on mobile) a concept link, a tooltip popup appears showing:
- The linked page's title
- Its primary LaTeX formula (from frontmatter `latex` field)
- The first paragraph of the page content (or the `description` from frontmatter)
- A "Read more →" link to navigate to the full page

Tooltip data is loaded lazily — a lightweight server endpoint returns just the summary data (title, formula, description) without the full page content. Tooltips are cached client-side after the first load.

```
┌─────────────────────────────────────────┐
│ Fundamental Theorem of Arithmetic       │
│                                         │
│  n = p₁^a₁ · p₂^a₂ ⋯ pₖ^aₖ           │
│                                         │
│ Every integer > 1 has a unique prime    │
│ factorization.                          │
│                                         │
│ Read more →                             │
└─────────────────────────────────────────┘
```

#### Backlinks Section

At the bottom of each showcase page, a "Referenced by" section lists all other pages that link to this page. This allows users to discover related content and navigate the knowledge graph in reverse:

```
── Referenced by ──────────────────────────
• Fermat's Little Theorem (Number Theory)
• Impossibility of the Quintic Formula (Galois Theory)
```

Backlinks are computed at build/load time by scanning all showcase pages for `[[...]]` references.

#### Concept Index

A dedicated page at `/showcase/concepts` provides a full index of all cross-referenced concepts, showing:
- Each concept with the number of references across all pages
- Links to every page that mentions it
- An adjacency visualization (graph) of how concepts connect across categories

#### Implementation Notes

- **Content format:** `[[...]]` wiki-links in markdown are preprocessed (similar to math delimiters) before pulldown-cmark parsing, converted to `<a class="concept-link" data-target="slug" data-category="category">` elements
- **Tooltip data:** Baked into the static HTML at build time as embedded `<span class="concept-tooltip">` elements alongside each concept link. No runtime API call needed — tooltips are pure CSS `:hover` reveals of pre-rendered HTML.
- **Backlink index:** Built at build time by scanning all page sources for `[[...]]` patterns; the "Referenced by" section is pre-rendered into each page's static HTML.
- **Client-side:** Tooltip positioning is handled by CSS. On mobile, tooltips appear on long-press and dismiss on tap-away (small WASM helper).

### Home Page

The home page serves as the entry point with:
- Featured/highlighted showcase pages
- Category grid showing all categories with item counts
- Quick search across all showcase pages

## User Flows

### Browse by Category

1. User lands on the home page and sees all categories displayed as cards
2. User clicks a category (e.g. "Fractal Geometry")
3. Category page shows all showcase pages in that category as a card grid
4. User clicks a specific showcase page (e.g. "Mandelbrot Set")
5. Showcase page renders the full theorem/finding with proof, visualization, and Lean4 code
6. User navigates to the next showcase in the category via prev/next links, or back to the category via breadcrumbs

### Explore a Formula

1. User selects a formula or topic from a category or search
2. App renders the formula with animated explanation
3. User can pause, step through, adjust speed, and interact with terms
4. Related algorithms or proofs are linked for deeper exploration

### Visualize an Algorithm

1. User picks an algorithm from a category or pastes pseudocode
2. WASM engine runs the algorithm on sample or user-provided input
3. Visualization renders each step with annotations
4. User can scrub through the execution timeline

### Write & Compile Lean4

1. User writes Lean4 code in the built-in editor
2. User clicks "Compile" — code is sent to the backend Lean4 service
3. Compilation results (success or errors) are displayed inline
4. On success, user selects output format (LaTeX or Typst)
5. Rendered output is shown in-browser with an option to download

## Non-Functional Requirements

- **Performance**: Visualizations must run at 60fps for datasets up to 10k elements
- **Latency**: Lean4 compilation feedback within 10 seconds for typical proofs
- **Accessibility**: Keyboard navigation, screen reader support for text content
- **Browser support**: Latest Chrome, Firefox, Safari, Edge
- **Offline**: Core formula animations and algorithm visualizations work offline once loaded

## Milestones

| Phase | Scope |
|-------|-------|
| **v0.1** | Project scaffolding — Leptos shell, content pipeline, KaTeX math, cargo-leptos build. **Done.** |
| **v0.1.5** | Category navigation — showcase index, category pages, breadcrumbs, prev/next within category, frontmatter-driven content from `content/showcase/`. **Done.** |
| **v0.1.6** | Backlinks & concept tooltips — `[[wiki-links]]` in markdown, hover preview popups, "Referenced by" section, backlink index, concept graph page. **Done.** |
| **v0.1.7** | **SSG migration** — Replace SSR + hydration with static site generation. All pages pre-rendered at build time, no runtime server. Eliminates hydration mismatches. |
| **v0.1.8** | **Lean4 in every showcase** — Add Lean4 proof snippets to all 10 existing showcase pages. Add `lean4_status` frontmatter field. Lean4 syntax highlighting in rendered HTML. |
| **v0.1.9** | **Premier tag + card images** — Remove `difficulty` from frontmatter and UI. Introduce `premier` boolean. Add build-time card image generation (CDP screenshot of `/_card/...` route) and require non-text cards site-wide. |
| **v0.1.10** | **SEO surface** — Per-page metadata (title, description, canonical, Open Graph, Twitter card, JSON-LD `TechArticle`), `sitemap.xml` and `robots.txt` emitted by SSG, default OG card. See [`specs/seo.md`](./seo.md). |
| **v0.2** | egui canvas + Leptos↔egui bridge for interactive visualizations |
| **v0.2.5** | **Cloudflare Worker deployment** — `wrangler.toml`, Workers Static Assets binding, GitHub Actions deploy pipeline, preview deploys per PR, optional Lean compile fallback route. |
| **v0.2.6** | **Acceptance test suite (CDP)** — `crates/moonmath-acceptance` with chromiumoxide. Build-blocking checks for Lean4 compile, formula rendering, and animation/interaction smoke. Wired into CI. |
| **v0.3** | Algorithm visualization engine — 3-5 algorithms with interactive controls |
| **v0.3.5** | **Immersive Mode** — `eframe`-based per-page animation that narrates the page's markdown. `immersive.json` build artifact, `/showcase/:category/:slug/immersive` route, playback controls, reduced-motion support. |
| **v0.4** | Lean4 client-side compilation — lean4web WASM integration, egui-based editor, compile-and-check in browser |
| **v0.5** | LeanTeX→LaTeX→KaTeX + Typst PDF export, search, offline support, accessibility |

## Categories & Showcase Pages

The app organizes content into mathematical **categories**. Each category contains multiple **showcase pages**, each dedicated to a single theorem, proof, or finding. Showcase pages demonstrate end-to-end proof walkthroughs with Lean4 compilation, KaTeX-rendered math, proof dependency graphs, and step-by-step explanations.

Categories and their showcase pages are defined by the content in `content/showcase/` with the following directory structure:

```
content/showcase/
├── _index.md                              → Showcase index metadata
├── number-theory/
│   ├── _index.md                          → Category metadata (title, description, ordering)
│   ├── prime-theorem.md                   → Individual showcase page
│   ├── fundamental-theorem-arithmetic.md
│   ├── fermats-little-theorem.md
│   └── quadratic-reciprocity.md
├── galois-theory/
│   ├── _index.md
│   ├── quintic.md
│   ├── fundamental-theorem.md
│   └── constructible-numbers.md
├── fractal-geometry/
│   ├── _index.md
│   ├── mandelbrot.md
│   ├── hausdorff-dimension.md
│   └── iterated-function-systems.md
└── ...                                    → More categories added over time
```

### Category: Number Theory (`/showcase/number-theory`)

Slug | Title | Status | Lean4 | Tags
-----|-------|--------|-------|-----
`prime-theorem` | Infinitude of Primes | **Implemented** | planned | lean4-proof
`fundamental-theorem-arithmetic` | Fundamental Theorem of Arithmetic | Implemented | planned | lean4-proof
`fermats-little-theorem` | Fermat's Little Theorem | Implemented | planned | lean4-proof
`quadratic-reciprocity` | Quadratic Reciprocity | Implemented | planned | lean4-proof

- **Infinitude of Primes** — Euclid's classic proof via factorial + 1 argument. Lean4 proof with `IsPrime`, `exists_prime_factor`, `dvd_factorial`, and the main `InfinitudeOfPrimes` theorem.
- **Fundamental Theorem of Arithmetic** — Every integer > 1 has a unique prime factorization. Proof by strong induction (existence) and proof by contradiction (uniqueness).
- **Fermat's Little Theorem** — $a^p \equiv a \pmod{p}$ for prime $p$. Proof via combinatorial argument (necklace counting) or induction.
- **Quadratic Reciprocity** — Relationship between solvability of $x^2 \equiv p \pmod{q}$ and $x^2 \equiv q \pmod{p}$. Gauss's lemma approach.

### Category: Galois Theory (`/showcase/galois-theory`)

Slug | Title | Status | Lean4 | Tags
-----|-------|--------|-------|-----
`quintic` | Impossibility of Quintic Formula | Implemented | planned | lean4-proof, visualization
`fundamental-theorem` | Fundamental Theorem of Galois Theory | Implemented | planned | lean4-proof, visualization
`constructible-numbers` | Constructible Numbers | Implemented | planned | lean4-proof, visualization

- **Impossibility of Quintic Formula** — No general algebraic formula for degree ≥ 5 polynomials. Showcase the chain: field extensions → splitting fields → Galois group → solvable groups → $S_5$ is not solvable.
- **Fundamental Theorem of Galois Theory** — Bijection between intermediate fields and subgroups of the Galois group. Visualize the lattice correspondence for a concrete example (e.g. splitting field of $x^4 - 2$ over $\mathbb{Q}$).
- **Constructible Numbers** — Which regular $n$-gons are constructible with compass and straightedge? Connect to Gauss's criterion via Galois theory over $\mathbb{Q}$.

### Category: Fractal Geometry (`/showcase/fractal-geometry`)

Slug | Title | Status | Lean4 | Tags
-----|-------|--------|-------|-----
`mandelbrot` | Mandelbrot Set | Implemented | planned | lean4-proof, interactive, visualization
`hausdorff-dimension` | Hausdorff Dimension | Implemented | planned | lean4-proof, visualization
`iterated-function-systems` | Iterated Function Systems | Implemented | planned | lean4-proof, interactive, visualization

- **Mandelbrot Set** — Interactive visualization of $z_{n+1} = z_n^2 + c$ iteration. WASM-rendered canvas with zoom, orbit tracing, and escape-time coloring. Connect to the theorem that the Mandelbrot set is connected.
- **Hausdorff Dimension** — Compute and visualize the dimension of classical fractals (Sierpinski triangle = $\log 3 / \log 2$, Koch curve = $\log 4 / \log 3$). Step-by-step box-counting demonstration.
- **Iterated Function Systems** — Barnsley fern and Sierpinski triangle via IFS. Show the contraction mapping theorem guarantees a unique attractor. Interactive parameter sliders for affine transformations.

### Adding New Categories

New categories are added by creating a new directory under `content/showcase/` with an `_index.md` containing TOML frontmatter:

```markdown
+++
title = "Category Name"
description = "Brief description of this area of mathematics"
weight = 40       # Controls ordering on the category index
+++

Optional long-form introduction to this category.
```

Individual showcase pages use frontmatter to declare metadata:

```markdown
+++
title = "Theorem Name"
description = "One-line summary for the card grid"
weight = 10
premier = true                # optional; true = assumes limited prerequisite knowledge of the topic
tags = ["lean4-proof", "interactive", "visualization"]
latex = "\\forall n, \\exists p > n"   # primary formula (used in tooltips and card image)
prerequisites = ["prime-theorem"]   # slugs of other showcase pages
lean4_status = "complete"     # complete | partial | sorry | planned
card_image = "custom-card.png"  # optional; overrides the build-time generated card
+++

Full content: math, explanation, [[wiki-links]], and Lean4 code blocks.

Every page MUST include at least one Lean4 fenced code block:

    ```lean4
    theorem my_theorem : ... := by
      ...
    ```
```

## Open Questions

- **Lean4 WASM:** Is lean4web mature enough for reliable client-side compilation, or should we provide a fallback API endpoint?
- **SSG tooling:** Does Leptos 0.7 support SSG natively, or should we use a build script that pre-renders all routes and outputs static HTML?
- Should we support collaborative editing or sharing of proofs?
- How should we handle showcase pages that span multiple categories (e.g. Constructible Numbers touches both Galois Theory and Geometry)?
- Should concept tooltips also work for inline math formulas (hover a formula to see its definition)?
- Should the concept graph visualization be interactive (zoom, pan, click-to-navigate)?
- **Lean4 completeness:** For theorems that are extremely difficult to formalize (e.g. Quadratic Reciprocity), is `sorry` acceptable as a placeholder with a comment explaining what's needed?
- **Immersive script authoring:** Is the auto-derived `immersive.json` from markdown enough, or do some pages need an optional `immersive.toml` override (custom timing, camera moves, hand-tuned highlights)?
- **Card image cost:** Headless Chrome at build time adds seconds per page. Is there a Rust-native renderer (resvg + a KaTeX-to-SVG path) that we should use to keep `cargo dev` fast?
- **Acceptance test flake budget:** What's the policy when a CDP test flakes — auto-retry up to N times, or fail fast and require the author to investigate? Pixel-diff thresholds in particular are flaky territory.
- **Lean fallback service:** The Cloudflare Worker fallback proxies to a Lean compile service — do we self-host on Cloudflare Containers, or use a managed third-party endpoint, and how do we keep it from becoming an abuse vector?
