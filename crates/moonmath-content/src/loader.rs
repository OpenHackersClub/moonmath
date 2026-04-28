use std::path::{Path, PathBuf};

use gray_matter::engine::TOML;
use gray_matter::Matter;
use moonmath_types::{ContentPage, Frontmatter, Section};

use crate::markdown;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse frontmatter in {path}: {message}")]
    Frontmatter { path: String, message: String },
}

/// Load all markdown files from `content_dir`, returning pages and sections.
pub fn load_all(content_dir: &Path) -> Result<(Vec<ContentPage>, Vec<Section>), LoadError> {
    let mut pages = Vec::new();
    let mut sections = Vec::new();

    walk_dir(content_dir, content_dir, &mut pages, &mut sections)?;

    pages.sort_by(|a, b| a.slug.cmp(&b.slug));
    sections.sort_by(|a, b| a.slug.cmp(&b.slug));

    Ok((pages, sections))
}

fn walk_dir(
    base: &Path,
    dir: &Path,
    pages: &mut Vec<ContentPage>,
    sections: &mut Vec<Section>,
) -> Result<(), LoadError> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();

    for entry in entries {
        if entry.is_dir() {
            walk_dir(base, &entry, pages, sections)?;
        } else if entry.extension().is_some_and(|ext| ext == "md") {
            let source = std::fs::read_to_string(&entry)?;
            let slug = path_to_slug(base, &entry);

            let file_name = entry
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if file_name == "_index.md" {
                let section = parse_section(&source, &slug, &entry)?;
                sections.push(section);
            } else {
                let page = parse_page(&source, &slug, &entry)?;
                pages.push(page);
            }
        }
    }

    Ok(())
}

fn path_to_slug(base: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(base).unwrap_or(file);
    let slug = rel
        .with_extension("")
        .to_string_lossy()
        .replace('\\', "/");
    // Remove trailing "/_index"
    slug.strip_suffix("/_index")
        .unwrap_or(&slug)
        .to_string()
}

fn parse_page(source: &str, slug: &str, path: &Path) -> Result<ContentPage, LoadError> {
    let mut matter = Matter::<TOML>::new();
    matter.delimiter = "+++".to_string();
    let result = matter.parse(source);

    let frontmatter: Frontmatter = result
        .data
        .ok_or_else(|| LoadError::Frontmatter {
            path: path.display().to_string(),
            message: "missing frontmatter".to_string(),
        })?
        .deserialize()
        .map_err(|e| LoadError::Frontmatter {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

    let html = markdown::render_markdown(&result.content);

    Ok(ContentPage {
        slug: slug.to_string(),
        frontmatter,
        html,
        source: source.to_string(),
    })
}

fn parse_section(source: &str, slug: &str, path: &Path) -> Result<Section, LoadError> {
    let mut matter = Matter::<TOML>::new();
    matter.delimiter = "+++".to_string();
    let result = matter.parse(source);

    let frontmatter: Frontmatter = result
        .data
        .ok_or_else(|| LoadError::Frontmatter {
            path: path.display().to_string(),
            message: "missing frontmatter".to_string(),
        })?
        .deserialize()
        .map_err(|e| LoadError::Frontmatter {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

    Ok(Section {
        slug: slug.to_string(),
        title: frontmatter.title,
        description: Some(result.content.trim().to_string()),
        children: Vec::new(), // populated later by taxonomy
    })
}
