use leptos::prelude::*;
use leptos_meta::*;

use crate::components::compile_panel::CompilePanel;
use crate::components::proof_viz::{ProofGraph, ProofWalkthrough};

const PRIME_THEOREM_SOURCE: &str = r#"import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Nat.Factorial.Basic

/-!
# Infinitude of Primes (Euclid's Proof)

A demonstration proof formalized in Lean 4 with Mathlib.
We use Euclid's classical argument: for any n, the number n! + 1
has a prime factor p, and p must be greater than n.
-/

/-- Our custom definition of primality, shown equivalent to Mathlib's `Nat.Prime`. -/
def IsPrime (p : Nat) : Prop :=
  p ≥ 2 ∧ ∀ m : Nat, m ∣ p → m = 1 ∨ m = p

/-- `Nat.Prime p` implies our custom `IsPrime p`. -/
theorem natPrime_to_isPrime {p : Nat} (hp : Nat.Prime p) : IsPrime p :=
  ⟨hp.two_le, hp.eq_one_or_self_of_dvd⟩

/-- Our custom `IsPrime p` implies `Nat.Prime p`. -/
theorem isPrime_to_natPrime {p : Nat} (hp : IsPrime p) : Nat.Prime p :=
  Nat.prime_def.mpr hp

/-- For any n ≥ 2, there exists a prime factor. Uses Mathlib's `Nat.exists_prime_and_dvd`. -/
theorem exists_prime_factor (n : Nat) (hn : n ≥ 2) :
    ∃ p, IsPrime p ∧ p ∣ n := by
  have h : n ≠ 1 := by omega
  obtain ⟨p, hp, hdvd⟩ := Nat.exists_prime_and_dvd h
  exact ⟨p, natPrime_to_isPrime hp, hdvd⟩

/-- n! ≥ 1 for all n. Follows directly from Mathlib's `Nat.factorial_pos`. -/
theorem factorial_pos (n : Nat) : Nat.factorial n ≥ 1 :=
  Nat.factorial_pos n

/-- If 1 ≤ p ≤ n then p ∣ n!. Follows directly from Mathlib's `Nat.dvd_factorial`. -/
theorem dvd_factorial (p n : Nat) (h1 : 1 ≤ p) (h2 : p ≤ n) :
    p ∣ Nat.factorial n :=
  Nat.dvd_factorial (by omega : 0 < p) h2

/-- For every natural number n, there exists a prime greater than n. -/
theorem InfinitudeOfPrimes (n : Nat) :
    ∃ p, p > n ∧ IsPrime p := by
  -- Consider n! + 1, which is ≥ 2
  have h1 : Nat.factorial n + 1 ≥ 2 := by
    have := Nat.factorial_pos n; omega
  -- n! + 1 has a prime factor p
  obtain ⟨p, hp, hdvd⟩ := exists_prime_factor _ h1
  use p
  constructor
  · -- Show p > n by contradiction
    by_contra h
    push_neg at h
    -- Since p ≥ 2 > 0 and p ≤ n, we have p ∣ n!
    have hp2 := hp.1  -- p ≥ 2
    have hdvd_fact : p ∣ Nat.factorial n :=
      dvd_factorial p n (by omega) h
    -- Since p ∣ (n! + 1) and p ∣ n!, we get p ∣ ((n! + 1) - n!) = p ∣ 1
    have hdvd_sub := Nat.dvd_sub hdvd hdvd_fact
    rw [Nat.add_sub_cancel_left] at hdvd_sub
    -- But p ≥ 2, contradicting p ∣ 1
    exact absurd (Nat.le_of_dvd Nat.one_pos hdvd_sub) (by omega)
  · exact hp
"#;

#[component]
pub fn PrimeShowcasePage() -> impl IntoView {
    let highlighted = crate::components::lean_code_block::highlight_lean(PRIME_THEOREM_SOURCE);

    view! {
        <Title text="Infinitude of Primes — MoonMath Showcase"/>
        <div class="showcase-page">
            <header class="showcase-header">
                <h1>"Infinitude of Primes"</h1>
                <p class="showcase-subtitle">
                    "A complete Lean4 proof that there are infinitely many prime numbers."
                </p>
            </header>

            <section class="showcase-section">
                <h2>"Lean4 Source"</h2>
                <div class="lean-code-block">
                    <pre class="lean-code"><code inner_html=highlighted/></pre>
                </div>
                {
                    let (compile_result, set_compile_result) = signal::<Option<Result<moonmath_types::CompileResponse, String>>>(None);
                    let (compiling, set_compiling) = signal(false);
                    view! {
                        <div class="lean-compile-bar">
                            <button
                                class="compile-btn lean-compile-btn"
                                on:click=move |_| {
                                    let code = PRIME_THEOREM_SOURCE.to_string();
                                    set_compiling.set(true);
                                    set_compile_result.set(None);
                                    leptos::task::spawn_local(async move {
                                        let result = crate::pages::showcase_detail::compile_lean(code).await;
                                        set_compile_result.set(Some(result.map_err(|e| e.to_string())));
                                        set_compiling.set(false);
                                    });
                                }
                                disabled=move || compiling.get()
                            >
                                {move || if compiling.get() {
                                    "Compiling...".to_string()
                                } else {
                                    "Compile Proof".to_string()
                                }}
                            </button>
                        </div>
                        <CompilePanel result=compile_result/>
                    }
                }
            </section>

            <section class="showcase-section">
                <h2>"Proof Dependency Graph"</h2>
                <p class="showcase-hint">"Click a node to expand its formula."</p>
                <ProofGraph/>
            </section>

            <section class="showcase-section">
                <h2>"Proof Walkthrough"</h2>
                <ProofWalkthrough/>
            </section>
        </div>
    }
}
