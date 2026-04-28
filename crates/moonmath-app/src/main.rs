#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use moonmath_app::app::{shell, App};
    use tower_http::services::ServeDir;

    tracing_subscriber::fmt::init();

    // Explicit server function registration (inventory auto-registration
    // doesn't work on all platforms, notably macOS).
    server_fn::axum::register_explicit::<moonmath_app::pages::showcase_detail::CompileLean>();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Serve SSG-generated JSON from target/ssg-data/ at /data/*.
    // This directory lives outside target/site/ so cargo-leptos won't wipe it.
    let app = Router::new()
        .route("/api/*tail", axum::routing::post(leptos_axum::handle_server_fns))
        .nest_service("/data", ServeDir::new("target/ssg-data"))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // No client-side main — see lib.rs for WASM entry point
}
