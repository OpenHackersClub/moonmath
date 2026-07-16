//! Leptos mount for the egui+eframe AES-128-GCM visualization.
//!
//! On SSR we render a placeholder `<canvas id="gcm-canvas">`. On hydrate,
//! `eframe::WebRunner` boots into that canvas and animates the two GCM
//! machines: AES-CTR keystream generation and the GHASH accumulator folding
//! blocks through `GF(2^128)` into the authentication tag.

use leptos::prelude::*;

const CANVAS_ID: &str = "gcm-canvas";

/// Shows the egui-driven GCM scene on pages tagged `gcm`. The side panel hosts
/// key/nonce/AAD/plaintext inputs and playback controls; the canvas steps the
/// GHASH accumulation `X ← (X ⊕ Bᵢ)·H` and lights up the matching CTR column.
#[component]
pub fn GcmCanvas() -> impl IntoView {
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
        <section class="gcm-section">
            <h2>"Interactive AES-128-GCM"</h2>
            <p class="gcm-subtitle">
                "A complete, verifiable GCM run — real AES-128 and real "
                <strong>"GF(2¹²⁸)"</strong>" arithmetic, so every hex value matches a \
                 production library. Edit the key, nonce, associated data, or plaintext \
                 and watch both machines react: the "<strong>"CTR"</strong>" lane turns \
                 counters into a keystream that hides the message, while the "
                <strong>"GHASH"</strong>" lane folds each block into the accumulator "
                <em>"X ← (X ⊕ Bᵢ)·H"</em>". Press "<strong>"Play"</strong>" to step through \
                 the absorption block by block and see the authentication tag form."
            </p>
            <div class="gcm-frame">
                <canvas
                    id=CANVAS_ID
                    width="960"
                    height="620"
                    class="gcm-canvas"
                    tabindex="0"
                />
                <noscript>
                    <p class="gcm-noscript">
                        "The GCM visualizer requires JavaScript / WASM to be enabled."
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
                    Box::new(|_cc| Ok(Box::new(moonmath_egui::GcmApp::new()))),
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
