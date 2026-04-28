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

            // Extract and highlight Lean4 blocks
            let lean4_sources: Vec<String> =
                lean_highlight::extract_lean4_blocks(&page.source);
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
                difficulty: page.frontmatter.difficulty,
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
