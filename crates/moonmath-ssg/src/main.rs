use std::fs;
use std::path::Path;

use moonmath_content::{lean_highlight, render, showcase, wikilinks};
use moonmath_math::katex_render;
use moonmath_types::*;

/// Default base URL used in canonical links + sitemap. Overridable via the
/// `MOONMATH_BASE_URL` env var (CI sets it to the active deploy host while
/// the custom domain is being wired up).
const DEFAULT_BASE_URL: &str = "https://moonmath.openhackers.club";

fn base_url() -> String {
    std::env::var("MOONMATH_BASE_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
        .trim_end_matches('/')
        .to_string()
}

/// Build-day stamp used as a fallback when frontmatter omits `date`. We use
/// `SystemTime` rather than the `chrono` crate to keep the SSG dep graph
/// minimal — sitemap.xml and JSON-LD only need ISO date precision.
fn today_iso_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    iso_date_from_unix(secs)
}

fn iso_date_from_unix(secs: i64) -> String {
    // Days since 1970-01-01.
    let days = secs.div_euclid(86_400);
    // Convert to civil date via Hinnant's algorithm.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let watch = args.iter().any(|a| a == "--watch");
    // `build` is the v0.1.7 SSG-only build path skeleton — it runs the
    // existing data pipeline, then additionally emits a minimal `dist/`
    // tree (placeholder shell + sitemap.xml + robots.txt). The plan is to
    // grow this subcommand into the full route walker that replaces
    // `scripts/prerender.sh`; today it just proves the wiring.
    let build = args.iter().any(|a| a == "build" || a == "--build");

    let content_dir = Path::new("content");
    let out_dir = Path::new("target/ssg-data");
    let site_dir = Path::new("target/site");

    // Clean and recreate output directory
    if out_dir.exists() {
        fs::remove_dir_all(out_dir).expect("failed to clean data dir");
    }
    fs::create_dir_all(out_dir).expect("failed to create data dir");

    // Build shared indexes for wikilinks/backlinks
    let page_index = wikilinks::build_page_index(content_dir);
    let backlink_index = wikilinks::build_backlink_index(content_dir);

    // Generate showcase data
    generate_showcase_categories(content_dir, out_dir);
    generate_showcase_concepts(content_dir, out_dir, &page_index, &backlink_index);
    let entries = generate_showcase_pages(content_dir, out_dir, &page_index, &backlink_index);

    // SEO surface — sitemap.xml + robots.txt land alongside index.html so
    // `scripts/prerender.sh` copies them into ./dist/ and the Cloudflare
    // Worker's Static Assets binding serves them at the root.
    let base = base_url();
    fs::create_dir_all(site_dir).expect("create site dir");
    generate_sitemap(site_dir, &base, &entries);
    generate_robots_txt(site_dir, &base);

    eprintln!("SSG: done.");

    if build {
        let dist_dir = Path::new("dist");
        emit_dist_skeleton(dist_dir, site_dir, &base);
    }

    if watch {
        let status = std::process::Command::new("cargo")
            .args(["leptos", "watch"])
            .status()
            .expect("failed to run cargo leptos watch");
        std::process::exit(status.code().unwrap_or(1));
    }
}

/// Emit the SSG-only `dist/` skeleton — a placeholder `index.html` plus the
/// sitemap and robots from `site_dir`. This is the v0.1.7 entry point that
/// will eventually replace `scripts/prerender.sh` once the route walker is
/// ported here. For now it is intentionally a stub so the build path can
/// land in CI alongside the existing SSR + curl prerender flow.
///
/// The placeholder body satisfies the same SEO sanity checks as the SSR
/// prerender (`<h1>` + `<nav>` present) so any downstream smoke test that
/// asserts non-empty HTML keeps passing once this is wired into CI.
fn emit_dist_skeleton(dist_dir: &Path, site_dir: &Path, base: &str) {
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir).expect("failed to clean dist dir");
    }
    fs::create_dir_all(dist_dir).expect("failed to create dist dir");

    let html = render_placeholder_shell(base);
    fs::write(dist_dir.join("index.html"), &html).expect("write dist/index.html");
    eprintln!("SSG-build: wrote dist/index.html ({} bytes)", html.len());

    for name in ["sitemap.xml", "robots.txt"] {
        let from = site_dir.join(name);
        if from.exists() {
            fs::copy(&from, dist_dir.join(name)).expect("copy SEO asset to dist");
            eprintln!("SSG-build: copied {} → dist/", name);
        } else {
            eprintln!("SSG-build: skip {} (missing in site_dir)", name);
        }
    }

    eprintln!("SSG-build: done. dist/ is a placeholder — route walker pending.");
}

/// Build the placeholder HTML shell written to `dist/index.html`. Kept as a
/// pure function so it's covered by a unit test without touching the
/// filesystem.
fn render_placeholder_shell(base: &str) -> String {
    let canonical = if base.ends_with('/') {
        format!("{}", base)
    } else {
        format!("{}/", base)
    };
    format!(
        concat!(
            "<!DOCTYPE html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "  <meta charset=\"utf-8\"/>\n",
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"/>\n",
            "  <title>MoonMath — SSG build placeholder</title>\n",
            "  <link rel=\"canonical\" href=\"{canonical}\"/>\n",
            "  <meta name=\"robots\" content=\"noindex\"/>\n",
            "</head>\n",
            "<body>\n",
            "  <nav class=\"site-nav\"><a href=\"/\">MoonMath</a></nav>\n",
            "  <main>\n",
            "    <h1>MoonMath</h1>\n",
            "    <p>SSG-only build path placeholder (PRD v0.1.7). ",
            "Full route rendering pending — production deploys still use ",
            "<code>scripts/prerender.sh</code>.</p>\n",
            "  </main>\n",
            "</body>\n",
            "</html>\n",
        ),
        canonical = canonical,
    )
}

fn generate_showcase_categories(content_dir: &Path, out_dir: &Path) {
    let categories = showcase::load_categories(content_dir);
    let dir = out_dir.join("showcase");
    fs::create_dir_all(&dir).expect("create showcase dir");
    write_json(&dir.join("categories.json"), &categories);
    eprintln!("SSG: wrote categories.json ({} categories)", categories.len());

    // Also write per-category page lists
    for cat in &categories {
        let cat_dir = dir.join(&cat.slug);
        fs::create_dir_all(&cat_dir).expect("create category dir");
        let pages = showcase::load_category_pages(content_dir, &cat.slug);
        write_json(&cat_dir.join("pages.json"), &pages);
        eprintln!(
            "SSG: wrote {}/pages.json ({} pages)",
            cat.slug,
            pages.len()
        );
    }
}

fn generate_showcase_concepts(
    content_dir: &Path,
    out_dir: &Path,
    page_index: &std::collections::HashMap<String, wikilinks::PageInfo>,
    backlink_index: &std::collections::HashMap<String, Vec<BacklinkEntry>>,
) {
    let mut concepts: Vec<ConceptEntry> = page_index
        .iter()
        .map(|(key, info)| {
            let referenced_by = backlink_index.get(key).cloned().unwrap_or_default();
            let reference_count = referenced_by.len();
            let category_title =
                showcase::load_category_title(content_dir, &info.category)
                    .unwrap_or_else(|| info.category.clone());
            ConceptEntry {
                category: info.category.clone(),
                category_title,
                slug: info.slug.clone(),
                title: info.title.clone(),
                description: info.description.clone(),
                reference_count,
                referenced_by,
            }
        })
        .collect();

    concepts.sort_by(|a, b| {
        b.reference_count
            .cmp(&a.reference_count)
            .then_with(|| a.title.cmp(&b.title))
    });

    let dir = out_dir.join("showcase");
    fs::create_dir_all(&dir).expect("create showcase dir");
    write_json(&dir.join("concepts.json"), &concepts);
    eprintln!("SSG: wrote concepts.json ({} concepts)", concepts.len());
}

/// Minimal record used to drive sitemap.xml emission.
#[derive(Debug, Clone)]
struct PageEntry {
    /// URL path (e.g. `/showcase/number-theory/prime-theorem`).
    path: String,
    /// ISO date for `<lastmod>`.
    last_mod: String,
}

fn generate_showcase_pages(
    content_dir: &Path,
    out_dir: &Path,
    page_index: &std::collections::HashMap<String, wikilinks::PageInfo>,
    backlink_index: &std::collections::HashMap<String, Vec<BacklinkEntry>>,
) -> Vec<PageEntry> {
    let categories = showcase::load_categories(content_dir);
    let today = today_iso_date();
    let mut entries: Vec<PageEntry> = Vec::new();

    for cat in &categories {
        let siblings = showcase::load_category_pages(content_dir, &cat.slug);
        let cat_dir = out_dir.join("showcase").join(&cat.slug);
        fs::create_dir_all(&cat_dir).expect("create category dir");

        // Category index page
        entries.push(PageEntry {
            path: format!("/showcase/{}", cat.slug),
            last_mod: today.clone(),
        });

        for (idx, page_summary) in siblings.iter().enumerate() {
            let page = match showcase::load_showcase_page(
                content_dir,
                &cat.slug,
                &page_summary.slug,
            ) {
                Some(p) => p,
                None => continue,
            };

            // Render math placeholders to KaTeX HTML
            let html_with_math = render::render_math_in_html(&page.html);

            // Resolve wikilinks with tooltips
            let html = wikilinks::resolve_wikilinks(
                &html_with_math,
                page_index,
                &|latex| {
                    katex_render::render_latex(latex, true).unwrap_or_default()
                },
            );

            // The interactive `<section class="lean4-section">` rendered by
            // ShowcaseDetailPage already shows a syntax-highlighted code
            // block plus a Compile button — drop the duplicate markdown-
            // rendered "## Lean4 Proof" heading + `<pre>...lean4...</pre>`.
            let html = strip_markdown_lean4_section(&html);

            // Prev/next
            let prev = if idx > 0 {
                let p = &siblings[idx - 1];
                Some((p.slug.clone(), p.title.clone()))
            } else {
                None
            };
            let next = if idx + 1 < siblings.len() {
                let p = &siblings[idx + 1];
                Some((p.slug.clone(), p.title.clone()))
            } else {
                None
            };

            // Backlinks
            let title_key = page.frontmatter.title.to_lowercase();
            let backlinks = backlink_index.get(&title_key).cloned().unwrap_or_default();

            // Pre-render primary formula
            let latex_html = page.frontmatter.latex.as_ref().map(|latex| {
                katex_render::render_latex(latex, true)
                    .unwrap_or_else(|e| format!("<span class=\"katex-error\">{}</span>", e))
            });

            // Extract and highlight Lean4 blocks. Drop any block containing
            // `sorry` — we don't showcase incomplete proofs. This keeps the
            // compile button honest: every visible Lean4 block must be a
            // proof Lean accepts without holes.
            let lean4_sources: Vec<String> = lean_highlight::extract_lean4_blocks(&page.source)
                .into_iter()
                .filter(|code| !contains_sorry_keyword(code))
                .collect();
            let lean4_blocks: Vec<String> = lean4_sources
                .iter()
                .map(|code| lean_highlight::highlight_lean(code))
                .collect();

            // SEO fields. `description` falls back to a derived default
            // when frontmatter omits it; `date_published` mirrors the
            // sitemap's `<lastmod>`.
            let description = page
                .frontmatter
                .description
                .clone()
                .unwrap_or_else(|| {
                    format!(
                        "Interactive walkthrough of {} on MoonMath.",
                        page.frontmatter.title
                    )
                });
            let date_published = page
                .frontmatter
                .date
                .clone()
                .unwrap_or_else(|| today.clone());

            let path = format!("/showcase/{}/{}", cat.slug, page_summary.slug);
            entries.push(PageEntry {
                path: path.clone(),
                last_mod: date_published.clone(),
            });

            let detail = ShowcaseDetailResponse {
                category_slug: cat.slug.clone(),
                category_title: cat.title.clone(),
                title: page.frontmatter.title,
                latex_html,
                html,
                premier: page.frontmatter.premier,
                tags: page.frontmatter.tags,
                prev,
                next,
                backlinks,
                lean4_blocks,
                lean4_sources,
                description,
                date_published,
            };

            write_json(
                &cat_dir.join(format!("{}.json", page_summary.slug)),
                &detail,
            );
            eprintln!("SSG: wrote {}/{}.json", cat.slug, page_summary.slug);
        }
    }

    entries
}

fn write_json<T: serde::Serialize>(path: &Path, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("serialize JSON");
    fs::write(path, json).expect("write JSON file");
}

/// Emit `target/site/sitemap.xml` covering every public route.
///
/// Static pages (`/`, `/showcase`, `/inspirations`, `/showcase/concepts`)
/// get fixed priorities + `weekly` change frequency. Showcase categories
/// and detail pages are appended from `entries`. Lighthouse SEO and
/// search engines both read this file from the document root, so it lives
/// under `target/site/` rather than `target/ssg-data/` (the latter is
/// served under `/data/`).
fn generate_sitemap(site_dir: &Path, base: &str, entries: &[PageEntry]) {
    let today = today_iso_date();

    let mut xml = String::with_capacity(4096);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    let static_routes: &[(&str, &str, &str)] = &[
        ("/", "1.0", "weekly"),
        ("/showcase", "0.7", "weekly"),
        ("/showcase/concepts", "0.6", "weekly"),
        ("/inspirations", "0.5", "weekly"),
    ];
    for (path, priority, freq) in static_routes {
        push_sitemap_url(&mut xml, base, path, &today, priority, freq);
    }

    for entry in entries {
        // Categories live at depth 2, detail pages at depth 3.
        let depth = entry.path.matches('/').count();
        let (priority, freq) = if depth <= 2 {
            ("0.6", "weekly")
        } else {
            ("0.8", "monthly")
        };
        push_sitemap_url(&mut xml, base, &entry.path, &entry.last_mod, priority, freq);
    }

    xml.push_str("</urlset>\n");
    fs::write(site_dir.join("sitemap.xml"), &xml).expect("write sitemap.xml");
    eprintln!(
        "SSG: wrote sitemap.xml ({} entries)",
        static_routes.len() + entries.len()
    );
}

fn push_sitemap_url(
    buf: &mut String,
    base: &str,
    path: &str,
    last_mod: &str,
    priority: &str,
    freq: &str,
) {
    // URLs must include trailing slashes to match the Cloudflare Worker's
    // html_handling = "auto-trailing-slash" and the canonical URLs emitted
    // by the SEO component. Without this, crawlers see a mismatch between
    // the sitemap URL and the canonical/redirect target.
    let loc = format!("{}{}", base, path);
    let loc = if loc.ends_with('/') { loc } else { format!("{}/", loc) };
    buf.push_str("  <url>\n");
    buf.push_str("    <loc>");
    buf.push_str(&xml_escape(&loc));
    buf.push_str("</loc>\n");
    buf.push_str("    <lastmod>");
    buf.push_str(&xml_escape(last_mod));
    buf.push_str("</lastmod>\n");
    buf.push_str("    <changefreq>");
    buf.push_str(freq);
    buf.push_str("</changefreq>\n");
    buf.push_str("    <priority>");
    buf.push_str(priority);
    buf.push_str("</priority>\n");
    buf.push_str("  </url>\n");
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            c => out.push(c),
        }
    }
    out
}

fn generate_robots_txt(site_dir: &Path, base: &str) {
    let body = format!(
        "User-agent: *\nAllow: /\nDisallow: /api/\nDisallow: /data/\n\nSitemap: {}/sitemap.xml\n",
        base
    );
    fs::write(site_dir.join("robots.txt"), body).expect("write robots.txt");
    eprintln!("SSG: wrote robots.txt");
}

/// Whether the Lean code uses `sorry` as a token (not as a substring of a
/// longer identifier like `sorry_proof`).
fn contains_sorry_keyword(code: &str) -> bool {
    code.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|tok| tok == "sorry")
}

/// Strip the markdown-rendered Lean4 section from a page's HTML so the
/// interactive component is the only place readers see the proof.
///
/// Removes any `<h2>Lean4 Proof</h2>` heading and every following
/// `<pre><code class="language-lean4">...</code></pre>` block, including
/// any whitespace between them.
fn strip_markdown_lean4_section(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(idx) = rest.find("<pre><code class=\"language-lean4\">") {
        // Look back through any whitespace + an optional `<h2>Lean4 Proof</h2>`.
        let before = &rest[..idx];
        let trimmed_end = before.trim_end_matches(|c: char| c.is_whitespace());
        let drop_heading_len = if let Some(stripped) =
            trimmed_end.strip_suffix("<h2>Lean4 Proof</h2>")
        {
            before.len() - stripped.len()
        } else {
            0
        };
        out.push_str(&before[..before.len() - drop_heading_len]);

        // Skip past the closing </code></pre>.
        let after_open = &rest[idx..];
        if let Some(close_rel) = after_open.find("</code></pre>") {
            rest = &after_open[close_rel + "</code></pre>".len()..];
        } else {
            // Malformed — keep the rest as-is to avoid silent loss.
            out.push_str(after_open);
            return out;
        }
    }

    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::{
        base_url, contains_sorry_keyword, emit_dist_skeleton, iso_date_from_unix,
        render_placeholder_shell, strip_markdown_lean4_section, xml_escape,
    };
    use std::fs;

    #[test]
    fn detects_bare_sorry() {
        assert!(contains_sorry_keyword("theorem t : True := sorry"));
        assert!(contains_sorry_keyword("  sorry -- TODO"));
        assert!(contains_sorry_keyword("· sorry\n· rfl"));
    }

    #[test]
    fn ignores_substrings() {
        assert!(!contains_sorry_keyword("def sorry_proof := True"));
        assert!(!contains_sorry_keyword("-- I'm sorry_about_this"));
        assert!(!contains_sorry_keyword("theorem t : True := trivial"));
    }

    #[test]
    fn strips_lean4_heading_and_block() {
        let html = "<p>intro</p>\n<h2>Lean4 Proof</h2>\n<pre><code class=\"language-lean4\">theorem t := rfl</code></pre>\n<p>after</p>";
        let stripped = strip_markdown_lean4_section(html);
        assert!(!stripped.contains("Lean4 Proof"));
        assert!(!stripped.contains("language-lean4"));
        assert!(stripped.contains("<p>intro</p>"));
        assert!(stripped.contains("<p>after</p>"));
    }

    #[test]
    fn strips_multiple_blocks() {
        let html = "<h2>Lean4 Proof</h2>\n<pre><code class=\"language-lean4\">a</code></pre>\n<pre><code class=\"language-lean4\">b</code></pre>\n<p>end</p>";
        let stripped = strip_markdown_lean4_section(html);
        assert!(!stripped.contains("language-lean4"));
        assert!(stripped.contains("<p>end</p>"));
    }

    #[test]
    fn leaves_html_without_lean4_alone() {
        let html = "<h2>Statement</h2><p>just prose</p>";
        assert_eq!(strip_markdown_lean4_section(html), html);
    }

    #[test]
    fn iso_date_matches_known_epochs() {
        // 1970-01-01 00:00 UTC → unix 0
        assert_eq!(iso_date_from_unix(0), "1970-01-01");
        // 2026-05-01 → 1761955200
        assert_eq!(iso_date_from_unix(1_777_852_800), "2026-05-04");
        // 2024-02-29 (leap day)
        assert_eq!(iso_date_from_unix(1_709_164_800), "2024-02-29");
    }

    #[test]
    fn xml_escape_handles_specials() {
        assert_eq!(xml_escape("a & b"), "a &amp; b");
        assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(xml_escape("\"q\""), "&quot;q&quot;");
    }

    #[test]
    fn base_url_uses_default_when_unset() {
        // Don't fight env state — just check that the default is well-formed.
        let url = base_url();
        assert!(url.starts_with("http"));
        assert!(!url.ends_with('/'));
    }

    #[test]
    fn placeholder_shell_has_seo_required_tags() {
        // The SSR prerender's sanity check (`scripts/prerender.sh`) refuses
        // to deploy if `dist/index.html` lacks `<h1>` or `<nav>`. The SSG
        // build path must clear the same bar so it can drop into CI behind
        // a flag without breaking the existing guardrails.
        let html = render_placeholder_shell("https://moonmath.openhackers.club");
        assert!(html.contains("<h1>"), "missing <h1> in placeholder shell");
        assert!(html.contains("<nav"), "missing <nav> in placeholder shell");
        assert!(html.contains("rel=\"canonical\""), "missing canonical link");
        assert!(html.contains("https://moonmath.openhackers.club/"));
    }

    #[test]
    fn placeholder_shell_normalizes_trailing_slash() {
        let with = render_placeholder_shell("https://example.com/");
        let without = render_placeholder_shell("https://example.com");
        // Both should canonicalize to the same trailing-slash URL so the
        // canonical link matches Cloudflare's auto-trailing-slash handling.
        assert!(with.contains("https://example.com/"));
        assert!(without.contains("https://example.com/"));
    }

    #[test]
    fn emit_dist_skeleton_writes_index_and_copies_seo_assets() {
        // Use a unique temp dir so parallel test runs don't trample each
        // other. Avoid pulling in `tempfile` — std is enough for a smoke.
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("moonmath-ssg-smoke-{}", stamp));
        let dist = root.join("dist");
        let site = root.join("site");
        fs::create_dir_all(&site).expect("create site");
        fs::write(site.join("sitemap.xml"), "<urlset/>").expect("seed sitemap");
        fs::write(site.join("robots.txt"), "User-agent: *\n").expect("seed robots");

        emit_dist_skeleton(&dist, &site, "https://moonmath.openhackers.club");

        let index = fs::read_to_string(dist.join("index.html"))
            .expect("dist/index.html should exist after build");
        assert!(index.contains("<h1>"));
        assert!(dist.join("sitemap.xml").exists());
        assert!(dist.join("robots.txt").exists());

        // Cleanup — best effort; the temp dir is harmless if it lingers.
        let _ = fs::remove_dir_all(&root);
    }
}
