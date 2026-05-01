+++
title = "Continuous Image of Compact is Compact"
description = "The continuous image of a compact set is compact, a cornerstone of analysis and topology"
weight = 120
tags = ["lean4-proof", "topology", "visualization"]
latex = "f : X \\to Y \\text{ continuous},\\; K \\text{ compact} \\Rightarrow f(K) \\text{ compact}"
prerequisites = ["heine-borel", "subspace-topology"]
lean4_status = "complete"
+++

## Statement

If $f : X \to Y$ is a continuous function between topological spaces and $K \subseteq X$ is compact, then the image $f(K) \subseteq Y$ is compact:

$$K \text{ compact},\; f \text{ continuous} \Rightarrow f(K) \text{ compact.}$$

## Visualization

**Concrete example:** $f : [-1, 1] \to \mathbb{R}$, $f(x) = x^2$.

```
Domain [-1, 1]:                     Image f([-1,1]) = [0, 1]:

  -1    -0.5    0    0.5    1          0         0.5        1
   |─────────────────────────|    →    |────────────────────|
   compact (closed, bounded)          compact (closed, bounded)

  f(-1) = 1 ────────────────────────────────────────→ 1
  f(-0.5) = 0.25 ──────────────────────────→ 0.25
  f(0)  = 0 ──────────────────────────→ 0
  f(0.5) = 0.25 ───────────────────────→ 0.25
  f(1)  = 1 ──────────────────────────────────────────→ 1
```

**Why compactness is preserved:**

| Property | Domain $[-1,1]$ | Image $[0,1]$ |
|---|---|---|
| Closed | Yes | Yes |
| Bounded | Yes (fits in $[-1,1]$) | Yes (fits in $[0,1]$) |
| Compact (Heine–Borel) | Yes | Yes |
| Any open cover has finite subcover | Yes | Yes |

**Proof idea via open covers:** Suppose $\{V_\alpha\}$ is an open cover of $f(K)$. Then $\{f^{-1}(V_\alpha)\}$ is an open cover of $K$ (by continuity, each $f^{-1}(V_\alpha)$ is open; since $f$ maps $K$ into $\bigcup V_\alpha$, these preimages cover $K$). Compact $K$ admits a finite subcover $\{f^{-1}(V_{\alpha_1}), \ldots, f^{-1}(V_{\alpha_n})\}$. Then $\{V_{\alpha_1}, \ldots, V_{\alpha_n}\}$ covers $f(K)$.

## Proof Sketch

1. Let $\{V_\alpha\}_{\alpha \in A}$ be an open cover of $f(K)$: $f(K) \subseteq \bigcup_\alpha V_\alpha$.

2. Since $f$ is continuous, each $f^{-1}(V_\alpha)$ is open in $X$.

3. We have $K \subseteq f^{-1}(f(K)) \subseteq f^{-1}\!\left(\bigcup_\alpha V_\alpha\right) = \bigcup_\alpha f^{-1}(V_\alpha)$.

4. So $\{f^{-1}(V_\alpha)\}$ is an open cover of $K$. By compactness of $K$, there is a finite subcover: $K \subseteq f^{-1}(V_{\alpha_1}) \cup \cdots \cup f^{-1}(V_{\alpha_n})$.

5. Applying $f$: $f(K) \subseteq V_{\alpha_1} \cup \cdots \cup V_{\alpha_n}$.

6. Therefore $f(K)$ is compact.

## Connections

- **[[Heine–Borel Theorem]]** — in $\mathbb{R}^n$, compactness equals closed and bounded; the theorem above combined with Heine–Borel shows that continuous images of closed bounded sets are closed and bounded — in particular, continuous functions on compact sets attain their extreme values.
- **[[Intermediate Value Theorem]]** — the IVT is an immediate corollary: $[a,b]$ is compact and connected; continuous images of connected sets are connected; connected subsets of $\mathbb{R}$ are intervals; so $f([a,b])$ is an interval containing $f(a)$ and $f(b)$.
- **[[Compact Subset of Hausdorff is Closed]]** — the image $f(K)$ is compact; if $Y$ is Hausdorff, then $f(K)$ is also closed. Together these give: continuous maps from compact spaces to Hausdorff spaces are closed maps.
- **[[Tychonoff's Theorem]]** — the product of compact spaces is compact; combined with continuous images being compact, any continuous map from the product into another space preserves compactness on products.

## Lean4 Proof

```lean4
import Mathlib.Topology.Compactness.Compact

/-- The continuous image of a compact set is compact. -/
theorem continuous_image_of_compact
    {X Y : Type*} [TopologicalSpace X] [TopologicalSpace Y]
    {f : X → Y} {s : Set X} (hs : IsCompact s) (hf : Continuous f) :
    IsCompact (f '' s) :=
  hs.image hf
```
