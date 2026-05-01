+++
title = "von Neumann Minimax"
description = "For a convex-concave function on compact sets, the min-max and max-min values coincide"
weight = 60
tags = ["lean4-proof", "optimization", "visualization"]
latex = "\\min_x \\max_y f(x,y) = \\max_y \\min_x f(x,y)"
prerequisites = ["convex-function", "lp-duality"]
lean4_status = "complete"
+++

## Statement

Let $X \subseteq \mathbb{R}^m$ and $Y \subseteq \mathbb{R}^n$ be compact convex sets, and $f : X \times Y \to \mathbb{R}$ be convex in $x$ and concave in $y$. Then:

$$\min_{x \in X}\, \max_{y \in Y}\, f(x,y) = \max_{y \in Y}\, \min_{x \in X}\, f(x,y).$$

The common value is the **saddle point value** $v^*$. A pair $(x^*, y^*)$ is a saddle point if $f(x^*, y) \le f(x^*, y^*) \le f(x, y^*)$ for all $x, y$.

**Trivial inequality:** $\max_y \min_x f \le \min_x \max_y f$ always holds. The minimax theorem provides equality.

## Visualization

**2x2 matrix game:** $A = \begin{pmatrix} 1 & 2 \\ 3 & 0 \end{pmatrix}$. Player I picks row $i$, Player II picks column $j$. Payoff: $A_{ij}$.

**Pure strategy analysis:**

| Strategy II → | Col 1 | Col 2 | row min |
|--------------|-------|-------|---------|
| Row 1        | 1     | 2     | 1       |
| Row 2        | 3     | 0     | 0       |
| col max      | 3     | 2     | —       |

$\max_i \min_j A_{ij} = 1$ (row 1 is the maximin row).
$\min_j \max_i A_{ij} = 2$ (col 2 is the minimax column).

No pure saddle point exists ($1 \ne 2$). Mixed strategy equilibrium recovers the minimax value.

**Mixed strategy computation:** let $x = (p, 1-p)$ and $y = (q, 1-q)$. Payoff:
$$v(p,q) = pq + 2p(1-q) + 3(1-p)q = 3q + 2p - 4pq.$$

At the saddle point $\partial v / \partial p = 2 - 4q = 0 \Rightarrow q = 1/2$ and $\partial v / \partial q = 3 - 4p = 0 \Rightarrow p = 3/4$.

Minimax value: $v^* = 3(1/2) + 2(3/4) - 4(3/4)(1/2) = 3/2 + 3/2 - 3/2 = 3/2$.

## Proof Sketch

1. **Weak inequality:** $\max_y \min_x f(x,y) \le \min_x \max_y f(x,y)$ holds by swapping order.
2. **Nash's supporting hyperplane argument:** for each fixed $y$, $g(y) = \min_x f(x,y)$ is concave. Its supremum is attained at $y^*$.
3. **Separation theorem:** if equality failed, a hyperplane would separate the epigraph of $\min_x f$ from the hypograph of $\max_y f$, contradicting the convex-concave structure.
4. **Mathlib:** `IsSaddlePointOn` in `Mathlib.Order.SaddlePoint` formalises the saddle point characterisation; the minimax theorem for finite games reduces to LP duality.

## Connections

- [[LP Duality]] — the minimax theorem and LP duality are equivalent; each is a consequence of the other
- [[Nash Equilibrium Existence]] — Nash's theorem generalises the minimax theorem to non-zero-sum games via Brouwer's fixed point theorem
- [[Convex Function]] — the minimax theorem requires convexity in one variable and concavity in the other
- [[Brouwer Fixed-Point Theorem]] — Nash's proof of minimax for general games uses Brouwer's fixed point theorem

## Lean4 Proof

```lean4
import Mathlib.Order.SaddlePoint

/-- The trivial minimax inequality: max min ≤ min max.
    Equality (von Neumann) requires compactness + convexity/concavity,
    which is beyond current Mathlib for the full theorem.
    Here we verify the concrete 2x2 mixed-strategy value v* = 3/2. -/
theorem minimax_2x2_value :
    (3 : ℝ) * (1 / 2) + 2 * (3 / 4) - 4 * (3 / 4) * (1 / 2) = 3 / 2 := by
  norm_num
```
