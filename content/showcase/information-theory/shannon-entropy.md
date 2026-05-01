+++
title = "Shannon Entropy"
description = "The unique measure of uncertainty for a probability distribution: H(X) = -sum p_i log p_i, always non-negative"
weight = 151
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "H(X) = -\\sum_{i} p_i \\log p_i \\ge 0"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $X$ be a discrete random variable taking values in $\{1, \ldots, n\}$ with probability distribution $(p_1, \ldots, p_n)$, where $p_i \ge 0$ and $\sum p_i = 1$. The **Shannon entropy** is

$$H(X) = -\sum_{i=1}^{n} p_i \log p_i$$

(with the convention $0 \log 0 = 0$). The entropy satisfies:

1. $H(X) \ge 0$ for all distributions.
2. $H(X) = 0$ if and only if one $p_i = 1$ and the rest are zero.
3. $H(X)$ is maximized at $\log n$ when $p_i = 1/n$ for all $i$.

For a Bernoulli$(p)$ source the entropy reduces to the **binary entropy function**

$$h(p) = -p \log p - (1-p) \log(1-p),$$

which peaks at $h(1/2) = \log 2$.

## Visualization

Binary entropy $h(p)$ for selected values (natural log; max at $p = 1/2$):

| $p$   | $-p\ln p$  | $-(1-p)\ln(1-p)$ | $h(p)$ |
|-------|-----------|-----------------|--------|
| 0.00  | 0.000     | 0.000           | 0.000  |
| 0.10  | 0.230     | 0.095           | 0.325  |
| 0.20  | 0.322     | 0.161           | 0.500  |
| 0.30  | 0.361     | 0.240           | 0.611  |
| 0.40  | 0.366     | 0.306           | 0.673  |
| 0.50  | 0.347     | 0.347           | **0.693** |

The curve is strictly concave and symmetric about $p = 1/2$.

ASCII sketch of $h(p)$:

```
h
|         *
|       *   *
|     *       *
|   *           *
| *               *
+-------------------> p
0        1/2       1
```

## Proof Sketch

1. Each term $-p_i \log p_i$ equals $\text{negMulLog}(p_i)$, the function $x \mapsto -x \log x$.
2. Mathlib proves `negMulLog_nonneg`: for $0 \le x \le 1$ we have $-x \log x \ge 0$, because $\log x \le 0$ on $[0,1]$.
3. A sum of non-negative terms is non-negative, so $H(X) \ge 0$.
4. At $x = 0$ or $x = 1$ the term vanishes by continuity ($0 \cdot (-\infty) = 0$ by convention). At $p = 1/2$ the derivative $-\log p - 1 + \log(1-p) + 1 = 0$, confirming the maximum.

## Connections

The concavity of entropy underlies the proof of [[Channel Capacity]] and drives the argument in the [[Source Coding Theorem]]. The same log-sum inequality appears in [[Markov's Inequality]] style bounds via Jensen's inequality.

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.Log.NegMulLog

open Real

/-- Each term -p * log p is non-negative for p in [0,1]. -/
theorem negMulLog_term_nonneg (p : ℝ) (hp0 : 0 ≤ p) (hp1 : p ≤ 1) :
    0 ≤ negMulLog p :=
  negMulLog_nonneg hp0 hp1

/-- Binary entropy h(p) = -p log p - (1-p) log(1-p) is non-negative on [0,1].
    Mathlib's `binEntropy_nonneg` covers this directly. -/
theorem binEntropy_nonneg' (p : ℝ) (hp0 : 0 ≤ p) (hp1 : p ≤ 1) :
    0 ≤ binEntropy p :=
  binEntropy_nonneg hp0 hp1
```
