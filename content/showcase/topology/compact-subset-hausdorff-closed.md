+++
title = "Compact Subset of Hausdorff is Closed"
description = "In a Hausdorff space every compact subset is closed, generalising the Heine–Borel direction compact implies closed"
weight = 130
tags = ["lean4-proof", "topology", "visualization"]
latex = "X \\text{ Hausdorff},\\; K \\text{ compact} \\Rightarrow K \\text{ closed}"
prerequisites = ["hausdorff-implies-t1", "continuous-image-compact"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a Hausdorff (T2) topological space and let $K \subseteq X$ be compact. Then $K$ is closed:

$$X \text{ Hausdorff},\quad K \subseteq X \text{ compact} \Rightarrow K \text{ closed.}$$

This is a strict generalisation of the Heine–Borel direction "compact $\Rightarrow$ closed" from $\mathbb{R}^n$ to arbitrary Hausdorff spaces.

**The Hausdorff condition is essential.** In a non-Hausdorff space, compact sets need not be closed. Example: the Sierpinski space $\{0,1\}$ with topology $\{\emptyset, \{1\}, \{0,1\}\}$ — the singleton $\{1\}$ is compact (trivially) but not closed (its complement $\{0\}$ is not open).

## Visualization

**Counter-example anatomy — why $\{1/n : n \ge 1\}$ is NOT compact in $\mathbb{R}$:**

```
ℝ:  0    1/4  1/3  1/2  1    
     |──●────●────●────●────●──▶
     ↑   ↑    ↑    ↑    ↑
  missing  1/4  1/3  1/2  n=1
  limit 0
  (not in set → not closed → not compact)

  Open cover: U_n = (1/(n+1), ∞) covers {1/n : n≥1}
  but NO finite sub-collection covers it (any finite sub-
  collection misses 1/n for all large n).
```

**Compact and closed — $K = \{0\} \cup \{1/n : n \ge 1\}$ in $\mathbb{R}$:**

```
ℝ:  0    1/4  1/3  1/2  1
     |────●────●────●────●──▶
     ●                      ← 0 IS in K (limit included)
  K is closed (contains its limit point 0)
  K is compact (every open cover has finite subcover)
  ℝ is Hausdorff → theorem applies ✓
```

**Proof idea in pictures:**

```
Fix x ∉ K. For each k ∈ K, Hausdorff gives disjoint opens:
   U_k ∋ x    V_k ∋ k    U_k ∩ V_k = ∅

{V_k : k ∈ K} covers K → finite subcover V_{k₁},...,V_{kₙ}
Let U = U_{k₁} ∩ ... ∩ U_{kₙ} (open, contains x)
Then U ∩ K ⊆ U ∩ (V_{k₁} ∪ ... ∪ V_{kₙ}) = ∅
So U ⊆ X \ K: every x ∉ K has an open neighbourhood in X \ K.
Therefore X \ K is open, i.e. K is closed.
```

## Proof Sketch

1. Fix $x \in X \setminus K$. We find an open neighbourhood of $x$ disjoint from $K$.

2. For each $k \in K$, by the Hausdorff condition, there exist disjoint open sets $U_k \ni x$ and $V_k \ni k$.

3. The collection $\{V_k : k \in K\}$ is an open cover of $K$. Since $K$ is compact, finitely many suffice: $K \subseteq V_{k_1} \cup \cdots \cup V_{k_n}$.

4. Let $U = U_{k_1} \cap \cdots \cap U_{k_n}$. This is a finite intersection of open sets, so $U$ is open, and $x \in U$.

5. For each $i$, $U \subseteq U_{k_i}$ and $U_{k_i} \cap V_{k_i} = \emptyset$, so $U \cap V_{k_i} = \emptyset$.

6. Therefore $U \cap K \subseteq U \cap (V_{k_1} \cup \cdots \cup V_{k_n}) = \emptyset$, meaning $U \subseteq X \setminus K$.

7. Since $x$ was arbitrary, $X \setminus K$ is open, so $K$ is closed.

## Connections

- **[[Heine–Borel Theorem]]** — in $\mathbb{R}^n$ (which is Hausdorff), this theorem gives "compact $\Rightarrow$ closed", exactly the easy half of Heine–Borel; the hard half (closed and bounded $\Rightarrow$ compact) uses completeness of $\mathbb{R}$.
- **[[Hausdorff Implies T1]]** — closedness of singletons (T1) follows from Hausdorff; the present theorem shows the Hausdorff property forces much more: all compact sets are closed, not just singletons.
- **[[Continuous Image of Compact is Compact]]** — combining these two theorems: a continuous bijection from a compact space to a Hausdorff space is automatically a homeomorphism (the inverse is continuous because it maps closed compact sets to closed compact sets).
- **[[Urysohn's Lemma]]** — compact Hausdorff spaces are normal (T4); this uses the compactness-implies-closed result to show any two disjoint closed sets in a compact Hausdorff space can be separated, the hypothesis Urysohn's lemma needs.

## Lean4 Proof

```lean4
import Mathlib.Topology.Separation.Hausdorff

/-- A compact subset of a Hausdorff space is closed. -/
theorem compact_in_hausdorff_is_closed
    {X : Type*} [TopologicalSpace X] [T2Space X]
    {s : Set X} (hs : IsCompact s) : IsClosed s :=
  hs.isClosed
```
