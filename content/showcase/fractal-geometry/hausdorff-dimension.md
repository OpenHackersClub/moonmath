+++
title = "Hausdorff Dimension"
description = "Measuring the fractional dimension of self-similar sets"
weight = 20
tags = ["visualization", "fractal"]
latex = "d = \\frac{\\log N}{\\log (1/r)}"
prerequisites = []
lean4_status = "sorry"
+++

## Definition

For a self-similar set composed of $N$ copies of itself scaled by a factor $r$, the Hausdorff (similarity) dimension is:

$$d = \frac{\log N}{\log (1/r)}$$

## Classical Examples

| Fractal | $N$ | $r$ | Dimension |
|---------|-----|-----|-----------|
| Cantor set | 2 | 1/3 | $\log 2 / \log 3 \approx 0.631$ |
| Sierpinski triangle | 3 | 1/2 | $\log 3 / \log 2 \approx 1.585$ |
| Koch curve | 4 | 1/3 | $\log 4 / \log 3 \approx 1.262$ |
| Sierpinski carpet | 8 | 1/3 | $\log 8 / \log 3 \approx 1.893$ |
| Menger sponge | 20 | 1/3 | $\log 20 / \log 3 \approx 2.727$ |

## Box-Counting Method

In practice, the Hausdorff dimension can be estimated by the box-counting dimension: cover the set with boxes of side length $\epsilon$ and count the number $N(\epsilon)$ of boxes needed. Then:

$$d = \lim_{\epsilon \to 0} \frac{\log N(\epsilon)}{\log(1/\epsilon)}$$

## Connections

The similarity dimension formula is used to compute the dimension of attractors of [[Iterated Function Systems]]. The boundary of the [[Mandelbrot Set]] famously has Hausdorff dimension 2.

## Lean4 Proof

```lean4
/-- Similarity dimension: for a self-similar set with N copies
    each scaled by factor r, the dimension d satisfies N · r^d = 1,
    giving d = log(N) / log(1/r). -/
noncomputable def similarityDimension (N : Nat) (r : ℝ) (hN : N ≥ 2) (hr : 0 < r ∧ r < 1) : ℝ :=
  Real.log N / Real.log (1 / r)

/-- The Cantor set has similarity dimension log(2)/log(3). -/
theorem cantor_dimension :
    similarityDimension 2 (1/3) (by norm_num) ⟨by norm_num, by norm_num⟩ =
      Real.log 2 / Real.log 3 := by
  sorry -- unfold definition and simplify log(1/(1/3)) = log(3)
```
