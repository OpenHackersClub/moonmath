use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, Route, Router, A};
use leptos_router::{StaticSegment, ParamSegment};

use crate::pages::concepts::ConceptsIndexPage;
use crate::pages::home::HomePage;
use crate::pages::inspirations::InspirationsPage;
use crate::pages::showcase::PrimeShowcasePage;
use crate::pages::showcase_index::ShowcaseIndexPage;
use crate::pages::showcase_category::ShowcaseCategoryPage;
use crate::pages::showcase_detail::ShowcaseDetailPage;

/// SSR shell — used by cargo-leptos dev server only.
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/moonmath-app.css"/>
                <link rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"
                    integrity="sha384-nB0miv6/jRmo5UMMR1wu3Gz6NLsoTkbqJghGIsx//Rlm+ZU03BU6SQNC66uf4l5+"
                    crossorigin="anonymous"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="MoonMath — Interactive Math Visualization"/>

        <Router>
            <ScrollToTop/>
            <Nav/>
            <main class="main-content">
                <FlatRoutes fallback=|| view! {
                    <div class="not-found">
                        <h1>"404"</h1>
                        <p>"Page not found."</p>
                        <A href="/" attr:class="feature-link">"Go home"</A>
                    </div>
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("inspirations") view=InspirationsPage/>
                    <Route
                        path=StaticSegment("showcase")
                        view=ShowcaseIndexPage
                    />
                    // Static `/showcase/concepts` must precede the
                    // `(showcase, :category)` dynamic match so the index page
                    // wins over a "concepts" category lookup.
                    <Route
                        path=(StaticSegment("showcase"), StaticSegment("concepts"))
                        view=ConceptsIndexPage
                    />
                    <Route
                        path=(StaticSegment("showcase"), ParamSegment("category"))
                        view=ShowcaseCategoryPage
                    />
                    <Route
                        path=(StaticSegment("showcase"), ParamSegment("category"), ParamSegment("slug"))
                        view=ShowcaseDetailPage
                    />
                    // Legacy: keep /showcase-legacy/prime-theorem working
                    <Route
                        path=(StaticSegment("showcase-legacy"), StaticSegment("prime-theorem"))
                        view=PrimeShowcasePage
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Scrolls the window to the top on every route change.
/// Must be placed inside a `<Router>` so `use_location()` has context.
#[component]
fn ScrollToTop() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        use leptos_router::hooks::use_location;
        let location = use_location();
        Effect::new(move |_| {
            let _ = location.pathname.get();
            if let Some(window) = web_sys::window() {
                let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        });
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav class="site-nav">
            <div class="nav-brand">
                <A href="/" attr:class="nav-logo">"MoonMath"</A>
            </div>
            <div class="nav-links">
                <A href="/" attr:class="nav-link">"Home"</A>
                <A href="/showcase" attr:class="nav-link">"Showcase"</A>
                <A href="/showcase/concepts" attr:class="nav-link">"Concepts"</A>
                <A href="/inspirations" attr:class="nav-link">"Inspirations"</A>
            </div>
        </nav>
    }
}
