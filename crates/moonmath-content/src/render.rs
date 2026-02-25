/// Shared HTML math-rendering utilities used by both the SSG generator
/// and the Leptos SSR server.
///
/// Requires the `math-render` feature (which brings in `moonmath-math`).

/// Post-process markdown HTML to replace `data-latex` placeholders with
/// KaTeX-rendered HTML.
#[cfg(feature = "math-render")]
pub fn render_math_in_html(html: &str) -> String {
    let mut result = replace_math_placeholders(
        html,
        "<div class=\"math-display\" data-latex=\"",
        "\"></div>",
        true,
    );
    result = replace_math_placeholders(
        &result,
        "<span class=\"math-inline\" data-latex=\"",
        "\"></span>",
        false,
    );
    result
}

#[cfg(feature = "math-render")]
fn replace_math_placeholders(html: &str, prefix: &str, suffix: &str, display: bool) -> String {
    let mut result = String::with_capacity(html.len());
    let mut remaining = html;

    while let Some(start_idx) = remaining.find(prefix) {
        result.push_str(&remaining[..start_idx]);
        remaining = &remaining[start_idx + prefix.len()..];

        if let Some(end_idx) = remaining.find(suffix) {
            let latex_escaped = &remaining[..end_idx];
            let latex = html_unescape(latex_escaped);

            let rendered = moonmath_math::katex_render::render_latex(&latex, display)
                .unwrap_or_else(|e| format!("<span class=\"katex-error\">{}</span>", e));

            let (tag, class) = if display {
                ("div", "math-display")
            } else {
                ("span", "math-inline")
            };
            result.push_str(&format!("<{} class=\"{}\">{}</{}>", tag, class, rendered, tag));

            remaining = &remaining[end_idx + suffix.len()..];
        } else {
            // Malformed placeholder — pass through
            result.push_str(prefix);
        }
    }
    result.push_str(remaining);
    result
}

pub fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}
