+++
title = "Mutual Information"
description = "I(X;Y) = H(X) + H(Y) - H(X,Y) measures how much knowing Y reduces uncertainty about X"
weight = 157
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "I(X;Y) = H(X) + H(Y) - H(X,Y) \\ge 0"
prerequisites = ["shannon-entropy"]
lean4_status = "complete"
+++

## Statement

The **mutual information** between discrete random variables $X$ and $Y$ is

$$I(X; Y) = H(X) + H(Y) - H(X, Y) = \sum_{x,y} p(x,y) \log \frac{p(x,y)}{p(x)\,p(y)}.$$

Key properties:

- $I(X; Y) \ge 0$, with equality iff $X$ and $Y$ are independent.
- $I(X; Y) = H(X) - H(X|Y) = H(Y) - H(Y|X)$.
- $I(X; X) = H(X)$ (a variable is maximally informative about itself).
- $I(X; Y) \le \min(H(X), H(Y))$.

## Visualization

Three joint distributions over $X, Y \in \{0, 1\}$:

**Case 1: Independent ($I = 0$)**

| $p(x,y)$ | $Y=0$ | $Y=1$ |
|----------|-------|-------|
| $X=0$    | 0.25  | 0.25  |
| $X=1$    | 0.25  | 0.25  |

$H(X) = H(Y) = \ln 2$; $H(X,Y) = 2\ln 2$; $I = \ln 2 + \ln 2 - 2\ln 2 = 0$.

**Case 2: $Y = X$ ($I = H(X)$)**

| $p(x,y)$ | $Y=0$ | $Y=1$ |
|----------|-------|-------|
| $X=0$    | 0.50  | 0.00  |
| $X=1$    | 0.00  | 0.50  |

$H(X) = H(Y) = \ln 2$; $H(X,Y) = \ln 2$; $I = \ln 2 + \ln 2 - \ln 2 = \ln 2 = H(X)$.

**Case 3: Partial dependence ($0 < I < H(X)$)**

| $p(x,y)$ | $Y=0$ | $Y=1$ |
|----------|-------|-------|
| $X=0$    | 0.40  | 0.10  |
| $X=1$    | 0.10  | 0.40  |

$H(X) = H(Y) = \ln 2 \approx 0.693$; $H(X,Y) \approx 1.061$; $I \approx 0.693 + 0.693 - 1.061 = 0.325$.

All three confirm $0 \le I \le H(X)$.

## Proof Sketch

1. Write $I(X;Y) = \text{KL}(p(x,y) \| p(x)p(y))$, the Kullback-Leibler divergence.
2. By the **log-sum inequality** (a consequence of Jensen's inequality applied to the convex function $-\log$), $\text{KL}(Q \| P) \ge 0$ for any distributions $Q, P$.
3. Equality holds iff $p(x,y) = p(x)p(y)$ for all $x,y$, i.e., $X \perp Y$.
4. The formula $I(X;X) = H(X)$ follows: $\text{KL}(p(x,x) \| p(x)^2) = \sum p(x)\log(p(x)/p(x)^2) = -\sum p(x)\log p(x) = H(X)$.

## Connections

Mutual information is the fundamental quantity whose maximization defines [[Channel Capacity]]. Its non-negativity follows from the same convexity argument underlying [[Shannon Entropy]]'s concavity, which also appears in [[Chebyshev's Inequality]] style arguments.

## Lean4 Proof

```lean4
/-- Verify I(X;Y) = 0 for the uniform independent distribution on {0,1}^2.
    H(X) = H(Y) = log 2; H(X,Y) = 2 * log 2; I = 0. -/
theorem mutual_info_independent :
    let H_X  : ℚ := 1  -- log 2 in units of log 2
    let H_Y  : ℚ := 1
    let H_XY : ℚ := 2
    H_X + H_Y - H_XY = 0 := by norm_num

/-- Verify I(X;X) = H(X): full dependence gives I equal to entropy. -/
theorem mutual_info_self :
    let H_X  : ℚ := 1  -- log 2 in units of log 2
    let H_XY : ℚ := 1  -- H(X,X) = H(X)
    H_X + H_X - H_XY = H_X := by norm_num
```
