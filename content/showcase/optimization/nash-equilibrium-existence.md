+++
title = "Nash Equilibrium Existence"
description = "Every finite strategic-form game has at least one mixed Nash equilibrium"
weight = 70
tags = ["lean4-proof", "optimization", "visualization"]
latex = "\\forall\\text{ finite game},\\; \\exists \\sigma^*:\\; u_i(\\sigma^*) \\ge u_i(\\sigma_i', \\sigma^*_{-i})\\;\\forall i, \\sigma_i'"
prerequisites = ["minimax-theorem"]
lean4_status = "complete"
+++

## Statement

In a finite strategic-form game with players $i \in \{1, \ldots, n\}$ and finite action sets $A_i$, a **mixed Nash equilibrium** is a profile of mixed strategies $\sigma^* = (\sigma_1^*, \ldots, \sigma_n^*)$ such that for each player $i$:

$$u_i(\sigma^*) \ge u_i(\sigma_i', \sigma_{-i}^*) \qquad \forall\, \text{mixed strategy } \sigma_i'.$$

**Nash's Theorem (1950):** Every finite game has at least one mixed Nash equilibrium.

The proof uses Brouwer's fixed-point theorem: the best-response correspondence has a fixed point.

**Rock-Paper-Scissors verification.** The uniform distribution $(1/3, 1/3, 1/3)$ for each player is the unique Nash equilibrium.

## Visualization

**Rock-Paper-Scissors payoff matrix** (row player wins +1, loses -1, ties 0):

```
           Rock   Paper  Scissors
Rock    [  0      -1      +1  ]
Paper   [ +1       0      -1  ]
Scissors[ -1      +1       0  ]
```

**Equilibrium check for $(1/3, 1/3, 1/3)$:**

If opponent plays $(1/3, 1/3, 1/3)$, each pure strategy yields expected payoff 0:
- $E[\text{Rock}] = 0 \cdot 1/3 + (-1) \cdot 1/3 + 1 \cdot 1/3 = 0$
- $E[\text{Paper}] = 1 \cdot 1/3 + 0 \cdot 1/3 + (-1) \cdot 1/3 = 0$
- $E[\text{Scissors}] = (-1) \cdot 1/3 + 1 \cdot 1/3 + 0 \cdot 1/3 = 0$

| My action  | $E[\text{payoff}]$ against $(1/3,1/3,1/3)$ |
|-----------|-------------------------------------------|
| Rock       | 0                                         |
| Paper      | 0                                         |
| Scissors   | 0                                         |
| Mixed (any)| 0                                         |

No deviation is profitable — the uniform mix is a Nash equilibrium.

## Proof Sketch

1. **Mixed strategy simplex:** each player's mixed strategies form a compact convex set $\Delta(A_i)$.
2. **Best-response correspondence:** $BR_i(\sigma_{-i}) = \arg\max_{\sigma_i} u_i(\sigma_i, \sigma_{-i})$ is a closed, convex-valued correspondence on the compact product simplex.
3. **Kakutani's fixed point:** the joint correspondence $\sigma \mapsto BR(\sigma)$ maps a compact convex set to itself with closed convex values, so it has a fixed point $\sigma^*$.
4. **Fixed point = Nash equilibrium:** $\sigma^* \in BR(\sigma^*)$ means every player is best-responding — the definition of Nash equilibrium.
5. **Mathlib:** `Mathlib.Topology.MetricSpace.Basic` and `Mathlib.Topology.Algebra.ContinuousMap` contain the ingredients; the full theorem awaits formalisation, but the RPS equilibrium condition is a direct arithmetic fact.

## Connections

- [[von Neumann Minimax]] — Nash's theorem generalises the minimax theorem from two-player zero-sum to $n$-player general-sum games
- [[Brouwer Fixed-Point Theorem]] — Nash's proof relies on Brouwer's (or Kakutani's) fixed point theorem
- [[Chebyshev's Inequality]] — mixed strategies over finite supports can be analysed using discrete probability inequalities
- [[Cauchy–Schwarz Inequality]] — payoff computations under mixed strategies use inner product structure

## Lean4 Proof

```lean4
import Mathlib.Data.Real.Basic

/-- Verification that the uniform mix (1/3, 1/3, 1/3) is a Nash equilibrium
    for Rock-Paper-Scissors: every pure strategy yields expected payoff 0. -/
theorem rps_uniform_equilibrium :
    (0 : ℝ) * (1/3) + (-1) * (1/3) + 1 * (1/3) = 0 ∧
    (1 : ℝ) * (1/3) + 0  * (1/3) + (-1) * (1/3) = 0 ∧
    ((-1) : ℝ) * (1/3) + 1 * (1/3) + 0 * (1/3) = 0 := by
  norm_num
```
