use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};
use crate::components::compile_panel::CompilePanel;
use crate::components::fractal_canvas::FractalVisualizations;
use crate::components::ifs_canvas::Ifs3dCanvas;
use crate::fetch::json_resource;

#[server(CompileLean, "/api")]
pub async fn compile_lean(code: String) -> Result<moonmath_types::CompileResponse, ServerFnError> {
    use moonmath_lean::compiler::LeanCompiler;
    use moonmath_lean::leantex::lean_to_latex;
    use moonmath_types::CompileRequest;

    let compiler = LeanCompiler::with_defaults();
    let mut response = compiler
        .compile(CompileRequest { code: code.clone() })
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latex = lean_to_latex(&code);
    if !latex.is_empty() {
        response.latex = Some(latex);
    }

    Ok(response)
}

#[component]
pub fn ShowcaseDetailPage() -> impl IntoView {
    let params = use_params_map();

    let detail = json_resource::<(String, String), moonmath_types::ShowcaseDetailResponse>(
        move || {
            let p = params.read();
            (
                p.get("category").unwrap_or_default(),
                p.get("slug").unwrap_or_default(),
            )
        },
        |(category, slug)| format!("/data/showcase/{}/{}.json", category, slug),
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="loading">"Loading..."</div>
        }>
            {move || Suspend::new(async move {
                match detail.await {
                    Ok(data) => {
                        let title_for_meta = data.title.clone();
                        let title_for_crumb = data.title.clone();
                        let cat_for_nav = data.category_slug.clone();
                        let cat_for_next = data.category_slug.clone();
                        let lean4_blocks = data.lean4_blocks.clone();
                        let lean4_sources = data.lean4_sources.clone();
                        let has_fractal_viz =
                            data.tags.iter().any(|t| t == "fractal")
                            && data.tags.iter().any(|t| t == "visualization");
                        let has_ifs_3d = data.tags.iter().any(|t| t == "ifs-3d");
                        view! {
                            <Title text=format!("{} — MoonMath Showcase", title_for_meta)/>
                            <div class="showcase-detail-page">
                                <Breadcrumbs crumbs=vec![
                                    Crumb { label: "Home".into(), href: "/".into() },
                                    Crumb { label: "Showcase".into(), href: "/showcase".into() },
                                    Crumb { label: data.category_title, href: format!("/showcase/{}", data.category_slug) },
                                    Crumb { label: title_for_crumb, href: String::new() },
                                ]/>

                                <header class="showcase-header">
                                    <h1>{data.title}</h1>
                                    <div class="showcase-detail-meta">
                                        {data.premier.then(|| view! {
                                            <span class="premier-badge">"Premier"</span>
                                        })}
                                        <div class="tags">
                                            {data.tags.into_iter().map(|tag| view! {
                                                <span class="tag">{tag}</span>
                                            }).collect_view()}
                                        </div>
                                    </div>
                                    {data.latex_html.map(|rendered| view! {
                                        <div class="showcase-main-formula">
                                            <div class="math-display" inner_html=rendered/>
                                        </div>
                                    })}
                                </header>

                                <div class="showcase-detail-content formula-content" inner_html=data.html/>

                                // Fractal visualizations (for pages tagged fractal + visualization)
                                {has_fractal_viz.then(|| view! { <FractalVisualizations/> })}

                                // 3D IFS scene (egui+eframe), gated on the `ifs-3d` tag
                                {has_ifs_3d.then(|| view! { <Ifs3dCanvas/> })}

                                // Lean4 code blocks (interactive with compile button)
                                {(!lean4_blocks.is_empty()).then(|| {
                                    let blocks = lean4_blocks.clone();
                                    let sources = lean4_sources.clone();
                                    view! {
                                        <section class="lean4-section">
                                            <h2>"Lean4 Proof"</h2>
                                            {blocks.into_iter().zip(sources.into_iter()).map(|(highlighted, source)| {
                                                let (compile_result, set_compile_result) = signal::<Option<Result<moonmath_types::CompileResponse, String>>>(None);
                                                let (compiling, set_compiling) = signal(false);
                                                let source_for_click = source.clone();
                                                let on_compile = move |_| {
                                                    let code = source_for_click.clone();
                                                    set_compiling.set(true);
                                                    set_compile_result.set(None);
                                                    leptos::task::spawn_local(async move {
                                                        let result = compile_lean(code).await;
                                                        set_compile_result.set(Some(result.map_err(|e| e.to_string())));
                                                        set_compiling.set(false);
                                                    });
                                                };
                                                view! {
                                                    <div class="lean-code-block">
                                                        <pre class="lean-code"><code inner_html=highlighted/></pre>
                                                        <div class="lean-compile-bar">
                                                            <button
                                                                class="compile-btn lean-compile-btn"
                                                                on:click=on_compile
                                                                disabled=move || compiling.get()
                                                            >
                                                                {move || if compiling.get() {
                                                                    "Compiling...".to_string()
                                                                } else {
                                                                    "Compile Proof".to_string()
                                                                }}
                                                            </button>
                                                        </div>
                                                        <CompilePanel result=compile_result compiling=compiling/>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </section>
                                    }
                                })}

                                // Backlinks ("Referenced by") section
                                {(!data.backlinks.is_empty()).then(|| {
                                    let links = data.backlinks;
                                    view! {
                                        <section class="backlinks-section">
                                            <h2 class="backlinks-title">"Referenced by"</h2>
                                            <ul class="backlinks-list">
                                                {links.into_iter().map(|bl| {
                                                    let href = format!("/showcase/{}/{}", bl.category, bl.slug);
                                                    view! {
                                                        <li class="backlink-item">
                                                            <A href=href attr:class="backlink-link">{bl.title}</A>
                                                            <span class="backlink-category">{bl.category_title}</span>
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </section>
                                    }
                                })}

                                // Prev/Next navigation
                                <nav class="showcase-prev-next">
                                    <div class="prev-next-left">
                                        {data.prev.map(|(slug, ptitle)| {
                                            let href = format!("/showcase/{}/{}", cat_for_nav, slug);
                                            view! {
                                                <A href=href attr:class="prev-next-link prev-link">
                                                    <span class="prev-next-arrow">"<"</span>
                                                    <span class="prev-next-label">{ptitle}</span>
                                                </A>
                                            }
                                        })}
                                    </div>
                                    <div class="prev-next-right">
                                        {data.next.map(|(slug, ntitle)| {
                                            let href = format!("/showcase/{}/{}", cat_for_next, slug);
                                            view! {
                                                <A href=href attr:class="prev-next-link next-link">
                                                    <span class="prev-next-label">{ntitle}</span>
                                                    <span class="prev-next-arrow">">"</span>
                                                </A>
                                            }
                                        })}
                                    </div>
                                </nav>
                            </div>
                        }.into_any()
                    }
                    Err(e) => {
                        view! {
                            <div class="not-found">
                                <h1>"Page not found"</h1>
                                <p>{e.to_string()}</p>
                                <A href="/showcase" attr:class="feature-link">"Back to showcase"</A>
                            </div>
                        }.into_any()
                    }
                }
            })}
        </Suspense>
    }
}
