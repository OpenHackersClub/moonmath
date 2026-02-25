use leptos::prelude::*;
use leptos_meta::*;

use crate::components::breadcrumbs::{Breadcrumbs, Crumb};

#[component]
pub fn InspirationsPage() -> impl IntoView {
    view! {
        <Title text="Inspirations & Attributions — MoonMath"/>
        <div class="inspirations-page">
            <Breadcrumbs crumbs=vec![
                Crumb { label: "Home".into(), href: "/".into() },
                Crumb { label: "Inspirations".into(), href: String::new() },
            ]/>

            <header class="inspirations-header">
                <h1>"Inspirations & Attributions"</h1>
                <p class="inspirations-subtitle">
                    "The ideas, tools, and people that shaped MoonMath."
                </p>
            </header>

            <section class="inspiration-section">
                <h2>"Lean4 & Interactive Theorem Proving"</h2>
                <div class="inspiration-list">
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://lean-lang.org/" target="_blank" rel="noopener">"Lean 4"</a>
                        </h3>
                        <p>"A functional programming language and interactive theorem prover developed at Microsoft Research. Lean4's approach to dependent type theory and its growing mathlib library inspire MoonMath's goal of making formal verification accessible to learners."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://leanprover-community.github.io/mathlib4_docs/" target="_blank" rel="noopener">"Mathlib"</a>
                        </h3>
                        <p>"The community-driven mathematics library for Lean 4, demonstrating that large-scale formalized mathematics is practical and collaborative."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://leanprover-community.github.io/mathematics_in_lean/" target="_blank" rel="noopener">"Mathematics in Lean"</a>
                        </h3>
                        <p>"Tutorial resource showing how mathematical proofs translate to Lean, a direct inspiration for MoonMath's interactive proof exploration."</p>
                    </div>
                </div>
            </section>

            <section class="inspiration-section">
                <h2>"Fractal Geometry"</h2>
                <div class="inspiration-list">
                    <div class="inspiration-card">
                        <h3>"Benoit Mandelbrot"</h3>
                        <p><em>"The Fractal Geometry of Nature"</em>" (1982). The foundational work on fractals as a language for describing natural complexity, inspiring MoonMath's visualization approach."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://en.wikipedia.org/wiki/Mandelbrot_set" target="_blank" rel="noopener">"Mandelbrot Set"</a>
                        </h3>
                        <p>"The iconic fractal demonstrating how simple iterative formulas produce infinite complexity — a core theme in MoonMath's visual explorations."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>"Iterated Function Systems"</h3>
                        <p>"Techniques for generating fractals (Sierpinski triangle, Barnsley fern, etc.) that MoonMath aims to make interactive."</p>
                    </div>
                </div>
            </section>

            <section class="inspiration-section">
                <h2>"Interactive Math Education"</h2>
                <div class="inspiration-list">
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://www.3blue1brown.com/" target="_blank" rel="noopener">"3Blue1Brown"</a>
                        </h3>
                        <p>"Grant Sanderson's visual approach to mathematics, showing that animation and interactivity transform abstract concepts into intuition."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://leastauthority.com/community-matters/moonmath-manual/" target="_blank" rel="noopener">"MoonMath Manual"</a>
                        </h3>
                        <p>"By Least Authority — an introduction to zk-SNARKs and the mathematics behind them, the direct namesake inspiration for this project."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://www.manim.community/" target="_blank" rel="noopener">"Manim"</a>
                        </h3>
                        <p>"Mathematical animation engine used by 3Blue1Brown, inspiring the visualization pipeline."</p>
                    </div>
                </div>
            </section>

            <section class="inspiration-section">
                <h2>"Tooling & Tech"</h2>
                <div class="inspiration-list">
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://leptos.dev/" target="_blank" rel="noopener">"Leptos"</a>
                        </h3>
                        <p>"Rust web framework enabling SSR + WASM hydration, chosen for a pure-Rust stack."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://katex.org/" target="_blank" rel="noopener">"KaTeX"</a>
                        </h3>
                        <p>"Fast LaTeX math rendering, used server-side via katex-rs for pre-rendered math output."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://www.egui.rs/" target="_blank" rel="noopener">"egui"</a>
                        </h3>
                        <p>"Immediate-mode GUI in Rust, planned for interactive canvas and code editor with no JS dependency."</p>
                    </div>
                </div>
            </section>

            <section class="inspiration-section">
                <h2>"Et Cetera"</h2>
                <div class="inspiration-list">
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://www.getzola.org/" target="_blank" rel="noopener">"Zola"</a>
                        </h3>
                        <p>"Static site generator whose content organization (TOML frontmatter, _index.md sections) inspired MoonMath's content pipeline."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>
                            <a href="https://typst.app/" target="_blank" rel="noopener">"Typst"</a>
                        </h3>
                        <p>"Modern typesetting system, planned for PDF export in later milestones."</p>
                    </div>
                    <div class="inspiration-card">
                        <h3>"LeanTeX"</h3>
                        <p>"Lean-to-LaTeX tooling, planned for the v0.5 milestone bridging formal proofs to rendered math."</p>
                    </div>
                </div>
            </section>
        </div>
    }
}
