//! Leptos mount for the egui+eframe IFS visualization.
//!
//! On SSR we render a placeholder `<canvas id="ifs-3d-canvas">`. On hydrate,
//! `eframe::WebRunner` boots into that canvas and takes over rendering.

use leptos::prelude::*;

const CANVAS_ID: &str = "ifs-3d-canvas";

/// Shows the egui-driven 3D IFS scene (Falconer Fig 9.3) on pages tagged
/// `ifs-3d`. The canvas takes over the row width; the egui side panel hosts
/// camera and iteration-depth controls.
#[component]
pub fn Ifs3dCanvas() -> impl IntoView {
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
        <section class="ifs3d-section">
            <h2>"3D View — Falconer Figure 9.3"</h2>
            <p class="ifs3d-subtitle">
                "Three affine contractions S\u{2081}, S\u{2082}, S\u{2083} mapping the unit \
                 square onto rectangles. Iteration depth k is lifted to the z-axis so each \
                 generation of Method (a) sits in its own layer; the chaos-game cloud of \
                 Method (b) is overlaid in front. Drag to orbit, scroll to zoom."
            </p>
            <div class="ifs3d-frame">
                <canvas
                    id=CANVAS_ID
                    width="900"
                    height="540"
                    class="ifs3d-canvas"
                    tabindex="0"
                />
                <noscript>
                    <p class="ifs3d-noscript">
                        "The IFS 3D viewer requires JavaScript / WASM to be enabled."
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
                    Box::new(|_cc| Ok(Box::new(moonmath_egui::IfsApp::new()))),
                )
                .await;
            if let Err(err) = result {
                web_sys::console::error_1(
                    &format!("eframe failed to start: {err:?}").into(),
                );
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
