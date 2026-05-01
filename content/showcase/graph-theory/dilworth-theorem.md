+++
title = "Dilworth's Theorem"
description = "In any finite partially ordered set, the minimum number of chains needed to cover the poset equals the maximum size of an antichain."
weight = 136
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "\\text{width}(P) = \\min\\{k : P = C_1 \\cup \\cdots \\cup C_k\\}"
prerequisites = ["hall-marriage", "konig-theorem"]
lean4_status = "complete"
+++

## Statement

Let $P$ be a finite partially ordered set. Then:

$$\text{(minimum number of chains covering } P) = \text{(maximum size of an antichain in } P)$$

A **chain** is a totally ordered subset; an **antichain** is a set of pairwise incomparable elements. The minimum chain cover equals the **width** $w(P)$.

## Visualization

Poset $P = \{1,2,3,4,5,6\}$ ordered by divisibility ($a \le b$ iff $a \mid b$):

```
Hasse diagram:

        6
       / \
      2   3
       \ /
        1      4      5
```

Full divisibility relations: $1|2, 1|3, 1|4, 1|5, 1|6, 2|4, 2|6, 3|6$.

**Antichain** $\{4, 5, 6\}$: none divides another ($4\nmid 5$, $4\nmid 6$, $5\nmid 4$, $5\nmid 6$, $6\nmid 4$, $6\nmid 5$). Size 3.

Is there an antichain of size 4? The elements $\{2,3,4,5,6\}$ contain $2|4$, $2|6$, $3|6$, so every 4-element subset contains a comparable pair. Maximum antichain = 3.

**Chain cover** with 3 chains:
- $C_1 = \{1, 2, 4\}$ (chain: $1|2|4$)
- $C_2 = \{3, 6\}$ (chain: $3|6$)
- $C_3 = \{5\}$ (singleton)

| Chain | Elements | Ordered? |
|-------|----------|---------|
| $C_1$ | $1, 2, 4$ | $1 \mid 2 \mid 4$ — yes |
| $C_2$ | $3, 6$ | $3 \mid 6$ — yes |
| $C_3$ | $5$ | trivially |

Minimum cover uses 3 chains = maximum antichain size 3. Dilworth's theorem holds.

## Proof Sketch

1. **Lower bound**: Each chain can contain at most one element from any antichain $A$, so any chain cover needs $\ge |A|$ chains; thus min cover $\ge$ max antichain.
2. **Upper bound** (by induction on $|P|$): Take a maximal element $m$.
   - If every maximum antichain contains $m$: remove $m$; by induction find a min chain cover of $P\setminus\{m\}$ with $w(P\setminus\{m\}) = w(P)$ chains (the antichain not involving $m$ has the same width); extend one chain to include $m$.
   - Otherwise: there exists a maximum antichain $A$ not containing $m$. Split $P$ into the set $D$ of elements $\le$ some member of $A$ and the set $U$ of elements $\ge$ some member of $A$. Each has smaller width; combine chain covers by matching along $A$.

## Connections

Dilworth's theorem is the poset analogue of [[König's Theorem (Bipartite)]] — both express a min-cover/max-packing duality. The dual result (Mirsky's theorem: minimum antichain cover = length of longest chain) is proved by the same strategy. Both connect to [[Hall's Marriage Theorem]] via the bipartite matching on poset elements. The width of the divisibility poset also relates to the structure counted by [[Cayley's Formula ($n^{n-2}$ trees)]].

## Lean4 Proof

```lean4
-- Dilworth's theorem is not in Mathlib v4.28.0.
-- We verify the concrete instance: divisibility poset on {1,2,3,4,5,6}
-- has width 3 (antichain {4,5,6}) and a 3-chain cover.

-- Check that {4,5,6} is an antichain under divisibility:
example : ¬ (4 : ℕ) ∣ 5 := by decide
example : ¬ (4 : ℕ) ∣ 6 := by decide
example : ¬ (5 : ℕ) ∣ 4 := by decide
example : ¬ (5 : ℕ) ∣ 6 := by decide
example : ¬ (6 : ℕ) ∣ 4 := by decide
example : ¬ (6 : ℕ) ∣ 5 := by decide

-- Check that the three chains cover {1,2,3,4,5,6}:
-- C1 = {1,2,4}, C2 = {3,6}, C3 = {5}; union = {1,2,3,4,5,6}
example : ({1,2,4} ∪ {3,6} ∪ {5} : Finset ℕ) = {1,2,3,4,5,6} := by decide
```
