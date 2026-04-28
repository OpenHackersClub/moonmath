use leptos::prelude::*;
use leptos_router::components::A;

/// A single breadcrumb item.
pub struct Crumb {
    pub label: String,
    pub href: String,
}

/// Breadcrumb navigation component.
#[component]
pub fn Breadcrumbs(
    /// List of breadcrumb items. The last item is shown as plain text (current page).
    crumbs: Vec<Crumb>,
) -> impl IntoView {
    let len = crumbs.len();
    view! {
        <nav class="breadcrumbs" aria-label="Breadcrumb">
            <ol class="breadcrumb-list">
                {crumbs.into_iter().enumerate().map(|(i, crumb)| {
                    let is_last = i == len - 1;
                    view! {
                        <li class="breadcrumb-item">
                            {if is_last {
                                view! {
                                    <span class="breadcrumb-current">{crumb.label}</span>
                                }.into_any()
                            } else {
                                view! {
                                    <A href={crumb.href} attr:class="breadcrumb-link">{crumb.label}</A>
                                    <span class="breadcrumb-sep">"/"</span>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}
