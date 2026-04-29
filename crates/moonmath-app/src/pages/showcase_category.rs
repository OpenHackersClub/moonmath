use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};
use crate::fetch::json_resource;

#[component]
pub fn ShowcaseCategoryPage() -> impl IntoView {
    let params = use_params_map();

    let pages = json_resource::<String, Vec<moonmath_types::ShowcasePageSummary>>(
        move || params.read().get("category").unwrap_or_default(),
        |category| format!("/data/showcase/{}/pages.json", category),
    );

    let categories =
        crate::fetch::json_resource_once::<Vec<moonmath_types::ShowcaseCategory>>(
            "/data/showcase/categories.json",
        );

    let category_slug = move || params.read().get("category").unwrap_or_default();

    view! {
        <Suspense fallback=move || view! {
            <div class="loading">"Loading category..."</div>
        }>
            {move || Suspend::new(async move {
                let cats: Vec<moonmath_types::ShowcaseCategory> = categories.await.unwrap_or_default();
                match pages.await {
                    Ok(page_list) => {
                        let cat = category_slug();
                        let title = cats.iter()
                            .find(|c| c.slug == cat)
                            .map(|c| c.title.clone())
                            .unwrap_or_else(|| cat.clone());
                        let title_for_meta = title.clone();
                        let title_for_header = title.clone();
                        view! {
                            <Title text=format!("{} — MoonMath Showcase", title_for_meta)/>
                            <div class="showcase-category-page">
                                <Breadcrumbs crumbs=vec![
                                    Crumb { label: "Home".into(), href: "/".into() },
                                    Crumb { label: "Showcase".into(), href: "/showcase".into() },
                                    Crumb { label: title.clone(), href: format!("/showcase/{}", cat) },
                                ]/>

                                <header class="category-header">
                                    <h1>{title_for_header}</h1>
                                </header>

                                <div class="showcase-card-grid">
                                    {page_list.into_iter().map(|page| {
                                        let href = format!("/showcase/{}/{}", page.category, page.slug);
                                        view! {
                                            <A href=href attr:class="showcase-card">
                                                <h3 class="showcase-card-title">{page.title}</h3>
                                                <p class="showcase-card-desc">{page.description}</p>
                                                <div class="showcase-card-meta">
                                                    {page.premier.then(|| view! {
                                                        <span class="premier-badge">"Premier"</span>
                                                    })}
                                                    <div class="tags">
                                                        {page.tags.into_iter().map(|tag| view! {
                                                            <span class="tag">{tag}</span>
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            </A>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    }
                    Err(_) => {
                        view! {
                            <div class="not-found">
                                <h1>"Category not found"</h1>
                                <p>"This category doesn't exist or content hasn't been generated yet."</p>
                                <A href="/showcase" attr:class="feature-link">"Back to showcase"</A>
                            </div>
                        }.into_any()
                    }
                }
            })}
        </Suspense>
    }
}
