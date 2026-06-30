//! Leptos mount for the egui+eframe HNSW visualization.
//!
//! On SSR we render a placeholder `<canvas id="hnsw-canvas">`. On hydrate,
//! `eframe::WebRunner` boots into that canvas and takes over rendering the
//! interactive HNSW index + nearest-neighbour search replay.

use leptos::prelude::*;

const CANVAS_ID: &str = "hnsw-canvas";

/// Shows the egui-driven HNSW scene on pages tagged `hnsw`. The egui side panel
/// hosts index/query/replay controls; the canvas shows the layer stack and the
/// greedy descent of a query through it.
#[component]
pub fn HnswCanvas() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |once: Option<()>| {
            if once.is_some() {
                return;
            }
            mount::start();
        });
    }

    view! {
        <section class="hnsw-section">
            <h2>"Interactive HNSW Search"</h2>
            <p class="hnsw-subtitle">
                "Each point is a vector in the unit square. Higher layers keep an \
                 exponentially thinner sample for long-range hops; layer 0 holds every \
                 point for fine search. Hit "<strong>"New query point"</strong>" and watch \
                 the search greedily descend the stack to the nearest neighbour. Drag to \
                 orbit, scroll to zoom."
            </p>
            <div class="hnsw-frame">
                <canvas
                    id=CANVAS_ID
                    width="900"
                    height="560"
                    class="hnsw-canvas"
                    tabindex="0"
                />
                <noscript>
                    <p class="hnsw-noscript">
                        "The HNSW viewer requires JavaScript / WASM to be enabled."
                    </p>
                </noscript>
            </div>
        </section>
    }
}

#[cfg(feature = "hydrate")]
mod mount {
    use super::CANVAS_ID;
    use std::cell::Cell;
    use wasm_bindgen::JsCast;

    thread_local! {
        /// Guard so the WebRunner boots only once even if the component
        /// re-mounts during navigation.
        static STARTED: Cell<bool> = const { Cell::new(false) };
    }

    pub fn start() {
        if STARTED.with(|s| s.replace(true)) {
            return;
        }
        let Some(canvas) = canvas_element() else {
            STARTED.with(|s| s.set(false));
            return;
        };

        let runner = eframe::WebRunner::new();
        let options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async move {
            let result = runner
                .start(
                    canvas,
                    options,
                    Box::new(|_cc| Ok(Box::new(moonmath_egui::HnswApp::new()))),
                )
                .await;
            if let Err(err) = result {
                web_sys::console::error_1(&format!("eframe failed to start: {err:?}").into());
                STARTED.with(|s| s.set(false));
            }
        });
    }

    fn canvas_element() -> Option<web_sys::HtmlCanvasElement> {
        let document = web_sys::window()?.document()?;
        let el = document.get_element_by_id(CANVAS_ID)?;
        el.dyn_into::<web_sys::HtmlCanvasElement>().ok()
    }
}
