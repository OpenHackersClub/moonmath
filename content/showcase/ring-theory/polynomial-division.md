+++
title = "Polynomial Division Algorithm"
description = "For any polynomial f and monic g, there exist unique q and r with f = qg + r and deg r < deg g."
weight = 128
tags = ["lean4-proof", "ring-theory", "visualization"]
latex = "f = q \\cdot g + r, \\quad \\deg r < \\deg g"
prerequisites = ["euclidean-domain"]
lean4_status = "complete"
+++

## Statement

Let $R$ be a commutative ring and $g \in R[x]$ a **monic** polynomial (leading coefficient $1$) of degree $d \ge 1$. For every $f \in R[x]$ there exist unique $q, r \in R[x]$ with

$$f = q \cdot g + r \quad \text{and} \quad \deg r < d \text{ (or } r = 0\text{)}.$$

More generally, if the leading coefficient of $g$ is a unit in $R$, the same division exists. Over a field, any non-zero $g$ suffices.

## Visualization

Divide $f(x) = x^3 - 2x + 1$ by $g(x) = x - 1$ (monic, degree 1).

```
         x^2 + x - 1
        _______________
x - 1 | x^3 + 0x^2 - 2x + 1
         x^3 -  x^2
         -----------
               x^2 - 2x
               x^2 -  x
               ---------
                    -x + 1
                    -x + 1
                    ------
                         0
```

Result: $x^3 - 2x + 1 = (x^2 + x - 1)(x - 1) + 0$.

Verification: $(x^2 + x - 1)(x - 1) = x^3 - x^2 + x^2 - x - x + 1 = x^3 - 2x + 1$. Quotient $q = x^2 + x - 1$, remainder $r = 0$.

**Non-zero remainder example.** Divide $f = x^4 + 1$ by $g = x^2 + 1$:

| Step | Dividend | Multiplier | Subtract |
|------|----------|------------|----------|
| 1 | $x^4 + 1$ | $x^2 \cdot (x^2+1)$ | $\to x^4 + x^2$ | remainder $-x^2 + 1$ |
| 2 | $-x^2 + 1$ | $-1 \cdot (x^2+1)$ | $\to -x^2 - 1$ | remainder $2$ |

So $x^4 + 1 = (x^2 - 1)(x^2 + 1) + 2$. Here $r = 2$, $\deg r = 0 < 2 = \deg g$.

## Proof Sketch

1. **Existence by induction on $\deg f$.** If $\deg f < \deg g$, take $q = 0$, $r = f$. Otherwise, let $f = a x^n + \ldots$ with $n \ge d$ and subtract $a x^{n-d} g$ to reduce degree: $f - a x^{n-d} g$ has degree $< n$. Induct.

2. **Monic hypothesis is used.** When $g$ is monic, $a x^{n-d} g$ has the same leading term as $a x^n$, so subtracting strictly reduces degree. If $g$ were not monic (or its leading coeff not a unit), this step might fail over a general ring.

3. **Uniqueness.** If $f = qg + r = q'g + r'$, then $(q - q')g = r' - r$. If $r \ne r'$, then $\deg((q-q')g) = \deg(q-q') + \deg g \ge \deg g > \deg r \ge \deg(r'-r)$, a contradiction. So $r = r'$, hence $q = q'$.

## Connections

Polynomial division is the analogue for $k[x]$ of the division algorithm in $\mathbb{Z}$ that powers the [[Euclidean Algorithm]]. It makes $k[x]$ a Euclidean domain, explaining why $k[x]$ is a PID (see [[Euclidean Domain]]). The remainder theorem — $f(a) = 0$ iff $(x-a) \mid f$ — is an immediate corollary.

## Lean4 Proof

```lean4
-- Mathlib: Polynomial.modByMonic_add_div
-- in Mathlib.Algebra.Polynomial.Div (line 259)
-- States: p %ₘ q + q * (p /ₘ q) = p  for monic q

open Polynomial in
/-- Division algorithm: for monic g, we have r + g * q = f
    where r = f %ₘ g and q = f /ₘ g. -/
theorem poly_division_algorithm {R : Type*} [CommRing R] (f g : R[X]) (hg : g.Monic) :
    f %ₘ g + g * (f /ₘ g) = f :=
  Polynomial.modByMonic_add_div f hg

/-- The degree of the remainder is less than the degree of the divisor. -/
theorem poly_remainder_degree {R : Type*} [CommRing R] [Nontrivial R]
    (f g : R[X]) (hg : g.Monic) :
    (f %ₘ g).natDegree < g.natDegree :=
  Polynomial.natDegree_modByMonic_lt f hg (Polynomial.Monic.ne_zero hg)
```
