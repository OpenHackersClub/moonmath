+++
title = "Spectral Radius Formula"
description = "The spectral radius equals the limit of the nth root of the operator norm of the nth power (Gelfand's formula)"
weight = 80
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "r(a) = \\lim_{n\\to\\infty} \\|a^n\\|^{1/n}"
prerequisites = ["uniform-boundedness"]
lean4_status = "complete"
+++

## Statement

Let $A$ be a Banach algebra and $a \in A$. The **spectral radius** of $a$ is

$$r(a) = \sup \{|\lambda| : \lambda \in \sigma(a)\},$$

where $\sigma(a) = \{\lambda \in \mathbb{C} : \lambda \cdot 1 - a \text{ is not invertible}\}$ is the spectrum. **Gelfand's formula** states:

$$r(a) = \lim_{n \to \infty} \|a^n\|^{1/n}.$$

The limit always exists, and equals the infimum $\inf_n \|a^n\|^{1/n}$ (submultiplicativity).

## Visualization

Take $A = M_2(\mathbb{C})$ and $a = \begin{pmatrix} 0 & 2 \\ 0 & 0 \end{pmatrix}$ (nilpotent: $a^2 = 0$).

Spectrum: $\sigma(a) = \{0\}$, so $r(a) = 0$.

| $n$ | $a^n$ | $\|a^n\|$ | $\|a^n\|^{1/n}$ |
|---|---|---|---|
| 1 | $\begin{pmatrix}0&2\\0&0\end{pmatrix}$ | $2$ | $2.000$ |
| 2 | $0$ (zero matrix) | $0$ | $0.000$ |
| 3 | $0$ | $0$ | $0.000$ |
| 4 | $0$ | $0$ | $0.000$ |

The limit is $0 = r(a)$. For $n \ge 2$ all norms are $0$ since $a$ is nilpotent.

**Diagonal example:** $b = \begin{pmatrix} 2 & 0 \\ 0 & 3 \end{pmatrix}$, eigenvalues $2, 3$, $r(b) = 3$.

| $n$ | $\|b^n\| = 3^n$ | $\|b^n\|^{1/n}$ |
|---|---|---|
| 1 | $3$ | $3.000$ |
| 2 | $9$ | $3.000$ |
| 5 | $243$ | $3.000$ |

For normal matrices the formula gives the spectral radius immediately since $\|b^n\| = r(b)^n$.

```
r(a) = lim ||a^n||^{1/n}

         ||a^n||^{1/n}
3.0 ─────────────────────────  (diagonal b)
         (constant sequence)

2.0 ─  ↓
0.0 ─────────────────────────  (nilpotent a, n ≥ 2)
         1   2   3   4   n →
```

## Proof Sketch

1. **Upper bound.** $\|a^n\|^{1/n} \le \|a\|$ for all $n$ (submultiplicativity), so $\limsup \le \|a\|$. In fact $\limsup_n \|a^n\|^{1/n} \le r(a)$ by analyzing the resolvent on $|\lambda| > r(a)$.
2. **Resolvent.** For $|\lambda| > r(a)$, the Neumann series $(\lambda - a)^{-1} = \sum_{n=0}^\infty a^n / \lambda^{n+1}$ converges, and this series fails to converge for $|\lambda| < r(a)$.
3. **Complex analysis.** The resolvent $\lambda \mapsto (\lambda - a)^{-1}$ is analytic for $|\lambda| > r(a)$ and its Laurent series radius is exactly $r(a)$. Cauchy–Hadamard gives $r(a) = \limsup \|a^n\|^{1/n}$.
4. **Limit exists.** By Fekete's lemma applied to $\log \|a^n\|$ (subadditive), $\lim_{n} \|a^n\|^{1/n} = \inf_n \|a^n\|^{1/n}$.

## Connections

- [[Uniform Boundedness Principle]] — boundedness of the resolvent family near the spectrum uses UBP to transfer local bounds to global ones.
- [[Cayley–Hamilton Theorem]] — for finite matrices, Cayley–Hamilton gives $p(a) = 0$, which implies that $\|a^n\|^{1/n}$ stabilises to the spectral radius $\rho(A)$.
- [[Geometric Series]] — the Neumann series $\sum a^n \lambda^{-n-1}$ is the operator-valued analogue of the geometric series, converging when $\|a\| < |\lambda|$.
- [[Taylor's Theorem]] — the power series of the resolvent around a regular point is the operator-algebraic Taylor expansion, with radius of convergence $= $ distance to the nearest spectrum point.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Algebra.Spectrum
import Mathlib.Analysis.Normed.Algebra.GelfandFormula

open spectrum

/-- **Gelfand's spectral radius formula**: the spectral radius is the limit of ‖aⁿ‖^(1/n).
    Uses `spectrum.pow_nnnorm_pow_one_div_tendsto_nhds_spectralRadius`. -/
theorem gelfand_spectral_radius_formula {A : Type*}
    [NormedRing A] [NormedAlgebra ℂ A] [CompleteSpace A] (a : A) :
    Filter.Tendsto (fun n : ℕ => ((‖a ^ n‖₊ : ℝ≥0) ^ (1 / (n : ℝ) : ℝ) : ℝ≥0∞))
      Filter.atTop (nhds (spectralRadius ℂ a)) :=
  pow_nnnorm_pow_one_div_tendsto_nhds_spectralRadius a
```
