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

## Architecture

### Static Site Generation (SSG)

MoonMath is a **statically generated site**. All pages are pre-rendered at build time with no runtime server:

- **Build time:** Content is processed (markdown → HTML, KaTeX rendering, wiki-link resolution, backlink indexing) during `cargo leptos build`. The output is a set of static HTML files + a WASM bundle.
- **Runtime:** The WASM bundle hydrates the static HTML and handles all interactivity client-side: navigation, tooltips, Lean4 compilation (via WASM or external API), visualizations.
- **No server functions:** All data that pages need is either baked into the HTML at build time or fetched client-side. No Axum server is required in production — just a static file host.

This eliminates SSR hydration mismatches and enables deployment to any static host (GitHub Pages, Cloudflare Pages, Vercel, S3 + CloudFront).

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
- Static asset hosting (GitHub Pages, Cloudflare Pages, Vercel, or S3 + CloudFront)
- No Docker containers needed for production

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust |
| WASM tooling | wasm-bindgen, cargo-leptos |
| Frontend framework | Leptos 0.7 (SSG mode) |
| Visualization | egui (via eframe WASM), Canvas 2D / WebGL via web-sys |
| Math rendering | katex-rs (build-time) + KaTeX CSS (client) |
| Code editor | egui-based editor (pure Rust, no JS) |
| Lean4 | lean4web WASM port (client-side) or external API fallback |
| LaTeX rendering | katex-rs (build-time) |
| Typst rendering | typst CLI (build-time) |
| Hosting | Any static file host (GitHub Pages, Cloudflare Pages, Vercel) |

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
  - Theorem/finding title
  - Brief summary (1–2 sentences)
  - Tags (e.g. "interactive", "lean4-proof", "visualization")
  - Difficulty indicator (introductory / intermediate / advanced)
- Breadcrumb navigation: Home → Showcase → Category

### Individual Showcase Page

Each showcase page presents a single theorem or finding with:
- Title, author attribution (where applicable), and difficulty level
- KaTeX-rendered mathematical statement
- Step-by-step proof walkthrough with animated explanations
- **Lean4 proof snippet** — every showcase page MUST include a Lean4 formalization of the theorem/finding, rendered with syntax highlighting and compilable client-side. This is a core differentiator: MoonMath provides machine-checked proofs alongside human-readable explanations.
- Interactive visualizations (where applicable)
- Proof dependency graph (prerequisites and related results)
- Breadcrumb navigation: Home → Showcase → Category → This Page
- Prev/Next navigation within the same category
- **Backlinks and concept tooltips** (see below)

#### Lean4 Proof Requirement

Every showcase page must contain at least one Lean4 code block that formalizes the page's theorem or a key lemma. The Lean4 snippet should:
- Be self-contained or import only from Mathlib
- Include the theorem statement and proof (or proof sketch with `sorry` for work-in-progress)
- Be compilable via the client-side Lean4 WASM compiler
- Include doc-comments explaining the proof strategy
- Be rendered with syntax highlighting using the Lean4 highlighter

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
| **v0.1.7** | **SSG migration** — Replace SSR + hydration with static site generation. All pages pre-rendered at build time, no runtime server. Eliminates hydration mismatches. Deploy to static host. |
| **v0.1.8** | **Lean4 in every showcase** — Add Lean4 proof snippets to all 10 existing showcase pages. Add `lean4_status` frontmatter field. Lean4 syntax highlighting in rendered HTML. |
| **v0.2** | egui canvas + Leptos↔egui bridge for interactive visualizations |
| **v0.3** | Algorithm visualization engine — 3-5 algorithms with interactive controls |
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
difficulty = "intermediate"   # introductory | intermediate | advanced
tags = ["lean4-proof", "interactive", "visualization"]
latex = "\\forall n, \\exists p > n"   # primary formula (used in tooltips)
prerequisites = ["prime-theorem"]   # slugs of other showcase pages
lean4_status = "complete"   # complete | partial | sorry | planned
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
