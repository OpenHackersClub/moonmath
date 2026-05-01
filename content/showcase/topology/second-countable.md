+++
title = "Second-Countable Spaces"
description = "A topological space is second-countable if its topology has a countable basis"
weight = 160
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\exists\\,\\mathcal{B}\\text{ countable basis for }\\tau:\\;\\forall\\, U\\in\\tau,\\;U = \\bigcup_{B\\in\\mathcal{B},B\\subseteq U} B"
prerequisites = ["separable-spaces"]
lean4_status = "complete"
+++

## Statement

A topological space $(X, \tau)$ is **second-countable** if there exists a countable collection $\mathcal{B}$ of open sets such that every open set is a union of members of $\mathcal{B}$:

$$\exists\, \mathcal{B} \text{ countable},\quad \forall\, U \in \tau,\; U = \bigcup_{\substack{B \in \mathcal{B} \\ B \subseteq U}} B.$$

Every second-countable space is **separable**: picking one point from each basis element yields a countable dense set.

## Visualization

Countable basis for $\mathbb{R}$ using rational endpoints:

```
Standard topology on ℝ — countable basis:

  ℬ = { (p, q) : p, q ∈ ℚ, p < q }

  Each basis element: ──────(p════════q)──────▶

  Example open sets expressed as unions:
  (0, √2) = ⋃ { (p,q) : p,q ∈ ℚ, 0 ≤ p < q ≤ √2 }
           = (0,1) ∪ (0,1.4) ∪ (0,1.41) ∪ (0.1,1.414) ∪ ...

  ℬ is countable: ℚ×ℚ is countable (product of countable sets).

Basis elements for ℝ²:

  ℬ₂ = { (p₁,q₁)×(p₂,q₂) : all pᵢ,qᵢ ∈ ℚ }

        q₂ ┄┄┄┄┄┐
           │ B  │  ← a basis rectangle with rational corners
        p₂ └┄┄┄┄┘
           p₁  q₁
```

Implication chain:

| Property | ℝ | Discrete uncountable | $\ell^\infty$ |
|---|---|---|---|
| Second-countable | yes | no | no |
| Separable | yes | no | no |
| First-countable | yes | yes | yes |

Second-countable $\Rightarrow$ separable (pick one point per basis element). The converse fails without metric structure.

## Proof Sketch

1. **Basis construction:** Let $\mathcal{B} = \{(p,q) : p, q \in \mathbb{Q},\, p < q\}$. This is countable since $\mathbb{Q} \times \mathbb{Q}$ is countable.
2. **Basis property:** every open set $U \subseteq \mathbb{R}$ is a union of open intervals; each interval $(a,b)$ is covered by all $(p,q) \subseteq (a,b)$ with $p,q \in \mathbb{Q}$ (rationals are dense).
3. **Separability:** pick $q_n \in B_n$ for each $B_n \in \mathcal{B}$. The set $\{q_n\}$ is countable. For any $x \in X$ and open $U \ni x$, there exists $B_n \subseteq U$ with $x \in B_n$, so $q_n \in U$. Hence $\{q_n\}$ is dense.

## Connections

- [[Separable Spaces]] — second-countable implies separable via `SecondCountableTopology.to_separableSpace`; the converse holds in metric spaces but not in general.
- [[Heine–Borel Theorem]] — $\mathbb{R}^n$ is second-countable (rational boxes form a countable basis), and every open cover of a compact set admits a countable subcover (Lindelof property, a consequence of second-countability).
- [[Urysohn's Lemma]] — Urysohn's lemma requires a normal Hausdorff space; combined with second-countability the space embeds into $[0,1]^\mathbb{N}$ (Urysohn metrization theorem).
- [[Tychonoff's Theorem]] — a countable product of second-countable spaces is second-countable; Tychonoff supplies the compactness of $[0,1]^\mathbb{N}$.

## Lean4 Proof

```lean4
import Mathlib.Topology.Bases
import Mathlib.Topology.MetricSpace.ProperSpace

/-- **ℝ is second-countable** — Mathlib registers this as an instance. -/
theorem real_secondCountable : SecondCountableTopology ℝ := inferInstance

/-- **Second-countable implies separable** — direct Mathlib instance. -/
theorem secondCountable_to_separable
    {X : Type*} [TopologicalSpace X] [SecondCountableTopology X] :
    SeparableSpace X :=
  SecondCountableTopology.to_separableSpace
```
