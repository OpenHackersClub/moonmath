+++
title = "Separable Extension"
description = "A field extension where every element's minimal polynomial has no repeated roots"
weight = 128
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "\\gcd(f, f') = 1 \\implies f \\text{ separable}"
prerequisites = ["splitting-field"]
lean4_status = "complete"
+++

## Statement

A polynomial $f \in F[x]$ is **separable** if it has no repeated roots in any extension of $F$, equivalently if $\gcd(f, f') = 1$ where $f'$ is the formal derivative. An algebraic element $\alpha$ over $F$ is separable if its minimal polynomial $\min_F(\alpha)$ is separable. An extension $K/F$ is separable if every $\alpha \in K$ is separable over $F$.

In Mathlib:
- `Polynomial.Separable f` — the polynomial predicate
- `IsSeparable F x` — the element predicate (`minpoly F x` is separable)
- `Algebra.IsSeparable F K` — the extension predicate

**Key fact:** Every algebraic extension in characteristic 0 is separable. Specifically, if $F$ has characteristic 0 and $K/F$ is algebraic and integral, then `Algebra.IsSeparable F K` holds automatically.

## Visualization

**Separable example** — $x^2 - 2 \in \mathbb{Q}[x]$:

$$f = x^2 - 2, \quad f' = 2x$$
$$\gcd(x^2 - 2,\; 2x) = 1 \quad (\text{no common root})$$

Roots $\pm\sqrt{2}$ are distinct. The extension $\mathbb{Q}(\sqrt{2})/\mathbb{Q}$ is separable.

**Inseparable example** — $x^p - t \in \mathbb{F}_p(t)[x]$ (char $p$ field):

$$f = x^p - t, \quad f' = p x^{p-1} = 0 \quad (\text{since char} = p)$$

So $\gcd(f, f') = f \ne 1$. The unique root $\alpha = t^{1/p}$ has multiplicity $p$:

$$x^p - t = (x - \alpha)^p \in \overline{\mathbb{F}_p(t)}[x]$$

This single root is repeated $p$ times — the extension is purely inseparable.

| Polynomial | Field | $f'$ | $\gcd(f,f')$ | Separable? |
|---|---|---|---|---|
| $x^2 - 2$ | $\mathbb{Q}$ | $2x$ | $1$ | Yes |
| $x^2 + 1$ | $\mathbb{Q}$ | $2x$ | $1$ | Yes |
| $x^p - t$ | $\mathbb{F}_p(t)$ | $0$ | $f$ | No |
| $x^p - x$ | $\mathbb{F}_p$ | $-1$ | $1$ | Yes |

## Proof Sketch

1. **Char 0 implies separable.** In characteristic 0 the derivative of a non-constant polynomial is non-zero and has strictly smaller degree. An irreducible $f$ and $f'$ cannot share a factor (since $f$ is irreducible and $\deg f' < \deg f$), so $\gcd(f, f') = 1$.
2. **Char $p$ inseparability.** If $f'= 0$ in char $p$, then $f = g(x^p)$ for some $g$. Expanding $g(x^p) = g(x)^p$ (by the Frobenius endomorphism) shows every root has multiplicity divisible by $p$.
3. **Extension perspective.** $K/F$ is separable iff the number of $F$-embeddings $K \to \overline{F}$ equals $[K:F]$, i.e., distinct embeddings correspond to distinct roots.

## Connections

Separability is one of the two conditions (the other being normality) for a [[Normal Extension]] to be Galois. The Primitive Element Theorem ([[Primitive Element Theorem]]) holds precisely for finite separable extensions.

## Lean4 Proof

```lean4
/-- In characteristic 0 every irreducible polynomial is separable.
    We derive: any integral algebraic extension of a char-0 field is separable. -/
theorem integral_charZero_isSeparable
    (F K : Type*) [Field F] [CharZero F] [Field K] [Algebra F K]
    [Algebra.IsIntegral F K] : Algebra.IsSeparable F K :=
  Algebra.IsSeparable.of_integral
```
