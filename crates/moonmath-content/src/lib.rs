pub mod lean_highlight;
pub mod loader;
pub mod markdown;
pub mod render;
pub mod showcase;
pub mod taxonomy;
pub mod wikilinks;

use moonmath_types::{ContentPage, Section, Taxonomy};

/// All processed content from the content/ directory.
#[derive(Debug, Clone)]
pub struct SiteContent {
    pub pages: Vec<ContentPage>,
    pub sections: Vec<Section>,
    pub taxonomy: Taxonomy,
}

/// Load and process all content from a directory.
pub fn load_content(content_dir: &std::path::Path) -> Result<SiteContent, loader::LoadError> {
    let (pages, sections) = loader::load_all(content_dir)?;
    let taxonomy = taxonomy::build_taxonomy(&pages);
    Ok(SiteContent {
        pages,
        sections,
        taxonomy,
    })
}
