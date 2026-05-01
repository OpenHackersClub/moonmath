+++
title = "Hall's Marriage Theorem"
description = "A bipartite graph has a perfect matching from X to Y iff every subset of X has at least as many neighbors in Y."
weight = 133
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "\\forall S \\subseteq X,\\; |S| \\le |N(S)| \\iff \\exists \\text{ perfect matching}"
prerequisites = ["handshake-lemma"]
lean4_status = "complete"
+++

## Statement

Let $G = (X \cup Y, E)$ be a bipartite graph. There exists an injection $f: X \to Y$ that is a matching (each $x \in X$ is matched to a distinct neighbor $f(x)$) if and only if **Hall's condition** holds:

$$\forall S \subseteq X,\quad |S| \le |N(S)|$$

where $N(S) = \{y \in Y : \exists x \in S,\ \{x,y\} \in E\}$ is the neighborhood of $S$.

## Visualization

Bipartite graph $X = \{a,b,c\}$, $Y = \{1,2,3\}$ with $N(a)=\{1,2\}$, $N(b)=\{2\}$, $N(c)=\{1\}$:

```
X side    Y side
  a ─────── 1
  a ─────── 2
  b ─────── 2
  c ─────── 1
```

Check Hall's condition for all subsets of $X$:

| Subset $S$ | $|S|$ | $N(S)$ | $|N(S)|$ | Hall? |
|-----------|-------|---------|----------|-------|
| $\{a\}$ | 1 | $\{1,2\}$ | 2 | ok |
| $\{b\}$ | 1 | $\{2\}$ | 1 | ok |
| $\{c\}$ | 1 | $\{1\}$ | 1 | ok |
| $\{a,b\}$ | 2 | $\{1,2\}$ | 2 | ok |
| $\{a,c\}$ | 2 | $\{1,2\}$ | 2 | ok |
| $\{b,c\}$ | 2 | $\{1,2\}$ | 2 | ok |
| $\{a,b,c\}$ | 3 | $\{1,2\}$ | 2 | **FAIL** |

Hall's condition fails for $S = \{a,b,c\}$: $|\{1,2\}| = 2 < 3$. No perfect matching exists. Indeed $b$ and $c$ both compete for vertex 2 or 1, so one of $\{b, c\}$ is unmatched.

For a success example: adding edge $c$–$3$ gives $N(\{a,b,c\}) = \{1,2,3\}$ and a perfect matching $a \mapsto 2, b \mapsto 2$ ... no, $a \mapsto 1, b \mapsto 2, c \mapsto 3$.

## Proof Sketch

1. **(Necessity)** If a matching $f$ exists, then for any $S \subseteq X$ the images $f(S) \subseteq N(S)$ are distinct, so $|S| \le |N(S)|$.
2. **(Sufficiency by induction on $|X|$)** Base: $|X|=0$ trivial. Inductive step has two cases:
   - **Tight case**: every proper $S \subsetneq X$ satisfies $|N(S)| > |S|$. Pick any $x \in X$, match it to any $y \in N(x)$, remove both, verify Hall's condition still holds for the remainder.
   - **Non-tight case**: there exists $S \subsetneq X$ with $|N(S)| = |S|$. Recurse on $S$ to get a matching; the subgraph on $X \setminus S$ still satisfies Hall's condition by the tight-case argument on the residual.
3. In both cases an injection $X \to Y$ is constructed, completing the induction.

## Connections

Hall's theorem is the combinatorial core underlying the [[König's Theorem (Bipartite)]] min-cover/max-matching duality. It is also used in the proof of the [[Dilworth's Theorem]] via Mirsky's dual. The injection it guarantees connects to [[Inclusion–Exclusion Principle]] counting arguments and [[Pigeonhole Principle]] obstructions.

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.Hall.Basic

/-- Hall's Marriage Theorem: a system of finite sets admits an injective
    choice function iff Hall's condition holds for every finite subfamily.
    Mathlib: `Finset.all_card_le_biUnion_card_iff_exists_injective`. -/
theorem halls_marriage {ι α : Type*} [Fintype ι] [DecidableEq α]
    (t : ι → Finset α) :
    (∀ s : Finset ι, s.card ≤ (s.biUnion t).card) ↔
    ∃ f : ι → α, Function.Injective f ∧ ∀ x, f x ∈ t x :=
  Finset.all_card_le_biUnion_card_iff_exists_injective t
```
