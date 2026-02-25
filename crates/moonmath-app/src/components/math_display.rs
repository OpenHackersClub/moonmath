use leptos::prelude::*;

/// Renders LaTeX math using KaTeX.
///
/// In SSG mode, math is pre-rendered by the SSG generator and delivered as HTML.
/// This component is kept for the legacy PrimeShowcasePage and ProofWalkthrough
/// which use it inline.
#[component]
pub fn MathDisplay(
    /// LaTeX source string
    latex: String,
    /// Whether to render in display mode (block) or inline
    #[prop(default = false)]
    display: bool,
) -> impl IntoView {
    let rendered = render_katex(latex, display);

    if display {
        view! {
            <div class="math-display" inner_html=rendered/>
        }
        .into_any()
    } else {
        view! {
            <span class="math-inline" inner_html=rendered/>
        }
        .into_any()
    }
}

fn render_katex(latex: String, display: bool) -> String {
    #[cfg(feature = "ssr")]
    {
        moonmath_math::katex_render::render_latex(&latex, display)
            .unwrap_or_else(|e| format!("<span class=\"katex-error\">{}</span>", e))
    }
    #[cfg(not(feature = "ssr"))]
    {
        // On the client, KaTeX HTML is already present from SSG pre-rendering.
        let _ = (latex, display);
        String::new()
    }
}

/// Post-process markdown HTML to replace `data-latex` placeholders with
/// KaTeX-rendered HTML. Delegates to moonmath_content::render when available.
#[cfg(feature = "ssr")]
pub fn render_math_in_html(html: &str) -> String {
    moonmath_content::render::render_math_in_html(html)
}
