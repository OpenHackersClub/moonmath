use leptos::prelude::*;

use crate::components::math_display::MathDisplay;

// ── Proof Graph ──────────────────────────────────────────────────────

/// A node in the proof dependency graph.
struct GraphNode {
    id: &'static str,
    label: &'static str,
    latex: &'static str,
}

const LAYER_DEFS: &[GraphNode] = &[
    GraphNode { id: "is-prime", label: "IsPrime", latex: r"\text{IsPrime}(p) \;:\; p \ge 2 \;\wedge\; \forall m,\; m \mid p \Rightarrow m = 1 \lor m = p" },
    GraphNode { id: "factorial", label: "factorial", latex: r"n! = \prod_{i=1}^{n} i" },
];

const LAYER_INTERMEDIATE: &[GraphNode] = &[
    GraphNode { id: "exists-prime", label: "exists_prime_factor", latex: r"\forall n \ge 2,\; \exists p,\; \text{IsPrime}(p) \;\wedge\; p \mid n" },
    GraphNode { id: "factorial-pos", label: "factorial_pos", latex: r"\forall n,\; n! \ge 1" },
    GraphNode { id: "dvd-factorial", label: "dvd_factorial", latex: r"\forall p,\; 1 \le p \le n \;\Rightarrow\; p \mid n!" },
];

const LAYER_FINAL: &[GraphNode] = &[
    GraphNode { id: "infinitude", label: "InfinitudeOfPrimes", latex: r"\forall n,\; \exists p,\; p > n \;\wedge\; \text{IsPrime}(p)" },
];

/// Interactive proof dependency graph with three layers.
#[component]
pub fn ProofGraph() -> impl IntoView {
    let (expanded, set_expanded) = signal::<Option<&'static str>>(None);

    let make_node = move |node: &'static GraphNode| {
        let is_exp = move || expanded.get() == Some(node.id);
        let toggle = move |_| {
            set_expanded.set(if expanded.get() == Some(node.id) { None } else { Some(node.id) });
        };
        view! {
            <div class="graph-node" class:graph-node-expanded=is_exp on:click=toggle>
                <span class="graph-node-label">{node.label}</span>
                {move || is_exp().then(|| view! {
                    <div class="graph-node-formula">
                        <MathDisplay latex=node.latex.to_string() display=true/>
                    </div>
                })}
            </div>
        }
    };

    view! {
        <div class="proof-graph">
            <div class="graph-layer graph-layer-defs">
                <span class="layer-label">"Definitions"</span>
                <div class="layer-nodes">
                    {LAYER_DEFS.iter().map(make_node).collect_view()}
                </div>
            </div>
            <div class="graph-edges">
                <div class="edge"></div>
                <div class="edge"></div>
                <div class="edge"></div>
            </div>
            <div class="graph-layer graph-layer-intermediate">
                <span class="layer-label">"Lemmas"</span>
                <div class="layer-nodes">
                    {LAYER_INTERMEDIATE.iter().map(make_node).collect_view()}
                </div>
            </div>
            <div class="graph-edges">
                <div class="edge"></div>
                <div class="edge"></div>
                <div class="edge"></div>
            </div>
            <div class="graph-layer graph-layer-final">
                <span class="layer-label">"Theorem"</span>
                <div class="layer-nodes">
                    {LAYER_FINAL.iter().map(make_node).collect_view()}
                </div>
            </div>
        </div>
    }
}

// ── Proof Walkthrough ────────────────────────────────────────────────

struct WalkthroughStep {
    title: &'static str,
    latex: &'static str,
    lean_snippet: &'static str,
    explanation: &'static str,
    default_open: bool,
}

const STEPS: &[WalkthroughStep] = &[
    WalkthroughStep {
        title: "1. IsPrime definition",
        latex: r"\text{IsPrime}(p) \;\Leftrightarrow\; p \ge 2 \;\wedge\; \forall m,\; m \mid p \Rightarrow m = 1 \lor m = p",
        lean_snippet: "def IsPrime (p : Nat) : Prop :=\n  p ≥ 2 ∧ ∀ m : Nat, m ∣ p → m = 1 ∨ m = p",
        explanation: "A natural number p is prime if it is at least 2 and its only divisors are 1 and itself.",
        default_open: false,
    },
    WalkthroughStep {
        title: "2. Every n ≥ 2 has a prime factor",
        latex: r"\forall n \ge 2,\; \exists p,\; \text{IsPrime}(p) \;\wedge\; p \mid n",
        lean_snippet: "theorem exists_prime_factor (n : Nat) (hn : n ≥ 2) :\n    ∃ p, IsPrime p ∧ p ∣ n := by\n  grind",
        explanation: "By strong induction on n: if n is prime, it is its own prime factor. Otherwise n has a non-trivial divisor, and we recurse on the smaller factor.",
        default_open: false,
    },
    WalkthroughStep {
        title: "3. Factorial is positive",
        latex: r"\forall n \in \mathbb{N},\; n! \ge 1",
        lean_snippet: "theorem factorial_pos (n : Nat) : Nat.factorial n ≥ 1 := by\n  induction n with\n  | zero => simp [Nat.factorial]\n  | succ k ih => simp [Nat.factorial]; omega",
        explanation: "Base case: 0! = 1. Inductive step: (k+1)! = (k+1) · k! ≥ 1 · 1 = 1 since both factors are positive.",
        default_open: false,
    },
    WalkthroughStep {
        title: "4. Primes up to n divide n!",
        latex: r"1 \le p \le n \;\Rightarrow\; p \mid n!",
        lean_snippet: "theorem dvd_factorial (p n : Nat) (h1 : 1 ≤ p) (h2 : p ≤ n) :\n    p ∣ Nat.factorial n := by\n  induction n with\n  | zero => omega\n  | succ k ih =>\n    simp [Nat.factorial]\n    by_cases hpk : p ≤ k\n    · exact Dvd.dvd.mul_left (ih hpk) (k + 1)\n    · have : p = k + 1 := by omega\n      subst this; exact Dvd.intro (Nat.factorial k) rfl",
        explanation: "By induction on n. If p ≤ k, we use the inductive hypothesis. If p = k+1, then p appears as a factor in (k+1)!.",
        default_open: false,
    },
    WalkthroughStep {
        title: "5. A prime factor of n!+1 is > n",
        latex: r"p \mid (n!+1) \\;\\wedge\\; \\text{IsPrime}(p) \\;\\Rightarrow\\; p > n",
        lean_snippet: "-- If p | n!+1 and p ≤ n, then p | n! and p | 1,\n-- contradicting p ≥ 2. So p > n.",
        explanation: "If p ≤ n then p divides n! (by step 4). Since p also divides n!+1, it must divide (n!+1) − n! = 1, which contradicts p ≥ 2.",
        default_open: false,
    },
    WalkthroughStep {
        title: "6. Infinitude of Primes",
        latex: r"\forall n \in \mathbb{N},\; \exists p > n,\; \text{IsPrime}(p)",
        lean_snippet: "theorem InfinitudeOfPrimes (n : Nat) :\n    ∃ p, p > n ∧ IsPrime p := by\n  have h1 : Nat.factorial n + 1 ≥ 2 := by omega\n  obtain ⟨p, hp, hdvd⟩ := exists_prime_factor _ h1\n  use p\n  constructor\n  · by_contra h\n    push_neg at h\n    have : p ∣ Nat.factorial n := dvd_factorial p n hp.1.le h\n    have : p ∣ 1 := (Nat.dvd_sub' this hdvd)\n    omega\n  · exact hp",
        explanation: "Given any n, consider n! + 1. It is ≥ 2 so it has a prime factor p. This p cannot be ≤ n (it would divide both n! and n!+1, hence divide 1). Therefore p > n.",
        default_open: true,
    },
];

/// Accordion walkthrough of the proof steps.
#[component]
pub fn ProofWalkthrough() -> impl IntoView {
    view! {
        <div class="proof-walkthrough">
            {STEPS.iter().enumerate().map(|(idx, step)| {
                view! { <WalkthroughStepView step=step idx=idx/> }
            }).collect_view()}
        </div>
    }
}

#[component]
fn WalkthroughStepView(step: &'static WalkthroughStep, idx: usize) -> impl IntoView {
    let (open, set_open) = signal(step.default_open);
    let _ = idx; // used for key in parent

    view! {
        <div class="walkthrough-step" class:walkthrough-open=open>
            <button class="walkthrough-toggle" on:click=move |_| set_open.set(!open.get())>
                <span class="walkthrough-arrow">{move || if open.get() { "▾" } else { "▸" }}</span>
                " "
                {step.title}
            </button>
            {move || open.get().then(|| view! {
                <div class="walkthrough-content">
                    <div class="walkthrough-formula">
                        <MathDisplay latex=step.latex.to_string() display=true/>
                    </div>
                    <pre class="walkthrough-lean"><code>{step.lean_snippet}</code></pre>
                    <p class="walkthrough-explanation">{step.explanation}</p>
                </div>
            })}
        </div>
    }
}
