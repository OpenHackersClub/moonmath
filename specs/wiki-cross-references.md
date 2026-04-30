# Wiki Cross-References — Implementation Spec

Companion to `specs/prd.md` §"Backlinks & Concept Tooltips (Wikipedia-style)".
This document narrows the PRD's prose into a concrete, testable set of behaviours
and tracks the gap between what is already implemented and what is still
required for v0.1.6 to be genuinely "done" rather than just code-complete.

## 1. Goals

A reader on any showcase page should be able to:

1. **See** that a phrase is a cross-reference (visual affordance).
2. **Hover** the phrase and read a preview (title + primary formula + summary)
   without leaving the page.
3. **Click** the phrase and navigate to the referenced page.
4. **Follow** the inverse relationship via a "Referenced by" section.
5. **Browse** the full list of cross-referenced concepts on a single index page.

The experience should feel Wikipedia-like — predictable, fast, no surprise
network calls — and degrade gracefully on touch devices and with JavaScript
disabled.

## 2. Non-goals

- Full-text search across all pages (deferred — separate spec).
- Auto-linking of bare prose terms (e.g. detecting "prime" anywhere and linking
  it). The matching cost and false-positive rate make this a poor default; we
  rely on explicit `[[...]]` syntax instead.
- Rich-text tooltips with embedded canvases or animations. The tooltip is HTML
  + KaTeX only.
- Cross-references between formulas/algorithms content trees and showcase
  pages. Scope is currently **showcase ↔ showcase** only — the page index in
  `wikilinks::build_page_index` walks `content/showcase` exclusively. Lifting
  this is tracked as a follow-up.

## 3. Authoring syntax

```markdown
The proof builds on the [[Fundamental Theorem of Arithmetic]].
The key step uses [[Galois group|the Galois group]] of the splitting field.
```

| Form                              | Resolved as                                        |
|-----------------------------------|----------------------------------------------------|
| `[[Page Title]]`                  | Link to page whose frontmatter `title` matches (case-insensitive). Display = title. |
| `[[Page Title|display text]]`     | Same target, custom display text.                  |
| `[[Missing Page]]`                | Rendered as a visually-distinct unresolved span.   |

Heading anchors (`[[Page#section]]`, `[[#local-anchor]]`) are described in the
PRD but **not in scope for v0.1.6** — they require a slugged-heading pass and
are deferred.

## 4. Build-time pipeline (already implemented)

1. `moonmath-content::markdown::render_markdown` runs
   `wikilinks::preprocess_wikilinks` **before** pulldown-cmark, replacing
   `[[…]]` with placeholder `<a class="concept-link" data-concept="…">…</a>`.
   This survives markdown parsing because pulldown-cmark passes raw HTML
   through.
2. `moonmath-ssg` (in `crates/moonmath-ssg/src/main.rs`) builds two indexes:
   - `wikilinks::build_page_index` — `lowercased_title → PageInfo { category,
     slug, title, description, latex }` for every page under
     `content/showcase/`.
   - `wikilinks::build_backlink_index` — `lowercased_target_title →
     [BacklinkEntry]`, only listing references whose target resolves.
3. For each showcase page, SSG rewrites the rendered HTML through
   `wikilinks::resolve_wikilinks`, swapping each placeholder for either:
   - **Resolved:** `<span class="concept-link-wrapper"><a class="concept-link"
     href="/showcase/cat/slug">display</a><span class="concept-tooltip">…title,
     KaTeX-rendered formula, description, "Read more →"…</span></span>`
   - **Unresolved:** `<span class="concept-link-unresolved" title="Page not
     found: X">display</span>`
4. The resolved tooltip's formula is pre-rendered to KaTeX HTML at build time —
   no client-side KaTeX run needed.
5. The page's `ShowcaseDetailResponse.backlinks` is populated from the backlink
   index and rendered as a "Referenced by" section in
   `pages/showcase_detail.rs`.
6. `concepts.json` is written for the index page (concept entry + reference
   count + backlinks list).

This pipeline is fully wired and exercised by tests in
`moonmath-content/src/wikilinks.rs`.

## 5. Runtime behaviour (the gap this spec closes)

### 5.1 Visual affordance & tooltips — **missing CSS**

The HTML is correct; what's missing is styling. CSS in
`crates/moonmath-app/style/main.css` must add:

- `.concept-link` — styled distinctly from external `<a>`. Use the accent
  colour with a subtle dotted underline to read as a cross-reference, not a
  generic link.
- `.concept-link-unresolved` — same family, but greyed/strikethrough-adjacent
  with a `cursor: help`, signalling the link is dangling.
- `.concept-link-wrapper` — `position: relative`; the positioning context for
  the tooltip.
- `.concept-tooltip` — absolutely positioned popup, hidden by default. Reveals
  on `:hover` and `:focus-within` of the wrapper (keyboard-friendly). Must:
  - have a sane max-width (~22rem) and word-wrap;
  - render the embedded `.katex` formula at a slightly reduced scale so it
    fits;
  - include arrow/triangle pointer (CSS-only);
  - clip to viewport on small screens (use `max-width: min(22rem, 90vw)`);
  - never break vertical rhythm of prose (the wrapper is `display: inline`).
- `.concept-tooltip-title`, `.concept-tooltip-formula`, `.concept-tooltip-desc`,
  `.concept-tooltip-more` — typographic hierarchy inside the popup.
- Mobile: `:hover` is unreliable. Add `@media (hover: none)` rules so taps on
  `.concept-link` follow the link normally; users can long-press to see
  iOS/Android's native preview, which is acceptable for v0.1.6. A dedicated
  WASM tap-to-toggle helper is **out of scope** here.
- Reduced motion: respect `prefers-reduced-motion: reduce` — drop the
  fade/translate transition, snap visibility.

### 5.2 Concepts index page — **missing route + page**

`concepts.json` is generated but no Leptos page consumes it. Add:

- `crates/moonmath-app/src/pages/concepts.rs` — `ConceptsIndexPage` component
  that loads `/data/showcase/concepts.json` via `json_resource_once`.
- Register the module in `pages/mod.rs`.
- Add the route in `crates/moonmath-app/src/app.rs` **before** the
  `(showcase, category)` dynamic segment so `/showcase/concepts` matches the
  static route, not the category one.
- Add a "Concepts" link in the top nav next to Showcase.

Page contents (v0.1.6):

- Title, breadcrumb (`Home / Showcase / Concepts`).
- Sorted list (most-referenced first, alphabetical tiebreak — the SSG already
  emits this order).
- Each row: title (linked), category badge, reference count, expandable list of
  pages that reference it.
- Empty state: "No cross-references yet."

The "concept graph visualization" mentioned in the PRD is **explicitly
deferred** — it's an open question in the PRD itself and warrants its own
spec.

### 5.3 Acceptance criteria

- [ ] Hovering any `.concept-link` on a showcase page reveals a tooltip with
      title, formula (KaTeX), description, and a "Read more →" link.
- [ ] Tabbing to a `.concept-link` with a keyboard reveals the same tooltip
      via `:focus-within`.
- [ ] Clicking a `.concept-link` navigates to the resolved page.
- [ ] An unresolved `[[Foo]]` renders with `.concept-link-unresolved` and a
      `title=` hint, no tooltip popup.
- [ ] `/showcase/concepts` lists every showcase page that has at least one
      backlink, sorted by reference count desc.
- [ ] The "Concepts" nav link is present and active state highlights when on
      `/showcase/concepts`.
- [ ] `cargo check --features ssr -p moonmath-app` and
      `cargo check --features hydrate -p moonmath-app --target
      wasm32-unknown-unknown` both pass.
- [ ] `cargo test` — all `wikilinks` tests still green.
- [ ] `cargo run -p moonmath-ssg` writes `target/ssg-data/showcase/concepts.json`
      with > 0 entries.
- [ ] Manual smoke: `/showcase/fractal-geometry/iterated-function-systems`
      shows tooltips on its three wikilinks.

## 6. Follow-ups (post v0.1.6)

1. **Cross-tree references.** Lift `build_page_index` to walk `content/`
   broadly (formulas/, algorithms/) and resolve cross-tree wikilinks. Requires
   a unifying URL convention (`/<tree>/<category>/<slug>`).
2. **Heading anchors.** `[[Page#section]]` after slugged-heading pass.
3. **Concept graph.** Force-directed adjacency view at
   `/showcase/concepts/graph`. Likely an egui canvas, dovetailing with the v0.2
   visualization milestone.
4. **Auto-detect candidate links.** A build-time linter that surfaces phrases
   matching known page titles but not wrapped in `[[…]]`, as a soft warning to
   authors. Not auto-rewriting — just prompting.
5. **Tap-to-pin tooltip on mobile.** Small WASM helper to toggle a `.is-open`
   class on tap; second tap follows the link.
