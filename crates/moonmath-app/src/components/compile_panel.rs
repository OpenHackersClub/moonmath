use leptos::prelude::*;
use moonmath_types::ErrorSeverity;

use crate::components::math_display::MathDisplay;

/// Displays Lean4 compilation results.
#[component]
pub fn CompilePanel(
    /// Compilation result signal
    result: ReadSignal<Option<Result<moonmath_types::CompileResponse, String>>>,
) -> impl IntoView {
    view! {
        <div class="compile-panel">
            {move || {
                result.get().map(|res| match res {
                    Ok(resp) if resp.success => {
                        let latex = resp.latex.clone();
                        view! {
                            <div class="compile-success">
                                <span class="compile-icon">"✓"</span>
                                " Compilation successful"
                            </div>
                            {latex.map(|l| view! {
                                <div class="compile-latex">
                                    <p class="compile-latex-title">"Verified Statements:"</p>
                                    <MathDisplay latex=l display=true/>
                                </div>
                            })}
                        }.into_any()
                    }
                    Ok(resp) => {
                        let latex = resp.latex.clone();
                        let has_errors = !resp.errors.is_empty();
                        let raw_output = resp.output.clone();
                        view! {
                            <div class="compile-errors">
                                <p class="compile-errors-title">"Compilation errors:"</p>
                                {has_errors.then(|| {
                                    let errors = resp.errors;
                                    view! {
                                        <ul class="compile-error-list">
                                            {errors.into_iter().map(|err| {
                                                let badge_class = match err.severity {
                                                    ErrorSeverity::Error => "severity-error",
                                                    ErrorSeverity::Warning => "severity-warning",
                                                    ErrorSeverity::Info => "severity-info",
                                                };
                                                let severity_text = match err.severity {
                                                    ErrorSeverity::Error => "error",
                                                    ErrorSeverity::Warning => "warning",
                                                    ErrorSeverity::Info => "info",
                                                };
                                                view! {
                                                    <li class="compile-error-item">
                                                        <span class={format!("severity-badge {}", badge_class)}>
                                                            {severity_text}
                                                        </span>
                                                        <span class="error-location">
                                                            {format!("{}:{}", err.line, err.column)}
                                                        </span>
                                                        " "
                                                        {err.message}
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }
                                })}
                                {raw_output.map(|out| view! {
                                    <pre class="compile-raw-output">{out}</pre>
                                })}
                            </div>
                            {latex.map(|l| view! {
                                <div class="compile-latex">
                                    <p class="compile-latex-title">"Extracted Statements:"</p>
                                    <MathDisplay latex=l display=true/>
                                </div>
                            })}
                        }.into_any()
                    }
                    Err(e) => {
                        view! {
                            <div class="compile-errors">
                                <p class="compile-errors-title">{e}</p>
                            </div>
                        }.into_any()
                    }
                })
            }}
        </div>
    }
}
