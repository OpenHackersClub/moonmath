use serde::{Deserialize, Serialize};

/// Frontmatter metadata parsed from TOML in content markdown files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub difficulty: Option<String>,
    /// Links to an egui visualization, e.g. "formula_viz::chain_rule"
    #[serde(default)]
    pub interactive: Option<String>,
    /// Primary LaTeX formula for this page
    #[serde(default)]
    pub latex: Option<String>,
    #[serde(default)]
    pub draft: bool,
    /// Ordering weight (lower = first)
    #[serde(default)]
    pub weight: Option<i32>,
    /// Short description for card summaries
    #[serde(default)]
    pub description: Option<String>,
    /// Slugs of prerequisite showcase pages
    #[serde(default)]
    pub prerequisites: Vec<String>,
    /// Status of the Lean4 proof: "complete", "sorry", or absent
    #[serde(default)]
    pub lean4_status: Option<String>,
}

/// A processed content page ready for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPage {
    /// URL slug derived from file path, e.g. "formulas/calculus/chain-rule"
    pub slug: String,
    pub frontmatter: Frontmatter,
    /// Rendered HTML from markdown
    pub html: String,
    /// Raw markdown source
    pub source: String,
}

/// A section index (from _index.md files).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub children: Vec<String>,
}

/// Taxonomy: maps tags/categories to page slugs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Taxonomy {
    pub tags: std::collections::HashMap<String, Vec<String>>,
    pub categories: std::collections::HashMap<String, Vec<String>>,
}

/// Lean4 compilation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileRequest {
    pub code: String,
}

/// Lean4 compilation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResponse {
    pub success: bool,
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub errors: Vec<CompileError>,
    #[serde(default)]
    pub latex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
}

/// A showcase category (e.g. "Number Theory", "Fractal Geometry").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowcaseCategory {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub weight: i32,
    pub page_count: usize,
}

/// Summary of a showcase page, used in category page card grids.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowcasePageSummary {
    pub slug: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub difficulty: Option<String>,
    pub tags: Vec<String>,
    pub weight: i32,
}

/// A backlink entry — a page that references another page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklinkEntry {
    pub category: String,
    pub category_title: String,
    pub slug: String,
    pub title: String,
}

/// Full response for a showcase detail page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowcaseDetailResponse {
    pub category_slug: String,
    pub category_title: String,
    pub title: String,
    /// Pre-rendered KaTeX HTML for the primary formula (avoids hydration mismatch)
    pub latex_html: Option<String>,
    pub html: String,
    pub difficulty: Option<String>,
    pub tags: Vec<String>,
    pub prev: Option<(String, String)>,
    pub next: Option<(String, String)>,
    pub backlinks: Vec<BacklinkEntry>,
    /// Pre-highlighted Lean4 code blocks found in the page content
    #[serde(default)]
    pub lean4_blocks: Vec<String>,
}

/// An entry in the concept index page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptEntry {
    pub category: String,
    pub category_title: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub reference_count: usize,
    pub referenced_by: Vec<BacklinkEntry>,
}
