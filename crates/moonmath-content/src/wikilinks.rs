use std::collections::HashMap;
use std::path::Path;

use moonmath_types::BacklinkEntry;

use crate::showcase;

/// Page info for wiki-link resolution and tooltip rendering.
pub struct PageInfo {
    pub category: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub latex: Option<String>,
}

/// Preprocess `[[wiki-links]]` in markdown source to HTML placeholders.
///
/// - `[[Page Title]]` → `<a class="concept-link" data-concept="Page Title">Page Title</a>`
/// - `[[Page Title|display]]` → `<a class="concept-link" data-concept="Page Title">display</a>`
pub fn preprocess_wikilinks(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '[' {
            i += 2;
            let start = i;
            while i + 1 < chars.len() && !(chars[i] == ']' && chars[i + 1] == ']') {
                i += 1;
            }
            let inner: String = chars[start..i].iter().collect();

            let (target, display) = if let Some(pipe_pos) = inner.find('|') {
                (inner[..pipe_pos].to_string(), inner[pipe_pos + 1..].to_string())
            } else {
                (inner.clone(), inner)
            };

            result.push_str(&format!(
                "<a class=\"concept-link\" data-concept=\"{}\">{}</a>",
                html_escape(&target),
                html_escape(&display)
            ));

            if i + 1 < chars.len() {
                i += 2; // skip ]]
            }
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Extract wiki-link target titles from raw markdown source.
pub fn extract_wikilink_targets(source: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '[' {
            i += 2;
            let start = i;
            while i + 1 < chars.len() && !(chars[i] == ']' && chars[i + 1] == ']') {
                i += 1;
            }
            let inner: String = chars[start..i].iter().collect();
            let target = if let Some(pipe_pos) = inner.find('|') {
                inner[..pipe_pos].to_string()
            } else {
                inner
            };
            targets.push(target);
            if i + 1 < chars.len() {
                i += 2;
            }
            continue;
        }
        i += 1;
    }

    targets
}

/// Build a page index mapping lowercase title → PageInfo for all showcase pages.
pub fn build_page_index(content_dir: &Path) -> HashMap<String, PageInfo> {
    let showcase_dir = content_dir.join("showcase");
    if !showcase_dir.is_dir() {
        return HashMap::new();
    }

    let mut index = HashMap::new();

    for entry in std::fs::read_dir(&showcase_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
    {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }

        let category = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        for page_entry in std::fs::read_dir(&dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
        {
            let path = page_entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && path.file_name().is_some_and(|n| n != "_index.md")
            {
                let slug = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let source = match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                if let Some(fm) = showcase::parse_frontmatter(&source) {
                    let key = fm.title.to_lowercase();
                    index.insert(
                        key,
                        PageInfo {
                            category: category.clone(),
                            slug,
                            title: fm.title,
                            description: fm.description.unwrap_or_default(),
                            latex: fm.latex,
                        },
                    );
                }
            }
        }
    }

    index
}

/// Build a backlink index: target_title (lowercase) → list of pages referencing it.
pub fn build_backlink_index(content_dir: &Path) -> HashMap<String, Vec<BacklinkEntry>> {
    let showcase_dir = content_dir.join("showcase");
    if !showcase_dir.is_dir() {
        return HashMap::new();
    }

    let page_index = build_page_index(content_dir);
    let mut backlinks: HashMap<String, Vec<BacklinkEntry>> = HashMap::new();

    for entry in std::fs::read_dir(&showcase_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
    {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }

        let category = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let category_title = showcase::load_category_title(content_dir, &category)
            .unwrap_or_else(|| category.clone());

        for page_entry in std::fs::read_dir(&dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
        {
            let path = page_entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && path.file_name().is_some_and(|n| n != "_index.md")
            {
                let slug = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let source = match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let page_title = showcase::parse_frontmatter(&source)
                    .map(|f| f.title)
                    .unwrap_or_default();

                for target in extract_wikilink_targets(&source) {
                    let target_key = target.to_lowercase();
                    // Skip self-references
                    if target_key == page_title.to_lowercase() {
                        continue;
                    }
                    // Only add if target exists in the page index
                    if page_index.contains_key(&target_key) {
                        backlinks
                            .entry(target_key)
                            .or_default()
                            .push(BacklinkEntry {
                                category: category.clone(),
                                category_title: category_title.clone(),
                                slug: slug.clone(),
                                title: page_title.clone(),
                            });
                    }
                }
            }
        }
    }

    backlinks
}

/// Resolve wiki-link placeholders in rendered HTML to full links with tooltips.
///
/// `render_formula` is a callback that renders LaTeX to HTML (e.g. KaTeX).
pub fn resolve_wikilinks(
    html: &str,
    page_index: &HashMap<String, PageInfo>,
    render_formula: &dyn Fn(&str) -> String,
) -> String {
    let prefix = "<a class=\"concept-link\" data-concept=\"";
    let middle = "\">";
    let suffix = "</a>";

    let mut result = String::with_capacity(html.len() * 2);
    let mut remaining = html;

    while let Some(start_idx) = remaining.find(prefix) {
        result.push_str(&remaining[..start_idx]);
        remaining = &remaining[start_idx + prefix.len()..];

        if let Some(mid_idx) = remaining.find(middle) {
            let concept_escaped = &remaining[..mid_idx];
            let concept = html_unescape(concept_escaped);
            remaining = &remaining[mid_idx + middle.len()..];

            if let Some(end_idx) = remaining.find(suffix) {
                let display = &remaining[..end_idx];
                remaining = &remaining[end_idx + suffix.len()..];

                let concept_key = concept.to_lowercase();
                if let Some(info) = page_index.get(&concept_key) {
                    let href = format!("/showcase/{}/{}", info.category, info.slug);
                    let tooltip = build_tooltip_html(info, render_formula);
                    result.push_str(&format!(
                        "<span class=\"concept-link-wrapper\">\
                         <a class=\"concept-link\" href=\"{}\">{}</a>\
                         {}</span>",
                        href, display, tooltip
                    ));
                } else {
                    // Unresolved link — render with visual indicator
                    result.push_str(&format!(
                        "<span class=\"concept-link-unresolved\" \
                         title=\"Page not found: {}\">{}</span>",
                        html_escape(&concept),
                        display
                    ));
                }
            } else {
                // Malformed — pass through
                result.push_str(prefix);
                result.push_str(concept_escaped);
                result.push_str(middle);
            }
        } else {
            result.push_str(prefix);
        }
    }

    result.push_str(remaining);
    result
}

fn build_tooltip_html(
    info: &PageInfo,
    render_formula: &dyn Fn(&str) -> String,
) -> String {
    let mut html = String::from("<span class=\"concept-tooltip\">");
    html.push_str(&format!(
        "<span class=\"concept-tooltip-title\">{}</span>",
        html_escape(&info.title)
    ));

    if let Some(ref latex) = info.latex {
        let rendered = render_formula(latex);
        html.push_str(&format!(
            "<span class=\"concept-tooltip-formula\">{}</span>",
            rendered
        ));
    }

    if !info.description.is_empty() {
        html.push_str(&format!(
            "<span class=\"concept-tooltip-desc\">{}</span>",
            html_escape(&info.description)
        ));
    }

    let href = format!("/showcase/{}/{}", info.category, info.slug);
    html.push_str(&format!(
        "<a href=\"{}\" class=\"concept-tooltip-more\">Read more →</a>",
        href
    ));
    html.push_str("</span>");
    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_simple_wikilink() {
        let result = preprocess_wikilinks("See [[Fundamental Theorem]].");
        assert!(result.contains("data-concept=\"Fundamental Theorem\""));
        assert!(result.contains(">Fundamental Theorem</a>"));
    }

    #[test]
    fn test_preprocess_wikilink_with_display() {
        let result = preprocess_wikilinks("See [[Page Title|display text]].");
        assert!(result.contains("data-concept=\"Page Title\""));
        assert!(result.contains(">display text</a>"));
    }

    #[test]
    fn test_extract_targets() {
        let source = "See [[Foo]] and [[Bar|baz]] here.";
        let targets = extract_wikilink_targets(source);
        assert_eq!(targets, vec!["Foo", "Bar"]);
    }

    #[test]
    fn test_resolve_with_index() {
        let html = r#"<p><a class="concept-link" data-concept="Test Page">Test Page</a></p>"#;
        let mut index = HashMap::new();
        index.insert(
            "test page".to_string(),
            PageInfo {
                category: "cat".into(),
                slug: "test-page".into(),
                title: "Test Page".into(),
                description: "A test".into(),
                latex: None,
            },
        );
        let result = resolve_wikilinks(html, &index, &|_| String::new());
        assert!(result.contains("href=\"/showcase/cat/test-page\""));
        assert!(result.contains("concept-tooltip"));
    }

    #[test]
    fn test_resolve_unresolved() {
        let html = r#"<a class="concept-link" data-concept="Missing">Missing</a>"#;
        let result = resolve_wikilinks(html, &HashMap::new(), &|_| String::new());
        assert!(result.contains("concept-link-unresolved"));
    }
}
