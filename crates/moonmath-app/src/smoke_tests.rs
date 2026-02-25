//! Smoke tests — verify the web app renders key pages with expected content.
//!
//! Run with: `cargo test --features ssr -p moonmath-app`
//!
//! These tests start a real Axum server on a random port and use reqwest to
//! verify that pages render correctly via Leptos streaming SSR.
//!
//! Note: pages that load data from `target/site/data/` may show fallback or
//! error states if the SSG hasn't been run (`cargo run -p moonmath-ssg`).
//! The tests still pass because they check the HTML shell, not dynamic data.

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::net::SocketAddr;
use std::sync::Once;

use crate::app::{shell, App};

static SET_CWD: Once = Once::new();

/// Start the app on a random port and return the base URL.
async fn spawn_app() -> String {
    // The server reads data files from `target/site/` relative to CWD.
    // Ensure CWD is the workspace root so these paths resolve correctly.
    SET_CWD.call_once(|| {
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let _ = std::env::set_current_dir(workspace_root);
    });

    let cargo_toml = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let conf = get_configuration(Some(cargo_toml)).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Bind to port 0 → OS assigns a random available port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    format!("http://{addr}")
}

/// Fetch a page and return (status_code, body_html).
async fn get(base_url: &str, path: &str) -> (u16, String) {
    let url = format!("{base_url}{path}");
    let resp = reqwest::get(&url).await.unwrap();
    let status = resp.status().as_u16();
    let body = resp.text().await.unwrap();
    (status, body)
}

// ─── Homepage ───────────────────────────────────────────

#[tokio::test]
async fn homepage_renders_html_shell() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/").await;

    assert_eq!(status, 200, "homepage should return 200");
    assert!(html.contains("<html"), "should produce an HTML document");
    assert!(
        html.contains("moonmath-app.css"),
        "should reference the app stylesheet"
    );
    assert!(
        html.contains("katex"),
        "should reference KaTeX stylesheet for math rendering"
    );
}

#[tokio::test]
async fn homepage_renders_navigation() {
    let base = spawn_app().await;
    let (_, html) = get(&base, "/").await;

    assert!(html.contains("site-nav"), "should render the navigation bar");
    assert!(html.contains("MoonMath"), "should contain MoonMath branding");
    assert!(html.contains("/showcase"), "nav should link to showcase");
}

#[tokio::test]
async fn homepage_renders_hero_and_features() {
    let base = spawn_app().await;
    let (_, html) = get(&base, "/").await;

    // Hero section (static content, always rendered)
    assert!(html.contains("hero"), "should render the hero section");
    assert!(
        html.contains("Interactive math"),
        "hero subtitle should describe the project"
    );

    // Feature cards (static, below the Suspense boundary)
    assert!(html.contains("Algorithms"), "should show Algorithms card");
    assert!(
        html.contains("Lean4 Proofs"),
        "should show Lean4 Proofs card"
    );
}

// ─── Showcase Routes ────────────────────────────────────

#[tokio::test]
async fn showcase_index_renders() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/showcase").await;

    assert_eq!(status, 200, "showcase index should return 200");
    assert!(html.contains("site-nav"), "should render navigation shell");
    assert!(html.contains("Showcase"), "should contain page heading");
}

#[tokio::test]
async fn showcase_category_route_renders() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/showcase/number-theory").await;

    assert_eq!(status, 200, "showcase category should return 200");
    assert!(html.contains("site-nav"), "should render navigation shell");
}

#[tokio::test]
async fn showcase_detail_route_renders() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/showcase/number-theory/prime-theorem").await;

    assert_eq!(status, 200, "showcase detail should return 200");
    assert!(html.contains("site-nav"), "should render navigation shell");
}

// ─── Legacy Route ───────────────────────────────────────

#[tokio::test]
async fn legacy_prime_theorem_route_renders() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/showcase-legacy/prime-theorem").await;

    assert_eq!(status, 200, "legacy route should return 200");
    assert!(html.contains("site-nav"), "should render navigation shell");
}

// ─── 404 / Unknown Route ────────────────────────────────

#[tokio::test]
async fn unknown_route_still_renders_shell() {
    let base = spawn_app().await;
    let (status, html) = get(&base, "/this-page-does-not-exist").await;

    // Leptos SSR may return 200 (SPA shell with client-side 404) or 404
    assert!(
        status == 200 || status == 404,
        "should return 200 or 404, got {status}"
    );
    assert!(
        html.contains("<html"),
        "should still render an HTML document even for unknown routes"
    );
}

// ─── SSG Data (requires `cargo run -p moonmath-ssg` first) ──

#[tokio::test]
async fn homepage_loads_categories_from_ssg() {
    let data_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/site/data/showcase/categories.json"
    );
    if !std::path::Path::new(data_path).exists() {
        eprintln!("SKIP: SSG data not found — run `cargo run -p moonmath-ssg` first");
        return;
    }

    let base = spawn_app().await;
    let (_, html) = get(&base, "/").await;

    // When SSG data exists, the homepage should render real category cards
    assert!(
        html.contains("category-card") || html.contains("category-grid"),
        "homepage should render category cards from SSG data"
    );
    assert!(
        !html.contains("Failed to read"),
        "homepage should not show file-read errors when SSG data exists"
    );
}

#[tokio::test]
async fn showcase_detail_loads_content_from_ssg() {
    let data_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/site/data/showcase/number-theory/prime-theorem.json"
    );
    if !std::path::Path::new(data_path).exists() {
        eprintln!("SKIP: SSG data not found — run `cargo run -p moonmath-ssg` first");
        return;
    }

    let base = spawn_app().await;
    let (_, html) = get(&base, "/showcase/number-theory/prime-theorem").await;

    assert!(
        html.contains("prime") || html.contains("Prime"),
        "showcase detail should render content about primes"
    );
    assert!(
        !html.contains("Failed to read"),
        "should not show file-read errors when SSG data exists"
    );
}
