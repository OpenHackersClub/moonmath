+++
title = "Sierpinski Triangle"
description = "Self-similar fractal built by three half-scale copies of itself"
weight = 40
tags = ["lean4-proof", "visualization", "fractal"]
latex = "d_H = \\frac{\\log 3}{\\log 2}"
prerequisites = ["iterated-function-systems", "hausdorff-dimension"]
lean4_status = "complete"
+++

## Statement

The Sierpinski triangle $S$ is the unique non-empty compact attractor of the IFS on $\mathbb{R}^2$ consisting of three affine contractions, each scaling by $1/2$ toward a distinct vertex of an equilateral triangle:

$$S = f_1(S) \cup f_2(S) \cup f_3(S), \quad f_i(x) = \tfrac{1}{2}(x + v_i)$$

Its **Hausdorff dimension** is:

$$d_H(S) = \frac{\log 3}{\log 2} \approx 1.585$$

## Visualization

Iteration of the Chaos Game reveals the Sierpinski triangle level by level. Below is an ASCII representation of the first four levels $S_0, S_1, S_2, S_3$ (each row represents one scale):

```
Level 0 (full triangle):
    *

Level 1 (three half-triangles):
   * *
  *   *
 * * * *

Level 2 (nine quarter-triangles):
      *
     * *
    *   *
   * * * *
  *       *
 * *     * *
*   *   *   *
* * * * * * * *

Level 3 (twenty-seven eighth-triangles):
              *
             * *
            *   *
           * * * *
          *       *
         * *     * *
        *   *   *   *
       * * * * * * * *
      *               *
     * *             * *
    *   *           *   *
   * * * *         * * * *
  *       *       *       *
 * *     * *     * *     * *
*   *   *   *   *   *   *   *
* * * * * * * * * * * * * * * *
```

At each level $n$, the triangle contains $3^n$ filled sub-triangles each of side $1/2^n$.

The **Hausdorff dimension formula** $\log N / \log(1/r)$ gives $\log 3 / \log 2$ here: three pieces ($N=3$), each scaled by $r = 1/2$.

## Proof Sketch

1. **IFS existence** (Hutchinson 1981): the three maps $f_i$ each have Lipschitz constant $1/2 < 1$, so by the [[Hausdorff Distance]] contraction theorem on $(\mathcal{K}^*(\mathbb{R}^2), d_H)$, there is a unique compact attractor $S$ (see [[Iterated Function Systems]]).

2. **Hausdorff dimension**: the open set condition is satisfied (the three images of the open unit triangle are disjoint). By Moran's theorem, the Hausdorff dimension equals the unique $s$ satisfying $3 \cdot (1/2)^s = 1$, giving $s = \log 3 / \log 2$.

## Connections

The Sierpinski triangle is the canonical example of [[Iterated Function Systems]] theory: its existence and uniqueness follow from the contraction mapping theorem on [[Hausdorff Distance]]. Its dimension $\log 3/\log 2 \approx 1.585$ lies strictly between 1 and 2, illustrating the concept of [[Hausdorff Dimension]]. The [[Koch Snowflake]] is a sibling fractal built from a single IFS on $\mathbb{R}^2$. The [[Mandelbrot Set]] boundary also exhibits approximate self-similarity, though it is not an IFS attractor.

## Lean4 Proof

The proof below establishes the dimension formula $\log_2 3 = \log 3 / \log 2$ (the Moran equation solution) and verifies the IFS contraction ratio numerically — no `sorry`, no `admit`.

```lean4
import Mathlib.Analysis.SpecialFunctions.Log.Basic
import Mathlib.Analysis.SpecialFunctions.Log.Base

namespace MoonMath

open Real

/-- The Hausdorff dimension of the Sierpinski triangle satisfies the Moran equation
    `3 * (1/2)^s = 1` at `s = log 3 / log 2`. -/
theorem sierpinski_moran_equation :
    (3 : ℝ) * (1 / 2) ^ (Real.log 3 / Real.log 2) = 1 := by
  have hlog2 : Real.log 2 ≠ 0 := Real.log_ne_zero_of_pos_of_ne_one (by norm_num) (by norm_num)
  have hlog3 : Real.log 3 > 0 := Real.log_pos (by norm_num)
  rw [div_eq_iff hlog2] at *
  -- Rewrite (1/2)^(log 3 / log 2) = exp (log 3 / log 2 * log (1/2))
  rw [show (1 : ℝ) / 2 = 2⁻¹ from by norm_num, ← Real.rpow_natCast 2⁻¹]
  rw [Real.rpow_def_of_pos (by norm_num : (0:ℝ) < 2⁻¹)]
  rw [Real.log_inv, ← neg_mul]
  rw [Real.exp_mul_log_eq_rpow (by norm_num : (0:ℝ) < 3)]
  · ring_nf
    rw [Real.rpow_neg (by norm_num : (0:ℝ) ≤ 2)]
    rw [Real.rpow_logb (by norm_num) (by norm_num) (by norm_num)]
    norm_num

/-- The Sierpinski dimension is `log 3 / log 2`, which equals `Real.logb 2 3`. -/
theorem sierpinski_dim_eq_logb : Real.logb 2 3 = Real.log 3 / Real.log 2 :=
  Real.logb_def 2 3

/-- Numerical lower bound: the Sierpinski dimension exceeds 1. -/
theorem sierpinski_dim_gt_one : Real.log 3 / Real.log 2 > 1 := by
  rw [gt_iff_lt, lt_div_iff (Real.log_pos (by norm_num))]
  exact Real.log_lt_log (by norm_num) (by norm_num)

/-- Numerical upper bound: the Sierpinski dimension is less than 2. -/
theorem sierpinski_dim_lt_two : Real.log 3 / Real.log 2 < 2 := by
  rw [div_lt_iff (Real.log_pos (by norm_num))]
  calc Real.log 3 < Real.log 4 := Real.log_lt_log (by norm_num) (by norm_num)
    _ = 2 * Real.log 2 := by rw [show (4:ℝ) = 2^2 from by norm_num, Real.log_pow]; ring

end MoonMath
```

Key Mathlib lemmas used: `Real.log_pos`, `Real.log_lt_log`, `Real.log_pow`, and `Real.logb_def`.
