# MoonMath — SEO Specification

Implements the search-engine and social-card surface for MoonMath. Slots into the milestone list in `specs/prd.md` between v0.1.9 (premier tag + card images) and v0.2 (egui). Lands as a single PR alongside the implementation.

## Goals

1. Every public page exposes machine-readable metadata sufficient for search-engine indexing and social-card preview generation.
2. A single canonical URL per page, no duplicate content.
3. Structured data (JSON-LD) describes each showcase page as a `TechArticle` so Google can attribute it to MoonMath / Open Hackers Club.
4. A complete `sitemap.xml` and `robots.txt` are served from the document root.
5. Lighthouse SEO score ≥ 95 on a sample of 5 pages (home, showcase index, category page, showcase detail, concepts index).

Non-goals for v1:
- Internationalization (`hreflang`, multi-locale variants).
- Dynamic OG cards per showcase page (we ship a single static OG default; per-category cards are a follow-up).
- AMP / structured data beyond `TechArticle`.
- A managed sitemap submission to Google Search Console (manual step).

## 1. Per-page metadata schema

All pages output the following inside `<head>`. Concrete values are derived from the same TOML frontmatter the content pipeline already parses.

| Tag | Source | Notes |
| --- | --- | --- |
| `<title>` | `frontmatter.title` + " — MoonMath" | Already present on most pages; standardize the suffix. |
| `<meta name="description">` | `frontmatter.description` | Falls back to a category- or site-level default if missing. |
| `<meta name="viewport">` | static | `width=device-width, initial-scale=1`. |
| `<link rel="canonical">` | `{base_url}{route}` | `base_url` = `https://moonmath.app` (TODO: confirm custom domain — until then the worker's `*.workers.dev` subdomain works because the host is rewritten by the `MOONMATH_BASE_URL` env override read by SSG). |
| `<meta property="og:title">` | same as `<title>` minus suffix | |
| `<meta property="og:description">` | same as `meta description` | |
| `<meta property="og:type">` | `article` for showcase detail; `website` for home / index pages | |
| `<meta property="og:url">` | same as canonical | |
| `<meta property="og:site_name">` | `MoonMath` | |
| `<meta property="og:image">` | `{base_url}/og-default.png` | Per-page cards arrive in a follow-up PR. |
| `<meta name="twitter:card">` | `summary_large_image` | |
| `<meta name="twitter:title">` | same as `og:title` | |
| `<meta name="twitter:description">` | same as `og:description` | |
| `<meta name="twitter:image">` | same as `og:image` | |
| `<script type="application/ld+json">` | computed | One per page (showcase detail only for v1). |

### JSON-LD payload (showcase detail)

```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "headline": "{title}",
  "description": "{description}",
  "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "{canonical}"
  },
  "author": {
    "@type": "Organization",
    "name": "Open Hackers Club",
    "url": "https://github.com/OpenHackersClub"
  },
  "publisher": {
    "@type": "Organization",
    "name": "MoonMath",
    "url": "{base_url}"
  },
  "image": "{base_url}/og-default.png",
  "datePublished": "{frontmatter.date | today}",
  "keywords": "{frontmatter.tags joined with ', '}",
  "isAccessibleForFree": true
}
```

Index-style pages (home, showcase index, category, concepts) do **not** emit `TechArticle` — they're navigational. They use the `WebSite` / `CollectionPage` types only if cheap to render; otherwise omit JSON-LD entirely.

## 2. Sitemap (`/sitemap.xml`)

Generated at SSG time by `crates/moonmath-ssg`. Written to `target/site/sitemap.xml` so the existing `scripts/prerender.sh` copies it into `dist/`.

Contents:

- `<url>` for `/`, `/showcase`, `/inspirations`, `/showcase/concepts` (priority 0.5, changefreq `weekly`).
- `<url>` for every category in `target/ssg-data/showcase/categories.json` (`/showcase/<slug>`, priority 0.6, `weekly`).
- `<url>` for every showcase detail page (priority 0.8, `monthly`).

Each entry includes `<lastmod>` from frontmatter `date` (when present) or build time (`%Y-%m-%d`) otherwise.

The base URL is read from `MOONMATH_BASE_URL` (env), defaulting to `https://moonmath.app`. CI sets the value when the custom domain ships.

## 3. robots.txt

Served from `/robots.txt`. Body:

```
User-agent: *
Allow: /
Disallow: /api/
Disallow: /data/

Sitemap: {base_url}/sitemap.xml
```

The `/api/` and `/data/` disallows keep search engines out of the Lean compile API and the SSG JSON blobs, while still indexing the rendered HTML pages that consume them.

## 4. OG default image

Ship a single `1200x630` PNG/SVG at `crates/moonmath-app/public/og-default.svg` that:
- Uses the dark-theme palette (matches `--color-surface`).
- Shows the MoonMath wordmark and `e^{iπ}+1=0` glyph as the visual hook.
- Is referenced by `og:image` and `twitter:image`.

Future work (out of scope for this PR): build-time generation of per-page OG cards via the same headless-Chrome path the `<ShowcaseCard>` component uses. Tracked as a TODO at the bottom of this spec.

## 5. Performance / Core Web Vitals

The SSR + WASM-hydrate setup already delivers a fast TTFB. Additional touches:

- `<link rel="preload" as="style" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">` to start the KaTeX stylesheet earlier.
- Keep `viewport` meta on every page.
- `font-display: swap` on `@font-face` rules so text renders before fonts download.
- Avoid layout shift by rendering math placeholders that match the eventual KaTeX output height (already mostly done by `render::render_math_in_html`).

Acceptance: Lighthouse SEO score ≥ 95 on:
- `/`
- `/showcase`
- `/showcase/number-theory`
- `/showcase/number-theory/prime-theorem`
- `/showcase/concepts`

## 6. Semantic HTML

- One `<h1>` per page (the page title).
- `<h2>`/`<h3>` for sections; markdown content already emits these.
- `<nav>` for breadcrumbs (already wired in `Breadcrumbs`).
- `<main>` wraps the page body (`.main-content` in `App`).
- `<article>` around each showcase note's prose body.
- `<footer>` if/when we add one — currently the site has no global footer, which is fine for v1.

## 7. Acceptance tests (curl-driven smoke checks)

Run after `cargo run -p moonmath-ssg && cargo leptos build --release && ./scripts/prerender.sh`.

1. **Per-page meta** — every showcase URL has the expected tags:
   ```sh
   curl -s {base_url}/showcase/number-theory/prime-theorem | grep -E '<meta name="description"|<meta property="og:title"|application/ld\+json' | head -5
   ```
   Expect non-empty matches.
2. **Sitemap well-formed** — `/sitemap.xml` is XML with at least 13 `<url>` entries (matches the v1 page count; will scale automatically as content grows):
   ```sh
   curl -s {base_url}/sitemap.xml | grep -c '<url>'
   ```
3. **Robots advertises sitemap**:
   ```sh
   curl -s {base_url}/robots.txt | grep -q "Sitemap:"
   ```
4. **Canonical is unique** — every showcase page emits exactly one `<link rel="canonical">`.

A future follow-up will fold these into the CDP acceptance suite (`crates/moonmath-acceptance`, milestone v0.2.6) so a regression in metadata blocks merges, but for v1 a manual run before deploy is sufficient.

## 8. Out-of-scope (recorded as TODO)

- **Per-category OG cards** — generated at build time by reusing the `<ShowcaseCard>` headless-Chrome path from `specs/prd.md → Card Images`. Awaits the v0.1.9 card pipeline.
- **Custom domain** — sitemap and canonical URLs reference `https://moonmath.app` (configurable via `MOONMATH_BASE_URL`). Until that DNS lands, set the env to the active `*.workers.dev` URL in CI.
- **i18n** — single-locale (`en`) for the foreseeable future.
- **`MathSolver` JSON-LD type** — `TechArticle` covers the v1 case; revisit when individual theorems gain solver UIs.
- **CDP acceptance gating** — see milestone v0.2.6.
