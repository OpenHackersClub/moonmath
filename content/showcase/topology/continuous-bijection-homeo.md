+++
title = "Continuous Bijection on Compact-Hausdorff is Homeo"
description = "A continuous bijection from a compact space to a Hausdorff space is automatically a homeomorphism"
weight = 210
tags = ["lean4-proof", "topology", "visualization"]
latex = "f: X\\xrightarrow{\\sim} Y\\text{ continuous bijection},\\;X\\text{ compact},\\;Y\\text{ Hausdorff}\\Rightarrow f\\text{ homeomorphism}"
prerequisites = ["heine-borel", "one-point-compactification"]
lean4_status = "complete"
+++

## Statement

Let $f: X \to Y$ be a continuous bijection. If $X$ is compact and $Y$ is Hausdorff ($T_2$), then $f$ is a **homeomorphism** (i.e., $f^{-1}$ is also continuous):

$$f \text{ continuous bijection},\quad X \text{ compact},\quad Y \text{ Hausdorff} \;\Longrightarrow\; f \text{ homeomorphism.}$$

The theorem fails if either hypothesis is dropped: $T_1$ alone does not suffice (see the counterexample with the indiscrete topology), and non-compact domains allow continuous bijections with discontinuous inverses.

## Visualization

Canonical counterexample — why compactness of the **domain** is needed:

```
f : [0, 2π) → S¹,   f(t) = (cos t, sin t)

  [0, 2π):                      S¹ (unit circle):
  ────────────────────────)      ╭───────────────────╮
  0                     2π      │          (1,0)=f(0)│
                                │      ╭────────╮    │
  f is continuous: ✓             │    f│→       │    │
  f is bijective:  ✓             │      ╰────────╯    │
                                │ 2π⁻ maps to (1,0)  │
  f⁻¹ is NOT continuous: ✗       ╰───────────────────╯
    Near (1,0) on S¹, the preimage
    has points near 0 AND near 2π — two disjoint ends.

Domain [0, 2π) is NOT compact → theorem does not apply.

Compact domain fix: use [0, 2π] / ∼ (quotient identifying endpoints)
  Then domain is compact, and f descends to a homeomorphism S¹ ≅ S¹.
```

Key idea — closed maps:

```
Compact  ──continuous──▶  Hausdorff
  X                           Y

  A ⊆ X closed  ─compact──▶  f(A) compact  ─Hausdorff──▶  f(A) closed

  So f is a closed map ⟹ f⁻¹ is continuous ⟹ f is a homeomorphism.
```

## Proof Sketch

1. **Closed maps:** Show $f$ is a closed map (images of closed sets are closed in $Y$).
2. **Closed $\subseteq$ compact:** In a compact space, every closed set is compact (a closed subset of a compact space is compact).
3. **Continuous image of compact is compact:** $f$ continuous and $A \subseteq X$ compact implies $f(A)$ compact.
4. **Compact in Hausdorff is closed:** In a Hausdorff space, every compact set is closed.
5. **Chain:** $A$ closed in $X$ $\Rightarrow$ $A$ compact $\Rightarrow$ $f(A)$ compact $\Rightarrow$ $f(A)$ closed in $Y$.
6. **Closed map $\Rightarrow$ homeomorphism:** A continuous bijection that is also closed is a homeomorphism (the inverse is continuous because preimages of closed sets under $f^{-1}$ are images of closed sets under $f$, hence closed).

## Connections

- [[Heine–Borel Theorem]] — the theorem that closed bounded sets in $\mathbb{R}^n$ are compact is exactly what makes many bijections $[0,1] \to S^1$-type maps into homeomorphisms when the domain is a closed bounded interval.
- [[Alexandrov One-Point Compactification]] — the homeomorphism $\text{OnePoint}(\mathbb{R}) \cong S^1$ is an instance: stereographic projection is a continuous bijection from the compact one-point compactification to the Hausdorff circle.
- [[Tychonoff's Theorem]] — Tychonoff's theorem produces compact product spaces; when a continuous bijection maps such a product to a Hausdorff space, this theorem applies automatically.
- [[Brouwer Fixed-Point Theorem]] — Brouwer's theorem relies on the fact that $D^n$ and $S^{n-1}$ are compact Hausdorff, and continuous maps from them behave rigidly; this theorem underlies the no-retraction lemma used in the proof.

## Lean4 Proof

```lean4
import Mathlib.Topology.Homeomorph.Lemmas

/-- **Continuous bijection from compact to T2 is a homeomorphism**.
    Mathlib: `Continuous.homeoOfEquivCompactToT2`. -/
theorem continuous_bij_compact_t2_homeo
    {X Y : Type*} [TopologicalSpace X] [TopologicalSpace Y]
    [CompactSpace X] [T2Space Y]
    (f : X ≃ Y) (hf : Continuous f) : X ≃ₜ Y :=
  Continuous.homeoOfEquivCompactToT2 hf
```
