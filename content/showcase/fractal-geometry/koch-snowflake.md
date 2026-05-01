+++
title = "Koch Snowflake"
description = "Infinite perimeter, finite area — a fractal curve of dimension log 4 / log 3"
weight = 50
tags = ["lean4-proof", "visualization", "fractal"]
latex = "P_n = 3s \\left(\\frac{4}{3}\\right)^n"
prerequisites = ["iterated-function-systems", "hausdorff-dimension"]
lean4_status = "complete"
+++

## Statement

Starting from an equilateral triangle of side $s$, the Koch snowflake is built by repeatedly replacing each edge segment with a four-segment Koch bump (each new segment one-third the original length). After $n$ steps:

$$P_n = 3s \cdot \left(\frac{4}{3}\right)^n \xrightarrow{n \to \infty} \infty$$

The **area** converges:

$$A = \frac{2\sqrt{3}}{5} s^2$$

The **Hausdorff dimension** of the Koch curve is:

$$d_H = \frac{\log 4}{\log 3} \approx 1.2619$$

## Visualization

Each iteration replaces every edge with four edges of length $1/3$ of the original, multiplying the total count by $4$ and the perimeter by $4/3$.

```
Level 0 (equilateral triangle, s = 1):
    *
   / \
  /   \
 *-----*

  P_0 = 3

Level 1 (Koch bump on each edge):
    *
   / \
  /   \
 *-*-*-*
   \/

  P_1 = 4

Level 2 (four bumps on each segment):
  P_2 = 16/3 ≈ 5.33

Level 3:
  P_3 = 64/9 ≈ 7.11
```

Perimeter table (with $s = 1$):

| Level $n$ | $P_n = 3 \cdot (4/3)^n$ | Edge count | Edge length |
|---|---|---|---|
| 0 | $3$ | $3$ | $1$ |
| 1 | $4$ | $12$ | $1/3$ |
| 2 | $16/3 \approx 5.33$ | $48$ | $1/9$ |
| 3 | $64/9 \approx 7.11$ | $192$ | $1/27$ |
| 4 | $256/27 \approx 9.48$ | $768$ | $1/81$ |
| $n$ | $3 \cdot (4/3)^n$ | $3 \cdot 4^n$ | $(1/3)^n$ |
| $\infty$ | $\infty$ | $\infty$ | $0$ |

The area converges because $\sum_{k=1}^\infty 3 \cdot 4^{k-1} \cdot \frac{\sqrt{3}}{4} \cdot (s/3^k)^2$ is a geometric series with ratio $4/9 < 1$.

## Proof Sketch

**Perimeter recursion.** At step $n$, there are $3 \cdot 4^n$ edges each of length $s/3^n$, giving $P_n = 3 \cdot 4^n \cdot s/3^n = 3s \cdot (4/3)^n$. The recursion $P_{n+1} = (4/3) P_n$ follows immediately.

**Divergence.** Since $4/3 > 1$, the sequence $P_n \to \infty$.

**Area.** The area added at step $k$ is $3 \cdot 4^{k-1}$ new equilateral triangles each of side $s/3^k$, contributing $3 \cdot 4^{k-1} \cdot \frac{\sqrt{3}}{4} \cdot (s/3^k)^2$. Summing from $k=1$ to $\infty$ gives a geometric series of ratio $4/9$.

**Hausdorff dimension.** Four self-similar pieces each scaled by $1/3$: Moran equation $4 \cdot (1/3)^s = 1$ gives $s = \log 4 / \log 3$.

## Connections

The Koch snowflake is an [[Iterated Function Systems|IFS]] attractor on $\mathbb{R}^2$, with four contractions of ratio $1/3$. Its [[Hausdorff Dimension]] $\log 4/\log 3 \approx 1.26$ is strictly between 1 and 2. The [[Sierpinski Triangle]] is a sibling IFS fractal with dimension $\log 3/\log 2 \approx 1.585$. The [[Hausdorff Distance]] between successive Koch iterates $K_n$ and $K_{n+1}$ shrinks geometrically, which is precisely the IFS contraction rate. The [[Mandelbrot Set]] boundary is conjectured (but not proven) to have Hausdorff dimension 2.

## Lean4 Proof

The proof establishes the perimeter recursion and its divergence to $+\infty$, using `tendsto_pow_atTop_atTop_of_one_lt` from `Mathlib.Analysis.SpecificLimits.Basic`.

```lean4
import Mathlib.Analysis.SpecificLimits.Basic

namespace MoonMath

open Filter Topology

/-- The perimeter of the Koch snowflake at iteration `n`,
    starting from a triangle of side `s`. -/
def kochPerim (s : ℝ) : ℕ → ℝ
  | 0     => 3 * s
  | n + 1 => (4 / 3) * kochPerim s n

/-- Closed form: `kochPerim s n = 3 * s * (4/3)^n`. -/
theorem koch_perim_formula (s : ℝ) (n : ℕ) :
    kochPerim s n = 3 * s * (4 / 3) ^ n := by
  induction n with
  | zero     => simp [kochPerim]
  | succ n ih => simp only [kochPerim, ih]; ring

/-- The Koch snowflake perimeter diverges to `+∞` for any positive side length. -/
theorem koch_perim_tendsto_atTop (s : ℝ) (hs : 0 < s) :
    Tendsto (kochPerim s) atTop atTop := by
  simp_rw [koch_perim_formula]
  apply Tendsto.const_mul_atTop (by positivity)
  exact tendsto_pow_atTop_atTop_of_one_lt (by norm_num : (1 : ℝ) < 4 / 3)

end MoonMath
```

Key Mathlib lemmas used: `tendsto_pow_atTop_atTop_of_one_lt` (for base $4/3 > 1$), `Tendsto.const_mul_atTop` (scaling by the positive constant $3s$), and `positivity` for the sign of $3s$.
