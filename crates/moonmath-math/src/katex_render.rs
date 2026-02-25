use katex;

#[derive(Debug, thiserror::Error)]
pub enum MathRenderError {
    #[error("KaTeX rendering failed: {0}")]
    Katex(String),
}

/// Render a LaTeX string to HTML using KaTeX.
pub fn render_latex(latex: &str, display_mode: bool) -> Result<String, MathRenderError> {
    let opts = katex::Opts::builder()
        .display_mode(display_mode)
        .throw_on_error(false)
        .build()
        .map_err(|e| MathRenderError::Katex(e.to_string()))?;

    katex::render_with_opts(latex, &opts).map_err(|e| MathRenderError::Katex(e.to_string()))
}

/// Render inline math ($...$).
pub fn render_inline(latex: &str) -> Result<String, MathRenderError> {
    render_latex(latex, false)
}

/// Render display math ($$...$$).
pub fn render_display(latex: &str) -> Result<String, MathRenderError> {
    render_latex(latex, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_latex() {
        let html = render_inline("x^2").unwrap();
        assert!(html.contains("katex"));
    }

    #[test]
    fn test_render_display_mode() {
        let html = render_display("\\frac{a}{b}").unwrap();
        assert!(html.contains("katex"));
    }
}
