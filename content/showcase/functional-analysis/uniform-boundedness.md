+++
title = "Uniform Boundedness Principle"
description = "A pointwise-bounded family of bounded linear operators on a Banach space is uniformly bounded in operator norm"
weight = 40
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "\\sup_n \\|T_n x\\| < \\infty\\;(\\forall x) \\Rightarrow \\sup_n \\|T_n\\| < \\infty"
prerequisites = ["closed-graph"]
lean4_status = "complete"
+++

## Statement

(Banach–Steinhaus Theorem.) Let $E$ be a **Banach space**, $F$ a normed space, and $\{T_\iota\}_{\iota \in I}$ a family of bounded linear operators $T_\iota : E \to F$. If

$$\sup_{\iota \in I} \|T_\iota x\| < \infty \quad \text{for every } x \in E,$$

then the operator norms are uniformly bounded:

$$\sup_{\iota \in I} \|T_\iota\| < \infty.$$

Pointwise boundedness (a condition on each vector $x$) automatically upgrades to a *uniform* bound on all operators.

## Visualization

**Counterexample anatomy** — why completeness is essential. On $c_{00}$ (finitely supported sequences, incomplete), define $T_n(x) = n x_n$:

```
x = (x₁, x₂, x₃, ...) ∈ c₀₀

T_n(x) = n · x_n

Pointwise: for each fixed x, only finitely many x_n ≠ 0,
           so sup_n |T_n(x)| = sup_n n|x_n| < ∞.

But ‖T_n‖ = n → ∞.   (Not uniformly bounded!)
```

On a **complete** space this cannot happen. Numerical trace on $\ell^2$ with a *convergent* family:

| $n$ | $T_n$ (truncation to first $n$ coords) | $\|T_n x\|$ for $x = (1/k^2)$ | $\|T_n\|$ |
|---|---|---|---|
| 1 | $(x_1, 0, 0, \ldots)$ | $1$ | $1$ |
| 2 | $(x_1, x_2, 0, \ldots)$ | $\sqrt{1 + 1/16}$ | $1$ |
| 5 | projection to 5 coords | $\sqrt{\sum_{k=1}^5 k^{-4}}$ | $1$ |
| $\infty$ | identity | $\pi^2/6$ | $1$ |

Here $\sup_n \|T_n\| = 1 < \infty$, consistent with UBP. The principle would fire if any $\sup_n \|T_n x\|$ were infinite — on a Banach space that cannot happen pointwise.

## Proof Sketch

1. **Define sets.** For each $M > 0$ let $A_M = \{x \in E : \sup_\iota \|T_\iota x\| \le M\}$.
2. **Closed sets.** Each $A_M$ is closed (intersection of closed sets $\{x : \|T_\iota x\| \le M\}$).
3. **Baire's theorem.** Since $E = \bigcup_{M \in \mathbb{N}} A_M$ and $E$ is complete, some $A_M$ has non-empty interior: $B_r(x_0) \subseteq A_M$.
4. **Symmetry trick.** For any $y \in B_r(0)$ and any $\iota$:
   $$\|T_\iota y\| \le \|T_\iota(y + x_0)\| + \|T_\iota x_0\| \le M + M = 2M.$$
5. **Scale.** Any $x$ with $\|x\| \le 1$ can be rescaled into $B_r(0)$, giving $\|T_\iota x\| \le 2M/r$ for all $\iota$.

## Connections

- [[Closed Graph Theorem]] — both theorems use Baire's category theorem on Banach spaces; UBP uses it for a family, Closed Graph for a single operator.
- [[Cauchy–Schwarz Inequality]] — in Hilbert spaces, uniform boundedness combines with Cauchy–Schwarz to bound sesquilinear forms.
- [[Spectral Radius Formula]] — operator norms bounded by the spectral radius formula (Gelfand) give a quantitative form of UBP for Banach algebras.
- [[Monotone Convergence Theorem]] — in measure theory, both MCT and UBP serve as "upgrade" theorems: pointwise properties become uniform ones.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Operator.BanachSteinhaus

/-- **Banach–Steinhaus / Uniform Boundedness Principle**:
    a pointwise-bounded family of continuous linear maps on a Banach space
    has uniformly bounded norms. Direct alias of `banach_steinhaus`. -/
theorem uniform_boundedness_principle {ι : Type*} {E F : Type*}
    [SeminormedAddCommGroup E] [NormedSpace ℝ E] [CompleteSpace E]
    [SeminormedAddCommGroup F] [NormedSpace ℝ F]
    (g : ι → E →L[ℝ] F)
    (h : ∀ x, BddAbove (Set.range fun i => ‖g i x‖)) :
    BddAbove (Set.range fun i => ‖g i‖) :=
  banach_steinhaus h
```
