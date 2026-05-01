+++
title = "Squeeze Theorem"
description = "When two bounding sequences converge to the same limit, any sequence caught between them must follow"
weight = 10
tags = ["lean4-proof", "analysis", "visualization"]
latex = "a_n \\leq b_n \\leq c_n,\\; \\lim a_n = \\lim c_n = L \\implies \\lim b_n = L"
prerequisites = []
lean4_status = "complete"
+++

The **squeeze theorem** (also called the sandwich or pinching theorem) is the workhorse for evaluating limits that resist direct computation. If two sequences converge to the same value and a third sequence is forever trapped between them, there is nowhere else for it to go.

## Statement

Let $a_n$, $b_n$, $c_n$ be real sequences. Suppose

$$a_n \leq b_n \leq c_n \quad \text{for all } n,$$

and

$$\lim_{n \to \infty} a_n = \lim_{n \to \infty} c_n = L.$$

Then $b_n$ converges and $\lim_{n \to \infty} b_n = L$.

The same statement holds for real-valued functions on a metric space, and more generally for any sequence valued in a topological ordered space.

## Visualization

The classic example is $b_n = \sin(n)/n$, squeezed between $a_n = -1/n$ and $c_n = 1/n$.

```
 n   |  a_n = -1/n  |  b_n = sin(n)/n  |  c_n = 1/n
-----|--------------|------------------|------------
  1  |  -1.000      |   0.841          |  1.000
  2  |  -0.500      |   0.455          |  0.500
  5  |  -0.200      |  -0.192          |  0.200
 10  |  -0.100      |  -0.054          |  0.100
 20  |  -0.050      |   0.046          |  0.050
 50  |  -0.020      |  -0.019          |  0.020
100  |  -0.010      |  -0.005          |  0.010
```

Both outer sequences tend to $0$, so $b_n \to 0$ even though $\sin(n)$ itself never settles.

A schematic view of the three curves converging:

```
 1/n  ···──────────────────────────────► 0
       \      sin(n)/n (oscillates)
-1/n  ···──────────────────────────────► 0
```

## Proof Sketch

Fix $\varepsilon > 0$. Since $a_n \to L$ there exists $N_1$ such that $|a_n - L| < \varepsilon$ for $n \geq N_1$, and since $c_n \to L$ there exists $N_2$ such that $|c_n - L| < \varepsilon$ for $n \geq N_2$. For $n \geq \max(N_1, N_2)$:

$$L - \varepsilon < a_n \leq b_n \leq c_n < L + \varepsilon,$$

so $|b_n - L| < \varepsilon$.

## Connections

The squeeze theorem is the standard route to limits like $\lim_{x\to 0} x \sin(1/x) = 0$, and it underpins the proof of [[Intermediate Value Theorem]] (via the nested-interval argument). In the continuous setting it combines with the [[Mean Value Theorem]] to bound derivative estimates. The bounded-oscillation intuition resurfaces in [[Cauchy Criterion]] (a sequence that satisfies the Cauchy condition is itself squeezed between its own partial sup/inf). Within this section see also [[Monotone Convergence Theorem]] for the case where $b_n$ is forced to converge by monotonicity rather than by a bounding pair.

## Lean4 Proof

```lean4
import Mathlib.Topology.Order.Basic

open Filter Topology

/-- Squeeze theorem for sequences: a sequence sandwiched between two sequences
    with a common limit must share that limit. -/
theorem squeeze {a b c : ℕ → ℝ} {L : ℝ}
    (ha : Tendsto a atTop (𝓝 L))
    (hc : Tendsto c atTop (𝓝 L))
    (hab : ∀ n, a n ≤ b n)
    (hbc : ∀ n, b n ≤ c n) :
    Tendsto b atTop (𝓝 L) :=
  tendsto_of_tendsto_of_tendsto_of_le_of_le ha hc
    (fun n => hab n) (fun n => hbc n)
```
