+++
title = "Channel Capacity"
description = "The BSC capacity C = 1 - H(p) is the maximum reliable bit rate; C(0)=1 and C(1/2)=0"
weight = 156
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "C = \\max_{P_X} I(X;Y) = 1 - H(p)"
prerequisites = ["shannon-entropy", "mutual-information"]
lean4_status = "complete"
+++

## Statement

The **channel capacity** of a discrete memoryless channel is

$$C = \max_{P_X} I(X; Y)$$

where the maximum is over all input distributions $P_X$ and $I(X;Y)$ is the mutual information. For the **binary symmetric channel (BSC)** with crossover probability $p \in [0,1]$:

$$C_{\text{BSC}}(p) = 1 - H_2(p) = 1 + p \log_2 p + (1-p) \log_2(1-p)$$

(measured in bits, with $H_2$ the binary entropy in bits).

Boundary values: $C(0) = 1$ (perfect channel), $C(1/2) = 0$ (pure noise).

## Visualization

BSC capacity as a function of crossover probability $p$:

| $p$   | $H_2(p)$ (bits) | $C = 1 - H_2(p)$ |
|-------|----------------|-----------------|
| 0.00  | 0.000          | **1.000**        |
| 0.05  | 0.286          | 0.714            |
| 0.10  | 0.469          | 0.531            |
| 0.15  | 0.610          | 0.390            |
| 0.25  | 0.811          | 0.189            |
| 0.50  | 1.000          | **0.000**        |

ASCII sketch of $C(p)$:

```
C
1 *
  |*
  | *
  |  *
  |    *
  |        *
  |               *
0 +-------------------> p
  0      1/4     1/2
```

The BSC is symmetric about $p = 1/2$, so $C(p) = C(1-p)$. The maximum input distribution achieving capacity is uniform ($P_X(0) = P_X(1) = 1/2$).

## Proof Sketch

1. For the BSC, $Y = X \oplus Z$ where $Z \sim \text{Bernoulli}(p)$ independent of $X$.
2. $I(X;Y) = H(Y) - H(Y|X) = H(Y) - H(Z) = H(Y) - H_2(p)$.
3. $H(Y) \le 1$ with equality when $Y$ is uniform, achieved by the uniform input.
4. Therefore $C = \max H(Y) - H_2(p) = 1 - H_2(p)$.
5. At $p = 0$: $H_2(0) = 0$, so $C = 1$. At $p = 1/2$: $H_2(1/2) = 1$, so $C = 0$.

## Connections

Channel capacity is defined via [[Mutual Information]] and saturated by the Gaussian channel variant of the [[Cauchy-Schwarz Inequality]]. The noiseless channel ($p = 0$) achieves the limit established by the [[Source Coding Theorem]].

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.BinaryEntropy

open Real

/-- BSC capacity at crossover p=0: no noise means C=1. -/
theorem bsc_capacity_zero : 1 - binEntropy (0 : ℝ) = 1 := by
  simp [binEntropy_zero]

/-- BSC capacity at crossover p=1/2: maximum noise means C=0. -/
theorem bsc_capacity_half : 1 - binEntropy (2 : ℝ)⁻¹ = 1 - log 2 := by
  simp [binEntropy_two_inv]

/-- BSC capacity at p=1 equals BSC capacity at p=0 by symmetry of binEntropy. -/
theorem bsc_capacity_symm (p : ℝ) : binEntropy p = binEntropy (1 - p) :=
  (binEntropy_one_sub p).symm
```
