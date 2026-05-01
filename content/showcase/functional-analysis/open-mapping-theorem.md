+++
title = "Open Mapping Theorem (Banach)"
description = "A surjective bounded linear map between Banach spaces maps open sets to open sets"
weight = 20
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "T : E \\twoheadrightarrow F \\text{ bounded, surjective} \\Rightarrow T \\text{ is an open map}"
prerequisites = ["hahn-banach"]
lean4_status = "complete"
+++

## Statement

Let $E$ and $F$ be Banach spaces and $T : E \to F$ a bounded (continuous) linear map. If $T$ is surjective, then $T$ is an **open mapping**: for every open set $U \subseteq E$, the image $T(U)$ is open in $F$.

Equivalently, there exists $c > 0$ such that $T(B_1(0)) \supseteq B_c(0)$, i.e., the unit ball maps onto a ball of positive radius in $F$.

## Visualization

Consider the left-shift on $\ell^2$: $T(x_1, x_2, x_3, \ldots) = (x_2, x_3, x_4, \ldots)$.

```
E = ℓ²         T = left-shift (surjective)         F = ℓ²

Open ball B_r in E:   all sequences with ‖x‖ < r

T(B_r) contains B_r because:
  given y = (y₁, y₂, ...) with ‖y‖ < r,
  pick x = (0, y₁, y₂, ...) ∈ B_r  and  Tx = y. ✓

   E: ──( B_r )──────────  T surjective
                 ───▶
   F: ──( B_r )──────────   T(B_r) ⊇ B_r  (open!)
```

| Open set $U \subseteq E$ | Image $T(U) \subseteq F$ | Open? |
|---|---|---|
| $B_1(0)$ | $\supseteq B_c(0)$ for some $c > 0$ | Yes |
| $B_r(x_0)$ | $\supseteq B_{cr}(Tx_0)$ | Yes |
| any open $U$ | $T(U)$ | Yes (open mapping!) |

The Baire category theorem is the engine: $F = \bigcup_n \overline{T(B_n(0))}$ forces one closed set to have non-empty interior.

## Proof Sketch

1. **Baire category.** Since $F$ is complete and $F = \bigcup_n T(\overline{B_n(0)})$, by Baire's theorem some $\overline{T(B_N(0))}$ has non-empty interior.
2. **Interior estimate.** There exists $r > 0$ with $B_r(0) \subseteq \overline{T(B_1(0))}$.
3. **Approximate preimages.** For every $y \in B_r(0)$, find $x_0 \in B_1(0)$ with $\|y - Tx_0\| < r/2$, then a correction $x_1 \in B_{1/2}(0)$, and so on.
4. **Convergence.** The series $x = x_0 + x_1 + \cdots$ converges (completeness of $E$), $\|x\| < 2$, and $Tx = y$. Hence $T(B_2(0)) \supseteq B_r(0)$.
5. **Conclusion.** Scale to any open set.

## Connections

- [[Closed Graph Theorem]] — the Closed Graph Theorem is an immediate corollary: if a linear map has closed graph and the domain is Banach, the Open Mapping Theorem produces continuity.
- [[Hahn–Banach Theorem]] — both use the Baire-category structure of Banach spaces; Hahn–Banach uses algebraic maximality, Open Mapping uses completeness of $F$.
- [[Bolzano–Weierstrass Theorem]] — the sequential compactness used in Baire's theorem is the infinite-dimensional analogue of Bolzano–Weierstrass.
- [[Intermediate Value Theorem]] — open maps in $\mathbb{R}^1$ imply the intermediate value property; the Banach theorem is the linear analogue.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Operator.Banach

/-- **Banach Open Mapping Theorem**: a surjective bounded linear map is an open map.
    Direct alias of `ContinuousLinearMap.isOpenMap` in Mathlib. -/
theorem open_mapping_theorem {E F : Type*}
    [NormedAddCommGroup E] [NormedSpace ℝ E] [CompleteSpace E]
    [NormedAddCommGroup F] [NormedSpace ℝ F] [CompleteSpace F]
    (T : E →L[ℝ] F) (surj : Function.Surjective T) : IsOpenMap T :=
  T.isOpenMap surj
```
