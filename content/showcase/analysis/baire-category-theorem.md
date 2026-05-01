+++
title = "Baire Category Theorem"
description = "A complete metric space cannot be written as a countable union of nowhere-dense closed sets"
weight = 130
tags = ["lean4-proof", "analysis", "visualization", "topology"]
latex = "X \\text{ complete metric} \\Rightarrow X = \\bigcup_n F_n,\\; F_n \\text{ closed} \\Rightarrow \\exists n: \\overline{\\mathrm{int}\\,F_n} \\neq \\emptyset"
prerequisites = []
lean4_status = "complete"
+++

The **Baire Category Theorem** is a topological miracle: a complete metric space is "large" in a precise sense — it cannot be exhausted by countably many thin (nowhere-dense) sets. Its contrapositive powers existence proofs throughout functional analysis.

## Statement

**Baire Category Theorem**: Let $X$ be a complete metric space (or a locally compact Hausdorff space). If $X = \bigcup_{n=1}^\infty F_n$ with each $F_n$ closed, then at least one $F_n$ has nonempty interior.

Equivalently: a countable intersection of **open dense** sets is dense.

**Corollary** (Banach-Steinhaus, Open Mapping, Closed Graph): all three follow from applying Baire to the Banach space viewed as a complete metric space.

A set is called **meager** (first category) if it is a countable union of nowhere-dense sets. Baire says: a complete metric space is **not meager in itself**.

## Visualization

**Why $\mathbb{Q}$ is meager in $\mathbb{R}$:**

Write $\mathbb{Q} = \bigcup_{n=1}^\infty \{q_n\}$ (rationals are countable). Each singleton $\{q_n\}$ is closed and has empty interior — so $\mathbb{Q}$ is a countable union of nowhere-dense sets.

But $\mathbb{R}$ is complete, so Baire says $\mathbb{R}$ is not meager — meaning $\mathbb{R} \neq \mathbb{Q}$. (This gives a non-constructive proof that irrationals exist!)

**Illustration of nowhere-dense sets stacking up:**

```
Real line:  ---[--]--[--]--[--]--[--]---...---> ℝ
             F1    F2    F3    F4

Each Fn = {q_n} is a point (closed, empty interior).
Their union = Q has dense complement (irrationals).
Baire: the complement of a meager set is dense.
```

**Baire vs. non-complete spaces:** $\mathbb{Q}$ itself with the induced metric is NOT complete, and indeed $\mathbb{Q}$ is meager in $\mathbb{Q}$: $\mathbb{Q} = \bigcup_n \{q_n\}$, each $\{q_n\}$ is nowhere dense in $\mathbb{Q}$. So $\mathbb{Q}$ fails to be a Baire space — consistent with incompleteness.

**Table: Baire vs. non-Baire spaces**

| Space | Complete? | Baire space? | Meager in itself? |
|-------|-----------|--------------|-------------------|
| $\mathbb{R}$ | yes | yes | no |
| $[0,1]$ | yes | yes | no |
| $\mathbb{Q}$ | no | no | yes |
| $C([0,1])$ | yes (sup norm) | yes | no |
| $\mathbb{R} \setminus \mathbb{Q}$ | no | yes (Baire, $G_\delta$) | no |

## Proof Sketch

1. Assume for contradiction that every $F_n$ has empty interior (i.e., $F_n^c$ is open dense for all $n$).

2. Start with any open ball $B_0$. Since $F_1^c$ is open dense, $B_0 \cap F_1^c$ is nonempty; pick a closed ball $\overline{B_1} \subset B_0 \cap F_1^c$ of radius $\leq 1/2$.

3. Since $F_2^c$ is open dense, pick $\overline{B_2} \subset B_1 \cap F_2^c$ with radius $\leq 1/4$.

4. Continue: get a nested sequence $\overline{B_1} \supset \overline{B_2} \supset \cdots$ with radii $\to 0$ and $\overline{B_n} \cap F_n = \emptyset$.

5. By completeness, the centres form a Cauchy sequence with limit $x$. Then $x \in \bigcap_n \overline{B_n}$, so $x \notin F_n$ for all $n$ — contradicting $X = \bigcup_n F_n$.

## Connections

The Baire Category Theorem underpins three pillars of functional analysis: the Banach-Steinhaus theorem (uniform boundedness), the open mapping theorem, and the closed graph theorem. It also explains why certain objects must exist without constructing them: e.g., continuous nowhere-differentiable functions (a generic function in $C([0,1])$ has this property). See [[Arzelà–Ascoli Theorem]] for the compactness companion, and [[Cauchy Criterion]] for the completeness concept the theorem rests on.

## Lean4 Proof

```lean4
import Mathlib.Topology.Baire.CompleteMetrizable

/-- Baire Category Theorem: every complete metric space is a Baire space,
    meaning a countable intersection of open dense sets is dense. -/
theorem baire_complete_metric (X : Type*) [MetricSpace X] [CompleteSpace X] :
    BaireSpace X :=
  inferInstance
```
