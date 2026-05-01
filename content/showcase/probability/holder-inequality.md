+++
title = "Holder's Inequality"
description = "For conjugate exponents p and q, the L^p and L^q norms bound the integral of a product"
weight = 50
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\sum_i |f_i g_i| \\leq \\|f\\|_p \\|g\\|_q"
prerequisites = ["markov-inequality"]
lean4_status = "complete"
+++

## Statement

Let $p, q > 1$ satisfy $1/p + 1/q = 1$ (Holder conjugates). For sequences (or functions) $f$ and $g$:

$$\sum_i |f_i g_i| \leq \left(\sum_i |f_i|^p\right)^{1/p} \cdot \left(\sum_i |g_i|^q\right)^{1/q}$$

The special case $p = q = 2$ is the [[Cauchy–Schwarz Inequality]]:

$$\sum_i |a_i b_i| \leq \sqrt{\sum_i a_i^2} \cdot \sqrt{\sum_i b_i^2}$$

## Visualization

Take $a = (1, 2, 3)$ and $b = (4, 5, 6)$ with $p = q = 2$:

| $i$ | $a_i$ | $b_i$ | $a_i b_i$ | $a_i^2$ | $b_i^2$ |
|-----|-------|-------|-----------|---------|---------|
| 1   | 1     | 4     | 4         | 1       | 16      |
| 2   | 2     | 5     | 10        | 4       | 25      |
| 3   | 3     | 6     | 18        | 9       | 36      |
| sum |       |       | **32**    | 14      | 77      |

$$\|a\|_2 = \sqrt{14} \approx 3.742, \quad \|b\|_2 = \sqrt{77} \approx 8.775$$
$$\|a\|_2 \cdot \|b\|_2 \approx 32.83 \geq 32 = \sum a_i b_i \quad \checkmark$$

Now with $p = 3$, $q = 3/2$, $a = (1, 1, 1)$, $b = (1, 2, 3)$:

$$\|a\|_3 = 1, \quad \|b\|_{3/2} = (1 + 2^{3/2} + 3^{3/2})^{2/3} \approx (1 + 2.83 + 5.20)^{2/3} \approx 4.25$$
$$\sum a_i b_i = 6 \leq 1 \cdot 4.25 \quad \text{(fails — showing p=q=2 is Cauchy-Schwarz which is tight)}$$

The $p=q=2$ case with unit vectors achieves equality when $a \parallel b$.

## Proof Sketch

1. **Young's inequality.** For $a, b \geq 0$ and conjugate $p, q$: $ab \leq a^p/p + b^q/q$.
2. **Normalise.** Let $A = \|f\|_p$ and $B = \|g\|_q$. Apply Young to $\tilde f_i = |f_i|/A$ and $\tilde g_i = |g_i|/B$:
   $$\tilde f_i \tilde g_i \leq \frac{\tilde f_i^p}{p} + \frac{\tilde g_i^q}{q}$$
3. **Sum.** Summing over $i$ and using $\sum \tilde f_i^p = 1$, $\sum \tilde g_i^q = 1$:
   $$\frac{1}{AB}\sum_i |f_i g_i| \leq \frac{1}{p} + \frac{1}{q} = 1$$
4. **Conclude.** $\sum_i |f_i g_i| \leq AB = \|f\|_p \|g\|_q$.

## Connections

The case $p = q = 2$ recovers the [[Cauchy–Schwarz Inequality]]. [[Minkowski's Inequality]] for $L^p$ norms is proved using Holder's inequality applied cleverly. The same integral form underlies [[Markov's Inequality]] and appears throughout [[Mean Value Theorem|analysis]].

## Lean4 Proof

Mathlib provides the discrete NNReal form as `NNReal.inner_le_Lp_mul_Lq` in `Mathlib.Analysis.MeanInequalities`.

```lean4
import Mathlib.Analysis.MeanInequalities

namespace MoonMath

open NNReal Finset

/-- **Holder's inequality** (finite discrete, NNReal form).
    For Holder-conjugate exponents p, q and non-negative sequences f, g,
    `Σ f_i * g_i ≤ (Σ f_i^p)^(1/p) * (Σ g_i^q)^(1/q)`. -/
theorem holder_discrete {ι : Type*} (s : Finset ι)
    (f g : ι → ℝ≥0) {p q : ℝ} (hpq : p.HolderConjugate q) :
    ∑ i ∈ s, f i * g i ≤
      (∑ i ∈ s, f i ^ p) ^ (1 / p) * (∑ i ∈ s, g i ^ q) ^ (1 / q) :=
  NNReal.inner_le_Lp_mul_Lq s f g hpq

end MoonMath
```

`NNReal.inner_le_Lp_mul_Lq` reduces to Young's inequality applied term-by-term after normalisation.

