+++
title = "Quotient Topology"
description = "The quotient topology makes the projection continuous and is universal for maps out of the quotient"
weight = 80
tags = ["lean4-proof", "topology", "visualization"]
latex = "g : X/\\!\\sim \\to Z \\text{ continuous} \\iff g \\circ \\pi \\text{ continuous}"
prerequisites = ["subspace-topology"]
lean4_status = "complete"
+++

## Statement

Given a topological space $X$ and an equivalence relation $\sim$ on $X$, the **quotient topology** on $X/{\sim}$ is the finest topology making the projection $\pi : X \to X/{\sim}$ continuous. Explicitly, $U \subseteq X/{\sim}$ is open if and only if $\pi^{-1}(U)$ is open in $X$.

The **universal property** (quotient map criterion): if $\pi : X \to Y$ is a quotient map, then for any topological space $Z$ and any function $g : Y \to Z$,

$$g \text{ is continuous} \iff g \circ \pi \text{ is continuous.}$$

## Visualization

**Construction of the circle $S^1 = \mathbb{R}/\mathbb{Z}$:**

Identify $x \sim x + 1$ for all $x \in \mathbb{R}$. Each equivalence class $[x]$ consists of all translates $x + n$, $n \in \mathbb{Z}$.

```
Real line ℝ:
  ...──0────0.25────0.5────0.75────1────1.25──...
         |           |           |
         └─────────────────────────→ all identified
         ↓ π (projection)
Circle S¹:
         *
      *     *        [0] = [1] = [2] = ... identified
     *       *
      *     *   ← [0.25] sits at "3 o'clock"
         *       [0.5]  sits at "6 o'clock"
                 [0.75] sits at "9 o'clock"
```

**Universal property in action:** A function $g : S^1 \to \mathbb{R}$ is continuous iff $g \circ \pi : \mathbb{R} \to \mathbb{R}$ is continuous AND $g \circ \pi$ is $\mathbb{Z}$-periodic (i.e. respects the identification). The quotient topology encodes exactly this compatibility condition.

**Open sets in the quotient:** Let $U = \pi((0.1, 0.9)) \subseteq S^1$. Then $\pi^{-1}(U) = \bigcup_{n \in \mathbb{Z}} (0.1+n, 0.9+n)$, which is open in $\mathbb{R}$, so $U$ is open in $S^1$.

## Proof Sketch

1. **Definition:** Give $X/{\sim}$ the topology $\tau = \{U : \pi^{-1}(U) \text{ open in } X\}$. Check this is a topology: preimage distributes over unions and intersections, and $\pi^{-1}(\emptyset) = \emptyset$, $\pi^{-1}(X/{\sim}) = X$.

2. **$\pi$ is continuous:** By construction, every open set in $X/{\sim}$ has open preimage.

3. **Universal property ($\Rightarrow$):** If $g$ is continuous, then $g \circ \pi$ is a composition of continuous maps, hence continuous.

4. **Universal property ($\Leftarrow$):** Suppose $g \circ \pi$ is continuous. For open $V \subseteq Z$, we need $g^{-1}(V)$ open in $X/{\sim}$. Compute $\pi^{-1}(g^{-1}(V)) = (g \circ \pi)^{-1}(V)$, which is open in $X$ by continuity of $g \circ \pi$. By definition of the quotient topology, $g^{-1}(V)$ is open.

5. This universal property characterises the quotient topology uniquely.

## Connections

- **[[Tychonoff's Theorem]]** — products and quotients are the two fundamental constructions on topological spaces; Tychonoff shows products of compact spaces are compact, while quotient maps of compact spaces onto Hausdorff spaces are automatically closed maps.
- **[[Urysohn's Lemma]]** — a quotient of a normal space need not be normal, but Urysohn's lemma gives the criterion for when the quotient retains normality.
- **[[Intermediate Value Theorem]]** — the circle $S^1 = \mathbb{R}/\mathbb{Z}$ is connected (quotient of a connected space); the IVT applied to loops on $S^1$ underlies the topological proof of the fundamental theorem of algebra.
- **[[First Isomorphism Theorem]]** — the algebraic analogue: if $\phi : G \to H$ is a group homomorphism, then $G/\ker\phi \cong \mathrm{im}\,\phi$; the topology version replaces group isomorphism with homeomorphism when the maps are open.

## Lean4 Proof

```lean4
import Mathlib.Topology.Maps.Basic

/-- Universal property of the quotient map:
    g is continuous iff g ∘ π is continuous. -/
theorem quotient_universal_property
    {X Y Z : Type*} [TopologicalSpace X] [TopologicalSpace Y] [TopologicalSpace Z]
    {f : X → Y} (hf : IsQuotientMap f) (g : Y → Z) :
    Continuous g ↔ Continuous (g ∘ f) :=
  hf.continuous_iff
```
