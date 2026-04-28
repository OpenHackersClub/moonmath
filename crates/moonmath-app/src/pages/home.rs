use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

use crate::fetch::json_resource_once;

#[component]
pub fn HomePage() -> impl IntoView {
    let categories =
        json_resource_once::<Vec<moonmath_types::ShowcaseCategory>>("/data/showcase/categories.json");

    view! {
        <Title text="MoonMath — Interactive Math Visualization"/>
        <div class="home-page">
            <section class="hero">
                <h1>"MoonMath"</h1>
                <p class="hero-subtitle">
                    "Interactive math visualization, algorithm demos, and Lean4 proof compilation."
                </p>
            </section>

            <section class="home-categories">
                <h2 class="home-section-title">"Explore by Category"</h2>
                <Suspense fallback=move || view! {
                    <div class="loading">"Loading categories..."</div>
                }>
                    {move || Suspend::new(async move {
                        match categories.await {
                            Ok(cats) if !cats.is_empty() => {
                                view! {
                                    <div class="category-grid">
                                        {cats.into_iter().map(|cat| {
                                            let href = format!("/showcase/{}", cat.slug);
                                            view! {
                                                <A href=href attr:class="category-card">
                                                    <h3 class="category-card-title">{cat.title}</h3>
                                                    <p class="category-card-desc">{cat.description}</p>
                                                    <span class="category-card-count">
                                                        {cat.page_count} {if cat.page_count == 1 { " topic" } else { " topics" }}
                                                    </span>
                                                </A>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                            _ => {
                                view! {
                                    <p class="showcase-hint">"Could not load categories. Explore the showcase directly:"</p>
                                    <div class="category-grid">
                                        <A href="/showcase" attr:class="category-card">
                                            <h3 class="category-card-title">"Browse All"</h3>
                                            <p class="category-card-desc">"Explore all mathematical topics and proofs."</p>
                                        </A>
                                    </div>
                                }.into_any()
                            }
                        }
                    })}
                </Suspense>
            </section>

            <section class="features">
                <div class="feature-card">
                    <h2>"Algorithms"</h2>
                    <p>"Interactive visualizations of sorting, graph traversal, and cryptographic algorithms."</p>
                    <span class="coming-soon">"Coming soon"</span>
                </div>

                <div class="feature-card">
                    <h2>"Lean4 Proofs"</h2>
                    <p>"Write, compile, and render Lean4 proofs with LaTeX and Typst output."</p>
                    <A href="/showcase/number-theory/prime-theorem" attr:class="feature-link">"View prime theorem proof"</A>
                </div>
            </section>
        </div>
    }
}
