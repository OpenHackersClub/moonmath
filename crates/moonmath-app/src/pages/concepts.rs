use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};
use crate::fetch::json_resource_once;

#[component]
pub fn ConceptsIndexPage() -> impl IntoView {
    let concepts = json_resource_once::<Vec<moonmath_types::ConceptEntry>>(
        "/data/showcase/concepts.json",
    );

    view! {
        <Title text="Concepts — MoonMath"/>
        <div class="concepts-page">
            <Breadcrumbs crumbs=vec![
                Crumb { label: "Home".into(), href: "/".into() },
                Crumb { label: "Showcase".into(), href: "/showcase".into() },
                Crumb { label: "Concepts".into(), href: String::new() },
            ]/>

            <header class="concepts-header">
                <h1>"Concepts"</h1>
                <p class="concepts-subtitle">
                    "Every cross-referenced page in MoonMath, sorted by how often it's cited.
                     Each row links back to the pages that mention it — follow the threads."
                </p>
            </header>

            <Suspense fallback=move || view! {
                <div class="loading">"Loading concepts..."</div>
            }>
                {move || Suspend::new(async move {
                    match concepts.await {
                        Ok(entries) if entries.is_empty() => view! {
                            <p class="concepts-empty">"No cross-references yet."</p>
                        }.into_any(),
                        Ok(entries) => view! {
                            <ul class="concepts-list">
                                {entries.into_iter().map(render_row).collect_view()}
                            </ul>
                        }.into_any(),
                        Err(e) => view! {
                            <div class="error">
                                <h2>"Failed to load concepts"</h2>
                                <p>{e.to_string()}</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

fn render_row(entry: moonmath_types::ConceptEntry) -> impl IntoView {
    let href = format!("/showcase/{}/{}", entry.category, entry.slug);
    let count = entry.reference_count;
    let count_label = if count == 1 { "reference" } else { "references" };
    let has_refs = !entry.referenced_by.is_empty();
    let refs = entry.referenced_by;
    let desc = entry.description;

    view! {
        <li class="concept-row">
            <div class="concept-row-head">
                <A href=href attr:class="concept-row-title">{entry.title}</A>
                <span class="concept-row-category">{entry.category_title}</span>
                <span class="concept-row-count">{count} " " {count_label}</span>
            </div>
            {(!desc.is_empty()).then(|| view! {
                <p class="concept-row-desc">{desc}</p>
            })}
            {has_refs.then(|| view! {
                <div class="concept-row-refs">
                    <span>"Referenced by:"</span>
                    {refs.into_iter().map(|bl| {
                        let h = format!("/showcase/{}/{}", bl.category, bl.slug);
                        view! {
                            <A href=h>{bl.title}</A>
                        }
                    }).collect_view()}
                </div>
            })}
        </li>
    }
}
