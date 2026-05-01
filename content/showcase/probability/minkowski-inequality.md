+++
title = "Minkowski's Inequality"
description = "The L^p norm satisfies the triangle inequality: ||f+g||_p <= ||f||_p + ||g||_p for p >= 1"
weight = 60
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\|f+g\\|_p \\leq \\|f\\|_p + \\|g\\|_p"
prerequisites = ["holder-inequality"]
lean4_status = "complete"
+++

## Statement

For $p \geq 1$ and sequences (or functions) $f, g$:

$$\left(\sum_i |f_i + g_i|^p\right)^{1/p} \leq \left(\sum_i |f_i|^p\right)^{1/p} + \left(\sum_i |g_i|^p\right)^{1/p}$$

This is the triangle inequality for the $\ell^p$ norm, asserting that $\ell^p$ is a genuine normed space for $p \geq 1$.

## Visualization

**Case $p = 2$ (Euclidean):** $f = (1, 1)$, $g = (1, -1)$.

```
  g = (1,-1)
        *
       /|
      / |
     /  | f+g = (2,0)
    *---*---------*
   (0,0) f=(1,1)  (2,0)
         *
```

| Vector  | Components  | $\ell^2$ norm            |
|---------|-------------|--------------------------|
| $f$     | $(1, 1)$    | $\sqrt{2} \approx 1.414$ |
| $g$     | $(1, -1)$   | $\sqrt{2} \approx 1.414$ |
| $f + g$ | $(2, 0)$    | $2$                      |

Minkowski: $\|f + g\|_2 = 2 \leq \sqrt{2} + \sqrt{2} \approx 2.828$. The triangle inequality holds.

**Case $p = 1$:** $\|f\|_1 = 2$, $\|g\|_1 = 2$, $\|f+g\|_1 = 2 \leq 4$. (Equality when $f, g$ same sign.)

**Case $p = \infty$ (limit):** $\|f\|_\infty = 1$, $\|g\|_\infty = 1$, $\|f+g\|_\infty = 2 \leq 2$. (Equality here.)

## Proof Sketch

1. **Write the sum.** $\sum |f_i + g_i|^p = \sum |f_i + g_i|^{p-1} |f_i + g_i| \leq \sum |f_i + g_i|^{p-1} (|f_i| + |g_i|)$.
2. **Apply Holder** with exponents $p$ and $q = p/(p-1)$ to each of $\sum |f_i + g_i|^{p-1} |f_i|$ and $\sum |f_i + g_i|^{p-1} |g_i|$:
   $$\sum |f_i + g_i|^{p-1} |f_i| \leq \left(\sum |f_i + g_i|^p\right)^{1/q} \cdot \|f\|_p$$
3. **Factor.** $\sum |f_i + g_i|^p \leq \left(\sum |f_i + g_i|^p\right)^{1/q} (\|f\|_p + \|g\|_p)$.
4. **Divide** both sides by $(\sum |f_i + g_i|^p)^{1/q}$ to get Minkowski.

## Connections

Minkowski's inequality is proved using [[Holder's Inequality]] as the key step. Together they establish that $L^p$ spaces are Banach spaces, the functional-analytic foundation underlying [[Chebyshev's Inequality]] (which uses $L^2$) and the Cauchy–Schwarz result [[Cauchy–Schwarz Inequality]].

## Lean4 Proof

Mathlib has `NNReal.Lp_add_le` in `Mathlib.Analysis.MeanInequalities` for the discrete NNReal form.

```lean4
import Mathlib.Analysis.MeanInequalities

namespace MoonMath

open NNReal Finset

/-- **Minkowski's inequality** (finite discrete, NNReal form).
    For p ≥ 1 and non-negative sequences f, g,
    `(Σ (f_i + g_i)^p)^(1/p) ≤ (Σ f_i^p)^(1/p) + (Σ g_i^p)^(1/p)`. -/
theorem minkowski_discrete {ι : Type*} (s : Finset ι)
    (f g : ι → ℝ≥0) {p : ℝ} (hp : 1 ≤ p) :
    (∑ i ∈ s, (f i + g i) ^ p) ^ (1 / p) ≤
      (∑ i ∈ s, f i ^ p) ^ (1 / p) + (∑ i ∈ s, g i ^ p) ^ (1 / p) :=
  NNReal.Lp_add_le s f g hp

end MoonMath
```

`NNReal.Lp_add_le` is proved by the argument in the proof sketch above, using `NNReal.inner_le_Lp_mul_Lq` (Holder) as the key tool.

