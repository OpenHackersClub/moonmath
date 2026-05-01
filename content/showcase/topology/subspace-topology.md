+++
title = "Subspace Topology"
description = "The subspace topology on a subset S of X is the coarsest topology making the inclusion map continuous"
weight = 100
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\tau_S = \\{U \\cap S : U \\in \\tau_X\\}"
prerequisites = ["heine-borel"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a topological space with topology $\tau_X$, and let $S \subseteq X$. The **subspace topology** (or induced topology) on $S$ is:

$$\tau_S = \{U \cap S : U \in \tau_X\}.$$

This is the coarsest topology on $S$ making the inclusion $\iota : S \hookrightarrow X$, $\iota(s) = s$, continuous.

**Continuity criterion:** A function $f : Z \to S$ is continuous (with $S$ carrying the subspace topology) if and only if $\iota \circ f : Z \to X$ is continuous.

Equivalently, $\iota = \text{Subtype.val}$ is continuous, and any $f$ is continuous into $S$ iff it is continuous when viewed as a map into $X$.

## Visualization

**$\mathbb{Q}$ as a subspace of $\mathbb{R}$:**

```
ℝ:  ────(────────────────────)────▶
         a                   b
         open interval (a,b) in ℝ

ℚ:  ─────●──●──●──●──●──●───────▶
          ↑                    (rational points)
    (a,b) ∩ ℚ is open in ℚ
    (a basic open set of the subspace topology)
```

| Open set in $\mathbb{R}$ | Corresponding open in $\mathbb{Q}$ | Contains irrationals? |
|---|---|---|
| $(-1, 1)$ | $(-1,1) \cap \mathbb{Q}$ | No |
| $(0, \sqrt{2})$ | $(0, \sqrt{2}) \cap \mathbb{Q}$ | No |
| $\mathbb{R}$ | $\mathbb{Q}$ | No |

Note that $(0, \sqrt{2}) \cap \mathbb{Q}$ is open in $\mathbb{Q}$ even though $\sqrt{2} \notin \mathbb{Q}$: the set $\{q \in \mathbb{Q} : 0 < q < \sqrt{2}\} = \{q \in \mathbb{Q} : q^2 < 2, q > 0\}$ is also equal to $(0, \sqrt{2}) \cap \mathbb{Q}$ and is open in the subspace topology.

**Continuity example:** The function $f : \mathbb{Q} \to \mathbb{Q}$, $f(q) = q^2$ is continuous in the subspace topology because $g(q) = q^2$ is continuous as a map $\mathbb{R} \to \mathbb{R}$ (a polynomial), and $f = g|_{\mathbb{Q}}$.

## Proof Sketch

1. **$\tau_S$ is a topology:** Empty set: $\emptyset = \emptyset \cap S$. Whole space: $S = X \cap S$. Unions: $\bigcup (U_\alpha \cap S) = (\bigcup U_\alpha) \cap S$. Finite intersections: $(U \cap S) \cap (V \cap S) = (U \cap V) \cap S$.

2. **Inclusion is continuous:** For $U \in \tau_X$, $\iota^{-1}(U) = U \cap S \in \tau_S$ by definition.

3. **Coarsest such topology:** Any topology $\tau'$ on $S$ making $\iota$ continuous must contain $\iota^{-1}(U) = U \cap S$ for all $U \in \tau_X$. So $\tau_S \subseteq \tau'$.

4. **Universal property:** Given $f : Z \to S$, note $\iota \circ f : Z \to X$. If $\iota \circ f$ is continuous and $V \in \tau_S$ with $V = U \cap S$, then $f^{-1}(V) = (\iota \circ f)^{-1}(U)$, which is open. Conversely, if $f$ is continuous, so is $\iota \circ f$ (composition of continuous maps).

## Connections

- **[[Heine–Borel Theorem]]** — the subspace topology on a closed bounded set $K \subseteq \mathbb{R}^n$ makes $K$ compact; Heine–Borel characterises exactly which subspaces are compact.
- **[[Bolzano–Weierstrass Theorem]]** — every bounded sequence in $\mathbb{R}^n$ takes values in a compact subspace (a closed ball), and the Bolzano–Weierstrass theorem is the statement that sequentially compact subspaces are compact.
- **[[Urysohn's Lemma]]** — subspaces of normal spaces need not be normal in general, but Urysohn's lemma constructs continuous functions separating closed sets, a tool used to show closed subspaces of normal spaces are normal.

## Lean4 Proof

```lean4
import Mathlib.Topology.Constructions

/-- The inclusion of a subtype is continuous in the subspace topology. -/
theorem subtype_inclusion_continuous
    {X : Type*} [TopologicalSpace X] (p : X → Prop) :
    Continuous (Subtype.val (p := p)) :=
  continuous_subtype_val
```
