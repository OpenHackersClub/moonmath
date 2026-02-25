use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};
use crate::fetch::json_resource_once;

#[component]
pub fn ShowcaseIndexPage() -> impl IntoView {
    let categories =
        json_resource_once::<Vec<moonmath_types::ShowcaseCategory>>("/data/showcase/categories.json");

    view! {
        <Title text="Showcase — MoonMath"/>
        <div class="showcase-index-page">
            <Breadcrumbs crumbs=vec![
                Crumb { label: "Home".into(), href: "/".into() },
                Crumb { label: "Showcase".into(), href: "/showcase".into() },
            ]/>

            <header class="showcase-index-header">
                <h1>"Showcase"</h1>
                <p class="showcase-index-subtitle">
                    "Browse interactive explorations of theorems, proofs, and mathematical findings."
                </p>
            </header>

            <Suspense fallback=move || view! {
                <div class="loading">"Loading categories..."</div>
            }>
                {move || Suspend::new(async move {
                    match categories.await {
                        Ok(cats) => {
                            view! {
                                <div class="category-grid">
                                    {cats.into_iter().map(|cat| {
                                        let href = format!("/showcase/{}", cat.slug);
                                        view! {
                                            <A href=href attr:class="category-card">
                                                <h2 class="category-card-title">{cat.title}</h2>
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
                        Err(e) => {
                            view! {
                                <div class="error">
                                    <h2>"Failed to load categories"</h2>
                                    <p>{e.to_string()}</p>
                                </div>
                            }.into_any()
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}
