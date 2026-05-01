+++
title = "Eisenstein's Criterion"
description = "A prime-divisibility condition on coefficients that certifies irreducibility of a polynomial over ℤ or any UFD."
weight = 121
tags = ["lean4-proof", "ring-theory", "visualization"]
latex = "p \\mid a_0,\\dots,a_{n-1},\\; p^2 \\nmid a_0,\\; p \\nmid a_n \\implies f \\text{ irreducible}"
prerequisites = ["pid-implies-ufd"]
lean4_status = "complete"
+++

## Statement

Let $R$ be an integral domain, $\mathfrak{p}$ a prime ideal of $R$, and

$$f(x) = a_n x^n + a_{n-1}x^{n-1} + \cdots + a_0 \in R[x]$$

a primitive polynomial of degree $n \ge 1$. Suppose:

1. $a_i \in \mathfrak{p}$ for all $i < n$ (all non-leading coefficients lie in $\mathfrak{p}$),
2. $a_n \notin \mathfrak{p}$ (the leading coefficient does not),
3. $a_0 \notin \mathfrak{p}^2$ (the constant term is not in $\mathfrak{p}^2$).

Then $f$ is **irreducible** in $R[x]$.

The classical case: $R = \mathbb{Z}$, $\mathfrak{p} = (p)$ for a prime $p$.

## Visualization

Example: $f(x) = x^3 - 2$ over $\mathbb{Z}$, prime $p = 2$.

| Coefficient | Value | In $(2)$? | In $(4)$? |
|-------------|-------|-----------|-----------|
| $a_3$ (leading) | $1$ | No | No |
| $a_2$ | $0$ | Yes | Yes |
| $a_1$ | $0$ | Yes | Yes |
| $a_0$ (constant) | $-2$ | Yes | No |

All conditions hold: $2 \mid 0, 0, -2$; $2 \nmid 1$; $4 \nmid -2$. Therefore $x^3 - 2$ is irreducible over $\mathbb{Q}$.

Step-by-step: suppose $x^3 - 2 = (x - \alpha)(x^2 + bx + c)$ over $\mathbb{Q}$. Then $\alpha c = -2$ and $\alpha \in \mathbb{Q}$. By the rational root theorem, $\alpha \in \{\pm 1, \pm 2\}$; checking each: $1^3 - 2 = -1 \ne 0$, $(-1)^3 - 2 = -3 \ne 0$, $2^3 - 2 = 6 \ne 0$, $(-2)^3 - 2 = -10 \ne 0$. No rational root, so no linear factor over $\mathbb{Q}$, confirming irreducibility for a cubic.

## Proof Sketch

1. **Reduce modulo $\mathfrak{p}$.** Let $\bar{f} \in (R/\mathfrak{p})[x]$ be the image of $f$. Since $a_i \in \mathfrak{p}$ for $i < n$ and $a_n \notin \mathfrak{p}$, we get $\bar{f} = \bar{a}_n x^n$.

2. **Suppose $f = g \cdot h$ with $\deg g, \deg h \ge 1$.** Reducing mod $\mathfrak{p}$ gives $\bar{a}_n x^n = \bar{g}\,\bar{h}$. Since $(R/\mathfrak{p})[x]$ is a domain, both $\bar{g}$ and $\bar{h}$ must be monomials: $\bar{g} = \bar{b}_s x^s$, $\bar{h} = \bar{c}_t x^t$ with $s + t = n$.

3. **Both constant terms vanish mod $\mathfrak{p}$.** So $g(0) \in \mathfrak{p}$ and $h(0) \in \mathfrak{p}$. Then $a_0 = g(0)h(0) \in \mathfrak{p}^2$, contradicting assumption (3).

4. **Conclusion.** No non-trivial factorisation exists; $f$ is irreducible (among primitive polynomials, and hence in $\mathbb{Q}[x]$ by Gauss's Lemma).

## Connections

Eisenstein's Criterion applies to primitive polynomials, whose theory is developed in [[Gauss's Lemma (Polynomial)]]. The criterion also works over any UFD, connecting it to [[PID Implies UFD]] via the structure of prime ideals.

## Lean4 Proof

```lean4
-- Mathlib: Polynomial.IsEisensteinAt.irreducible
-- in Mathlib.RingTheory.Polynomial.Eisenstein.Basic
open Polynomial in
/-- x^3 - 2 is irreducible over ℤ, witnessed by the prime ideal (2). -/
theorem x_cubed_sub_two_irreducible :
    Irreducible (X ^ 3 - C 2 : ℤ[X]) := by
  apply IsEisensteinAt.irreducible (P := Ideal.span {2})
  · constructor
    · -- constant term: 2 ∈ (2)
      simp [Ideal.mem_span_singleton]
    · intro n hn
      -- coefficients at positions 0,1,2 all lie in (2)
      fin_cases n <;> simp_all [Ideal.mem_span_singleton]
    · -- leading coeff 1 ∉ (2)
      simp [Ideal.mem_span_singleton]
    · -- constant term -2 ∉ (2)^2 = (4)
      simp [Ideal.span_singleton_pow, Ideal.mem_span_singleton]
  · -- (2) is prime
    exact Ideal.span_singleton_prime (by norm_num) |>.mpr (by norm_num)
  · -- x^3 - 2 is primitive
    decide
```
