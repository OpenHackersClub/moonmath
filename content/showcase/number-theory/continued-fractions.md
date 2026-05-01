+++
title = "Continued Fractions"
description = "Every rational number has a finite continued fraction expansion; irrationals have infinite periodic ones"
weight = 200
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "x = a_0 + \\cfrac{1}{a_1 + \\cfrac{1}{a_2 + \\cdots}}"
prerequisites = ["euclidean-algorithm"]
lean4_status = "complete"
+++

## Statement

A **continued fraction** (simple, regular) is an expression

$$x = a_0 + \cfrac{1}{a_1 + \cfrac{1}{a_2 + \cfrac{1}{a_3 + \cdots}}}$$

written $[a_0; a_1, a_2, \ldots]$, where each $a_i$ is a non-negative integer with $a_i \ge 1$ for $i \ge 1$.

**Theorem (rationality criterion).** $x$ has a finite continued fraction $[a_0; a_1, \ldots, a_n]$ if and only if $x \in \mathbb{Q}$. The algorithm is exactly the Euclidean algorithm applied to the numerator and denominator.

In Mathlib: `GenContFract.terminates_of_rat` states that `(GenContFract.of q).Terminates` for any `q : ℚ`.

## Visualization

**Continued fraction of $\sqrt{2} = [1; 2, 2, 2, \ldots]$ — convergents table:**

The $n$-th convergent $p_n/q_n$ is the rational approximation $[1; 2, 2, \ldots, 2]$ ($n$ twos after the semicolon):

| $n$ | $a_n$ | $p_n$ | $q_n$ | $p_n/q_n$  | Error $|p_n/q_n - \sqrt{2}|$ |
|-----|-------|-------|-------|------------|-------------------------------|
| 0   | 1     | 1     | 1     | 1          | 0.414...                      |
| 1   | 2     | 3     | 2     | 1.5        | 0.086...                      |
| 2   | 2     | 7     | 5     | 1.4        | 0.014...                      |
| 3   | 2     | 17    | 12    | 1.4167...  | 0.0024...                     |
| 4   | 2     | 41    | 29    | 1.4138...  | 0.00042...                    |

Recurrence: $p_n = a_n p_{n-1} + p_{n-2}$, $q_n = a_n q_{n-1} + q_{n-2}$.

These are the best rational approximations to $\sqrt{2}$: no fraction $p/q$ with $q \le q_n$ is closer than $p_n/q_n$.

## Proof Sketch

1. Apply the Euclidean algorithm to $p$ and $q$ (assuming $x = p/q$ in lowest terms). Each step $p = a_0 q + r$ gives one partial quotient $a_0 = \lfloor p/q \rfloor$.
2. Replace $x$ by $q/r$ (the reciprocal of the remainder) and repeat. Since $r < q$, the algorithm terminates in finitely many steps by [[Euclidean Algorithm]].
3. The resulting sequence $[a_0; a_1, \ldots, a_n]$ converges to $p/q$.
4. Conversely, any finite $[a_0; \ldots, a_n]$ is rational by induction.

## Connections

The continued fraction algorithm is the [[Euclidean Algorithm]] in disguise. The convergents $p_n/q_n$ satisfy $|p_n/q_n - x| < 1/q_n^2$, which is the content of the [[Cauchy Criterion]] for Diophantine approximation.

## Lean4 Proof

```lean4
/-- The continued fraction of any rational number terminates.
    Direct alias of `GenContFract.terminates_of_rat`. -/
theorem cf_rat_terminates (q : ℚ) : (GenContFract.of q).Terminates :=
  GenContFract.terminates_of_rat q
```
