use leptos::prelude::*;
use leptos_meta::*;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};

#[component]
pub fn InspirationsPage() -> impl IntoView {
    view! {
        <Title text="Inspirations — MoonMath"/>
        <div class="ins-page">
            <Breadcrumbs crumbs=vec![
                Crumb { label: "Home".into(), href: "/".into() },
                Crumb { label: "Inspirations".into(), href: String::new() },
            ]/>

            <header class="ins-header">
                <h1 class="ins-title">"Inspirations"</h1>
                <p class="ins-subtitle">
                    <em>"The luminaries, ideas, and instruments that light the path."</em>
                </p>
                <div class="ins-rule" aria-hidden="true"></div>
            </header>

            // ── Wing: Lean4 & Interactive Theorem Proving ──────
            <section class="ins-wing">
                <div class="ins-divider" aria-hidden="true">
                    <span>"\u{2726} \u{2726} \u{2726}"</span>
                </div>
                <h2 class="ins-wing-title">"Lean 4 & Interactive Theorem Proving"</h2>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://lean-lang.org/" target="_blank" rel="noopener">"Lean 4"</a>
                        </h3>
                        <span class="ins-plaque-year">"2021"</span>
                    </div>
                    <p class="ins-plaque-desc">"A functional programming language and interactive theorem prover developed at Microsoft Research. Lean 4\u{2019}s approach to dependent type theory and its growing mathlib library inspire MoonMath\u{2019}s goal of making formal verification accessible to learners."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://leanprover-community.github.io/mathlib4_docs/" target="_blank" rel="noopener">"Mathlib"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"The community-driven mathematics library for Lean 4, demonstrating that large-scale formalized mathematics is practical and collaborative."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://leanprover-community.github.io/mathematics_in_lean/" target="_blank" rel="noopener">"Mathematics in Lean"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Tutorial resource showing how mathematical proofs translate to Lean, a direct inspiration for MoonMath\u{2019}s interactive proof exploration."</p>
                </div>
            </section>

            // ── Wing: Fractal Geometry ─────────────────────────
            <section class="ins-wing">
                <div class="ins-divider" aria-hidden="true">
                    <span>"\u{2726} \u{2726} \u{2726}"</span>
                </div>
                <h2 class="ins-wing-title">"Fractal Geometry"</h2>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">"Benoit Mandelbrot"</h3>
                        <span class="ins-plaque-year">"1982"</span>
                    </div>
                    <p class="ins-plaque-desc"><em>"The Fractal Geometry of Nature"</em>" \u{2014} the foundational work on fractals as a language for describing natural complexity, inspiring MoonMath\u{2019}s visualization approach."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://en.wikipedia.org/wiki/Mandelbrot_set" target="_blank" rel="noopener">"Mandelbrot Set"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"The iconic fractal demonstrating how simple iterative formulas produce infinite complexity \u{2014} a core theme in MoonMath\u{2019}s visual explorations."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">"Iterated Function Systems"</h3>
                    </div>
                    <p class="ins-plaque-desc">"Techniques for generating fractals (Sierpinski triangle, Barnsley fern, etc.) that MoonMath aims to make interactive."</p>
                </div>
            </section>

            // ── Wing: Interactive Math Education ───────────────
            <section class="ins-wing">
                <div class="ins-divider" aria-hidden="true">
                    <span>"\u{2726} \u{2726} \u{2726}"</span>
                </div>
                <h2 class="ins-wing-title">"Interactive Math Education"</h2>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://www.3blue1brown.com/" target="_blank" rel="noopener">"3Blue1Brown"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Grant Sanderson\u{2019}s visual approach to mathematics, showing that animation and interactivity transform abstract concepts into intuition."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://www.manim.community/" target="_blank" rel="noopener">"Manim"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Mathematical animation engine used by 3Blue1Brown, inspiring the visualization pipeline."</p>
                </div>
            </section>

            // ── Wing: Tooling & Tech ───────────────────────────
            <section class="ins-wing">
                <div class="ins-divider" aria-hidden="true">
                    <span>"\u{2726} \u{2726} \u{2726}"</span>
                </div>
                <h2 class="ins-wing-title">"Tooling & Tech"</h2>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://leptos.dev/" target="_blank" rel="noopener">"Leptos"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Rust web framework enabling SSR + WASM hydration, chosen for a pure-Rust stack."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://katex.org/" target="_blank" rel="noopener">"KaTeX"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Fast LaTeX math rendering, used server-side via katex-rs for pre-rendered math output."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://www.egui.rs/" target="_blank" rel="noopener">"egui"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Immediate-mode GUI in Rust, planned for interactive canvas and code editor with no JS dependency."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://www.getzola.org/" target="_blank" rel="noopener">"Zola"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Static site generator whose content organization (TOML frontmatter, _index.md sections) inspired MoonMath\u{2019}s content pipeline."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">
                            <a href="https://typst.app/" target="_blank" rel="noopener">"Typst"</a>
                        </h3>
                    </div>
                    <p class="ins-plaque-desc">"Modern typesetting system, planned for PDF export in later milestones."</p>
                </div>

                <div class="ins-plaque">
                    <div class="ins-plaque-header">
                        <h3 class="ins-plaque-name">"LeanTeX"</h3>
                    </div>
                    <p class="ins-plaque-desc">"Lean-to-LaTeX tooling, planned for the v0.5 milestone bridging formal proofs to rendered math."</p>
                </div>
            </section>
        </div>
    }
}
