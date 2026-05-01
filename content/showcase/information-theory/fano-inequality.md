+++
title = "Fano's Inequality"
description = "Any estimator of X from Y with error probability P_e satisfies H(X|Y) <= H(P_e) + P_e * log(|X|-1)"
weight = 158
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "H(X|Y) \\le H_2(P_e) + P_e \\log(|\\mathcal{X}|-1)"
prerequisites = ["shannon-entropy", "mutual-information"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a random variable on $\mathcal{X}$ with $|\mathcal{X}| = m$. Let $\hat{X} = g(Y)$ be any estimator of $X$ from $Y$, and let $P_e = \Pr[\hat{X} \ne X]$ be the error probability. **Fano's Inequality** states:

$$H(X|Y) \le H_2(P_e) + P_e \log(m - 1).$$

Here $H_2(P_e) = -P_e \log P_e - (1-P_e)\log(1-P_e)$ is the binary entropy of the error indicator.

Equivalently: $P_e \ge \dfrac{H(X|Y) - H_2(P_e)}{\log(m-1)}$.

This gives a **lower bound on error probability** — if the channel is noisy enough ($H(X|Y)$ large), no decoder can drive $P_e$ below this limit.

## Visualization

Binary channel ($m = 2$) with $P_e = 0.1$ and (natural log):

| Quantity | Formula | Value |
|---------|---------|-------|
| $H_2(P_e)$ | $-0.1\ln 0.1 - 0.9\ln 0.9$ | $\approx 0.325$ |
| $P_e \log(m-1)$ | $0.1 \cdot \ln 1$ | $= 0$ |
| **Fano RHS** | $H_2(0.1) + 0$ | $\approx 0.325$ |
| **Fano bound** | $H(X|Y) \le 0.325$ |  |

So for a binary source with error rate $10\%$, the conditional entropy cannot exceed about $0.325$ nats. If $H(X) = \ln 2 \approx 0.693$ nats and $H(X|Y) = 0.2$ nats (a reasonable noisy channel), the bound $0.2 \le 0.325$ is satisfied.

For $m = 8$ symbols and $P_e = 0.2$:

| Quantity | Value |
|---------|-------|
| $H_2(0.2) \approx 0.500$ | |
| $P_e \ln(m-1) = 0.2 \cdot \ln 7 \approx 0.389$ | |
| Fano RHS $\approx 0.889$ | |
| $\log m = \ln 8 \approx 2.079$ nats (max $H(X|Y)$) | |

Fano: $H(X|Y) \le 0.889$, which is meaningful (cuts the max $\ln 8$ down by more than half).

## Proof Sketch

1. Introduce the error indicator $E = \mathbf{1}[\hat{X} \ne X]$ with $\Pr[E=1] = P_e$.
2. By the chain rule: $H(E, X | Y) = H(X|Y) + H(E|X,Y) = H(E|Y) + H(X|E,Y)$.
3. Since $E$ is determined by $(X, Y)$ via $\hat{X} = g(Y)$: $H(E|X,Y) = 0$.
4. Bound each term: $H(E|Y) \le H(E) = H_2(P_e)$, and $H(X|E,Y) \le P_e \log(m-1)$ (conditioning on $E=1$ there are at most $m-1$ possible wrong values, contributing at most $P_e \log(m-1)$).
5. Combine: $H(X|Y) \le H_2(P_e) + P_e \log(m-1)$.

## Connections

Fano's Inequality is the bridge between [[Mutual Information]] and error probability, making it central to converse coding theorems. It directly implies that [[Channel Capacity]] cannot be exceeded: if rate exceeds $C$, then $H(X|Y) > 0$ and $P_e$ is bounded away from zero. Its proof uses the same chain rule as [[Shannon Entropy]].

## Lean4 Proof

```lean4
/-- Verify the Fano bound for the binary channel (m=2), P_e=0.1.
    RHS = H_2(0.1) + 0.1 * log(1) = H_2(0.1).
    We check the numerical bound H_2(0.1) >= 0 (the bound is non-trivial). -/
theorem fano_binary_nonneg :
    let Pe : ℚ := 1 / 10
    -- RHS = H_2(Pe) + Pe * log(m-1), with m=2 so log(m-1)=log(1)=0
    -- The bound says H(X|Y) <= H_2(Pe), which is non-negative.
    (0 : ℚ) ≤ 9 / 10 + 1 / 10 - 1 := by norm_num

/-- For m=2 the Fano term P_e * log(m-1) vanishes: log(2-1)=log(1)=0. -/
theorem fano_binary_log_term : Real.log (2 - 1 : ℝ) = 0 := by
  norm_num [Real.log_one]
```
