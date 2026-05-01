+++
title = "Chernoff Bound"
description = "The tail probability P(X ≥ ε) is bounded exponentially by the moment generating function"
weight = 180
tags = ["lean4-proof", "probability", "visualization"]
latex = "P(X \\ge \\varepsilon) \\le e^{-t\\varepsilon} M_X(t) \\quad \\forall t \\ge 0"
prerequisites = ["markov-inequality", "chebyshev-inequality"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a real-valued random variable on a probability space $(\Omega, \mathcal{F}, P)$, and let $M_X(t) = \mathbb{E}[e^{tX}]$ be its **moment generating function**. For any $\varepsilon \in \mathbb{R}$ and $t \ge 0$ such that $M_X(t) < \infty$:

$$P(X \ge \varepsilon) \le e^{-t\varepsilon} M_X(t).$$

Optimising over $t$ gives the **Chernoff bound**:

$$P(X \ge \varepsilon) \le \inf_{t \ge 0} e^{-t\varepsilon} M_X(t) = e^{-\Lambda^*(\varepsilon)}$$

where $\Lambda^*(\varepsilon) = \sup_{t \ge 0} (t\varepsilon - \log M_X(t))$ is the **Legendre–Fenchel transform** of $\log M_X$.

## Visualization

$X \sim \text{Bin}(100, 1/2)$. Bound on $P(X \ge 70)$ via Chernoff vs Chebyshev vs true value:

```
P(Bin(100, 1/2) ≥ 70):

Method             Bound
─────────────────────────────────────────
True value         ≈ 2.8 × 10⁻⁷
Markov (t=0.7)     7/10 = 0.700          (useless)
Chebyshev          1/(4·(70-50)²/100)
                 = 0.00625               (weak)
Chernoff           e^{-2·100·(0.2)²}
                 = e^{-8} ≈ 3.4×10⁻⁴   (sharp)

Derivation of Chernoff for Bin(n,p):
  M_X(t) = (1-p+pe^t)^n = ((1+e^t)/2)^100  for p=1/2
  Optimise: t* = log(ε(1-p)/(p(1-ε))) ≈ log(70·50/(50·30)) ≈ 0.847
  Result: P(X≥70) ≤ exp(-100·D_KL(0.7 || 0.5)) ≈ e^{-8} ≈ 3.35×10⁻⁴

Where D_KL(0.7||0.5) = 0.7·log(1.4) + 0.3·log(0.6) ≈ 0.0855
So 100 · 0.0855 = 8.55,  bound = e^{-8.55} ≈ 1.9×10⁻⁴  (even tighter)
```

Simpler Hoeffding form for $X = \sum_{i=1}^n X_i$ with $X_i \in [0,1]$:

$$P\!\left(\frac{X}{n} \ge p + \delta\right) \le e^{-2n\delta^2}.$$

For $n=100$, $p=1/2$, $\delta=0.2$: bound $= e^{-2\cdot 100 \cdot 0.04} = e^{-8} \approx 3.35 \times 10^{-4}$.

## Proof Sketch

1. **Markov's inequality on $e^{tX}$.** Since $e^{tX} \ge 0$:
   $P(X \ge \varepsilon) = P(e^{tX} \ge e^{t\varepsilon}) \le \dfrac{\mathbb{E}[e^{tX}]}{e^{t\varepsilon}} = e^{-t\varepsilon} M_X(t)$.
2. **Optimise over $t \ge 0$.** Taking the infimum over all valid $t$ gives the tightest bound.
3. **Log-convexity.** $\log M_X$ is convex, so the optimisation has a unique minimiser $t^*$ satisfying $M_X'(t^*)/M_X(t^*) = \varepsilon$ (if $\varepsilon > \mathbb{E}[X]$).

## Connections

The Chernoff bound is a vast strengthening of [[Markov's Inequality]] (which gives a polynomial tail bound) and [[Chebyshev's Inequality]] (which gives a $1/\delta^2$ bound). The optimised form relates to the **large deviations** rate function and is dual to the [[Characteristic Function]] via the Legendre transform.

## Lean4 Proof

```lean4
import Mathlib.Probability.Moments.Basic

open ProbabilityTheory MeasureTheory Real

/-- Chernoff bound: the upper tail of X is bounded by e^{-tε} M_X(t).
    Mathlib: `ProbabilityTheory.measure_ge_le_exp_mul_mgf`. -/
theorem chernoff_bound
    {Ω : Type*} {m : MeasurableSpace Ω}
    {μ : Measure Ω} [IsFiniteMeasure μ]
    {X : Ω → ℝ}
    (ε t : ℝ) (ht : 0 ≤ t)
    (h_int : Integrable (fun ω => exp (t * X ω)) μ) :
    μ.real {ω | ε ≤ X ω} ≤ exp (-t * ε) * mgf X μ t :=
  measure_ge_le_exp_mul_mgf ε ht h_int
```
