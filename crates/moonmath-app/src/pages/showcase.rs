use leptos::prelude::*;
use leptos_meta::*;

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

/// Highlight lean code at compile time for the static showcase page.
fn highlight_lean_static(code: &str) -> String {
    // Simple inline highlighter matching the lean_code_block component
    let mut out = String::with_capacity(code.len() * 2);
    let chars: Vec<char> = code.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if i + 2 < len && chars[i] == '/' && chars[i + 1] == '-' && chars[i + 2] == '-' {
            let start = i;
            i += 3;
            while i + 1 < len && !(chars[i] == '-' && chars[i + 1] == '/') { i += 1; }
            if i + 1 < len { i += 2; }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!("<span class=\"lean-doc-comment\">{}</span>", escape_html(&text)));
            continue;
        }
        if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
            let start = i;
            while i < len && chars[i] != '\n' { i += 1; }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!("<span class=\"lean-comment\">{}</span>", escape_html(&text)));
            continue;
        }
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '"' { if chars[i] == '\\' { i += 1; } i += 1; }
            if i < len { i += 1; }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!("<span class=\"lean-string\">{}</span>", escape_html(&text)));
            continue;
        }
        if matches!(chars[i], '∀'|'∃'|'∧'|'∨'|'¬'|'∣'|'→'|'←'|'↔'|'≤'|'≥'|'≠'|'∈'|'∉'|'⊂'|'⊆'|'∅'|'×'|'⟨'|'⟩'|'λ') {
            out.push_str(&format!("<span class=\"lean-symbol\">{}</span>", chars[i]));
            i += 1;
            continue;
        }
        if chars[i].is_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') { i += 1; }
            let word: String = chars[start..i].iter().collect();
            let kw = ["def","theorem","lemma","by","have","let","obtain","suffices","intro","notation","where","if","then","else","match","with","fun","do","return","import","open","namespace","end","structure","class","instance","deriving","section","variable","example","noncomputable","private","protected","mutual","inductive","axiom","abbrev","set_option"];
            let tac = ["grind","simp","simp_all","by_cases","induction","exact","apply","rfl","ring","omega","norm_num","decide","trivial","constructor","cases","rcases","rintro","assumption","contradiction","linarith","positivity","field_simp","push_neg","use"];
            if kw.contains(&word.as_str()) {
                out.push_str(&format!("<span class=\"lean-kw\">{}</span>", escape_html(&word)));
            } else if tac.contains(&word.as_str()) {
                out.push_str(&format!("<span class=\"lean-tactic\">{}</span>", escape_html(&word)));
            } else if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                out.push_str(&format!("<span class=\"lean-type\">{}</span>", escape_html(&word)));
            } else if word.chars().all(|c| c.is_ascii_digit()) {
                out.push_str(&format!("<span class=\"lean-number\">{}</span>", escape_html(&word)));
            } else {
                out.push_str(&escape_html(&word));
            }
            continue;
        }
        out.push_str(&escape_html(&chars[i].to_string()));
        i += 1;
    }
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[component]
pub fn PrimeShowcasePage() -> impl IntoView {
    let highlighted = highlight_lean_static(PRIME_THEOREM_SOURCE);

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
                <div class="compile-controls">
                    <button class="compile-btn" disabled=true>
                        "Compile (coming in v0.4)"
                    </button>
                </div>
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
