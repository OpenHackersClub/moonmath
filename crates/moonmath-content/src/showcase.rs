use std::path::Path;

use gray_matter::engine::TOML;
use gray_matter::Matter;
use moonmath_types::{ContentPage, Frontmatter, ShowcaseCategory, ShowcasePageSummary};

use crate::markdown;

/// Load all showcase categories from `content/showcase/`.
pub fn load_categories(content_dir: &Path) -> Vec<ShowcaseCategory> {
    let showcase_dir = content_dir.join("showcase");
    if !showcase_dir.is_dir() {
        return Vec::new();
    }

    let mut categories = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&showcase_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let dir = entry.path();
        let index_file = dir.join("_index.md");
        if !index_file.exists() {
            continue;
        }

        let slug = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let source = match std::fs::read_to_string(&index_file) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let fm = match parse_frontmatter(&source) {
            Some(f) => f,
            None => continue,
        };

        let page_count = std::fs::read_dir(&dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let p = e.path();
                p.extension().is_some_and(|ext| ext == "md")
                    && p.file_name().is_some_and(|n| n != "_index.md")
            })
            .count();

        categories.push(ShowcaseCategory {
            slug,
            title: fm.title,
            description: fm.description.unwrap_or_default(),
            weight: fm.weight.unwrap_or(0),
            page_count,
        });
    }

    categories.sort_by_key(|c| c.weight);
    categories
}

/// Load all showcase page summaries for a given category.
pub fn load_category_pages(content_dir: &Path, category: &str) -> Vec<ShowcasePageSummary> {
    let category_dir = content_dir.join("showcase").join(category);
    if !category_dir.is_dir() {
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&category_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.extension().is_some_and(|ext| ext == "md")
                && p.file_name().is_some_and(|n| n != "_index.md")
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let slug = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let fm = match parse_frontmatter(&source) {
            Some(f) => f,
            None => continue,
        };

        pages.push(ShowcasePageSummary {
            slug,
            category: category.to_string(),
            title: fm.title,
            description: fm.description.unwrap_or_default(),
            difficulty: fm.difficulty,
            tags: fm.tags,
            weight: fm.weight.unwrap_or(0),
        });
    }

    pages.sort_by_key(|p| p.weight);
    pages
}

/// Load a single showcase page's full content.
pub fn load_showcase_page(
    content_dir: &Path,
    category: &str,
    slug: &str,
) -> Option<ContentPage> {
    let file_path = content_dir
        .join("showcase")
        .join(category)
        .join(format!("{}.md", slug));

    let source = std::fs::read_to_string(&file_path).ok()?;
    let fm = parse_frontmatter(&source)?;

    let mut matter = Matter::<TOML>::new();
    matter.delimiter = "+++".to_string();
    let result = matter.parse(&source);
    let html = markdown::render_markdown(&result.content);

    Some(ContentPage {
        slug: format!("showcase/{}/{}", category, slug),
        frontmatter: fm,
        html,
        source,
    })
}

/// Load the category title from its _index.md.
pub fn load_category_title(content_dir: &Path, category: &str) -> Option<String> {
    let index_file = content_dir
        .join("showcase")
        .join(category)
        .join("_index.md");
    let source = std::fs::read_to_string(&index_file).ok()?;
    let fm = parse_frontmatter(&source)?;
    Some(fm.title)
}

pub fn parse_frontmatter(source: &str) -> Option<Frontmatter> {
    let mut matter = Matter::<TOML>::new();
    matter.delimiter = "+++".to_string();
    let result = matter.parse(source);
    result.data?.deserialize().ok()
}
