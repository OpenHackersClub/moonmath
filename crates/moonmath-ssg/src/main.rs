use std::fs;
use std::path::Path;

use moonmath_content::{lean_highlight, render, showcase, wikilinks};
use moonmath_math::katex_render;
use moonmath_types::*;

fn main() {
    let watch = std::env::args().any(|a| a == "--watch");

    let content_dir = Path::new("content");
    let out_dir = Path::new("target/ssg-data");

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
    generate_showcase_pages(content_dir, out_dir, &page_index, &backlink_index);

    // Generate index.html shell
    generate_index_html();

    eprintln!("SSG: done.");

    if watch {
        let status = std::process::Command::new("cargo")
            .args(["leptos", "watch"])
            .status()
            .expect("failed to run cargo leptos watch");
        std::process::exit(status.code().unwrap_or(1));
    }
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

fn generate_showcase_pages(
    content_dir: &Path,
    out_dir: &Path,
    page_index: &std::collections::HashMap<String, wikilinks::PageInfo>,
    backlink_index: &std::collections::HashMap<String, Vec<BacklinkEntry>>,
) {
    let categories = showcase::load_categories(content_dir);

    for cat in &categories {
        let siblings = showcase::load_category_pages(content_dir, &cat.slug);
        let cat_dir = out_dir.join("showcase").join(&cat.slug);
        fs::create_dir_all(&cat_dir).expect("create category dir");

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
            };

            write_json(
                &cat_dir.join(format!("{}.json", page_summary.slug)),
                &detail,
            );
            eprintln!("SSG: wrote {}/{}.json", cat.slug, page_summary.slug);
        }
    }
}

fn generate_index_html() {
    let site_dir = Path::new("target/site");
    fs::create_dir_all(site_dir).expect("create site dir");

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <link rel="stylesheet" href="/pkg/moonmath-app.css"/>
    <link rel="stylesheet"
        href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"
        integrity="sha384-nB0miv6/jRmo5RLHM0EW/XZBG00j7eSsxGb4cT/Zmv3h2SQTmSx0bfnoTvkJFh"
        crossorigin="anonymous"/>
    <title>MoonMath — Interactive Math Visualization</title>
</head>
<body>
    <script type="module">
        import init, { hydrate } from '/pkg/moonmath-app.js';
        async function main() {
            await init('/pkg/moonmath-app_bg.wasm');
            hydrate();
        }
        main();
    </script>
</body>
</html>"#;

    fs::write(site_dir.join("index.html"), html).expect("write index.html");
    eprintln!("SSG: wrote index.html");
}

fn write_json<T: serde::Serialize>(path: &Path, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("serialize JSON");
    fs::write(path, json).expect("write JSON file");
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
    use super::{contains_sorry_keyword, strip_markdown_lean4_section};

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
}
