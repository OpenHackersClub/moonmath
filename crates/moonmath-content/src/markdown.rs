use pulldown_cmark::{Event, Options, Parser, html};

/// Render markdown to HTML, preserving LaTeX math blocks.
///
/// Inline math (`$...$`) and display math (`$$...$$`) are passed through
/// as-is wrapped in appropriate HTML elements for client-side KaTeX rendering.
pub fn render_markdown(source: &str) -> String {
    let with_wikilinks = crate::wikilinks::preprocess_wikilinks(source);
    let preprocessed = preprocess_math(&with_wikilinks);

    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(&preprocessed, options);

    // Process events — pass through math placeholders intact
    let events: Vec<Event<'_>> = parser.collect();

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    // Restore math from placeholders
    postprocess_math(&mut html_output);

    html_output
}

/// Replace LaTeX math delimiters with placeholders that won't be parsed by pulldown-cmark.
///
/// - `$$...$$` → `<div class="math-display">...</div>`
/// - `$...$` → `<span class="math-inline">...</span>`
fn preprocess_math(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Display math: $$...$$
        if i + 1 < chars.len() && chars[i] == '$' && chars[i + 1] == '$' {
            i += 2;
            let start = i;
            while i + 1 < chars.len() && !(chars[i] == '$' && chars[i + 1] == '$') {
                i += 1;
            }
            let math: String = chars[start..i].iter().collect();
            result.push_str(&format!(
                "\n\n<div class=\"math-display\" data-latex=\"{}\"></div>\n\n",
                html_escape(&math)
            ));
            if i + 1 < chars.len() {
                i += 2; // skip closing $$
            }
            continue;
        }

        // Inline math: $...$  (not preceded by \)
        if chars[i] == '$' && (i == 0 || chars[i - 1] != '\\') {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '$' {
                i += 1;
            }
            let math: String = chars[start..i].iter().collect();
            result.push_str(&format!(
                "<span class=\"math-inline\" data-latex=\"{}\"></span>",
                html_escape(&math)
            ));
            if i < chars.len() {
                i += 1; // skip closing $
            }
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

fn postprocess_math(html: &mut String) {
    // Replace ```lean4 code blocks with syntax-highlighted HTML.
    // pulldown-cmark renders fenced code blocks as <pre><code class="language-lean4">...</code></pre>.
    postprocess_lean4_blocks(html);
}

fn postprocess_lean4_blocks(html: &mut String) {
    let prefix = "<code class=\"language-lean4\">";
    let suffix = "</code>";
    let mut result = String::with_capacity(html.len());
    let mut remaining = html.as_str();

    while let Some(start_idx) = remaining.find(prefix) {
        result.push_str(&remaining[..start_idx]);
        remaining = &remaining[start_idx + prefix.len()..];

        if let Some(end_idx) = remaining.find(suffix) {
            let code_escaped = &remaining[..end_idx];
            // Unescape HTML entities that pulldown-cmark adds inside code blocks
            let code = code_escaped
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"");
            let highlighted = crate::lean_highlight::highlight_lean(&code);
            result.push_str("<code class=\"language-lean4\">");
            result.push_str(&highlighted);
            result.push_str(suffix);
            remaining = &remaining[end_idx + suffix.len()..];
        } else {
            result.push_str(prefix);
        }
    }
    result.push_str(remaining);
    *html = result;
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic_markdown() {
        let html = render_markdown("# Hello\n\nThis is a **test**.");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>test</strong>"));
    }

    #[test]
    fn test_render_inline_math() {
        let html = render_markdown("The formula $x^2 + 1$ is simple.");
        assert!(html.contains("math-inline"));
        assert!(html.contains("data-latex=\"x^2 + 1\""));
    }

    #[test]
    fn test_render_display_math() {
        let html = render_markdown("Here is a formula:\n\n$$\\frac{a}{b}$$\n\nDone.");
        assert!(html.contains("math-display"));
        assert!(html.contains("data-latex"));
    }
}
