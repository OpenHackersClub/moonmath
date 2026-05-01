+++
title = "Cauchy's Theorem (Groups)"
description = "If a prime p divides |G| then G has an element of order p"
weight = 50
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "p \\mid |G| \\implies \\exists g \\in G,\\; \\operatorname{ord}(g) = p"
prerequisites = ["lagrange-theorem"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a finite group and $p$ a prime. If $p \mid |G|$, then $G$ contains an element of order $p$ — that is, a $g \in G$ with $g^p = e$ and $g \neq e$.

$$p \mid |G| \;\implies\; \exists\, g \in G,\; \operatorname{ord}(g) = p$$

## Visualization

Take $|G| = 15 = 3 \times 5$. Cauchy guarantees at least one element of order 3 and at least one of order 5.

```
Group of order 15 (unique up to isomorphism: Z/15Z):

Order-1 elements: {e}                          count = 1
Order-3 elements: generators of Z/3 ≤ Z/15    count = 2  (e.g. [5] and [10])
Order-5 elements: generators of Z/5 ≤ Z/15    count = 4  (e.g. [3],[6],[9],[12])
Order-15 elements: generators of Z/15          count = 8
Total: 1 + 2 + 4 + 8 = 15  ✓

Cayley table fragment (Z/15, addition mod 15):
  [5]  + [5]  = [10]
  [10] + [5]  = [0]   →  [5] has order 3: {[0],[5],[10]}
  [3]  + [3]  = [6]
  [6]  + [3]  = [9]
  [9]  + [3]  = [12]
  [12] + [3]  = [0]   →  [3] has order 5: {[0],[3],[6],[9],[12]}
```

A smaller illustration for $|G| = 6$ ($\cong S_3$ or $\mathbb{Z}/6$):

```
p=2 divides 6  →  must have an element of order 2
p=3 divides 6  →  must have an element of order 3

In S_3:
  (12) has order 2   ✓
  (123) has order 3  ✓

In Z/6:
  [3] has order 2    ✓
  [2] has order 3    ✓
```

Cauchy says we are *guaranteed* to find these elements — they must exist even if we cannot easily exhibit them.

## Proof Sketch

The slickest proof (McKay/Aigner–Ziegler) considers the set

$$S = \{(g_1, \ldots, g_p) \in G^p \mid g_1 g_2 \cdots g_p = e\}$$

which has $|G|^{p-1}$ elements (choose $g_1,\ldots,g_{p-1}$ freely, then $g_p$ is forced). Since $p \mid |G|^{p-1}$, we have $p \mid |S|$. The cyclic group $\mathbb{Z}/p$ acts on $S$ by cyclic rotation; fixed points are tuples $(g,g,\ldots,g)$ with $g^p = e$. The identity $(e,e,\ldots,e)$ is one fixed point, and fixed points come in orbits of size $p$ or 1. Since $p \mid |S|$ and the total number of fixed points $\equiv 0 \pmod{p}$, there must be at least $p$ fixed points, i.e.\ an element $g \neq e$ with $g^p = e$. Then $\operatorname{ord}(g) \mid p$, so $\operatorname{ord}(g) = p$.

## Connections

Cauchy's theorem is the $k=1$ case of [[Sylow's Theorems]] (Sylow I guarantees a full subgroup of order $p^k$, not just an element of order $p$). It gives a partial converse to [[Lagrange's Theorem]]: Lagrange says element orders divide $|G|$; Cauchy says prime divisors of $|G|$ are realized as element orders. The theorem is applied in the classification of simple groups (e.g.\ showing $A_5$ is simple by ruling out normal $p$-Sylow subgroups), which feeds into the [[Impossibility of the Quintic Formula]]. It also appears implicitly when the [[First Isomorphism Theorem]] is used to factor maps through $p$-quotients. [[Cayley's Theorem]] embeds the group into $S_n$, and Cauchy ensures $S_n$ (for $n \geq p$) contains all the cycle types we expect.

## Lean4 Proof

```lean4
/-- **Cauchy's Theorem**: if a prime p divides the order of a finite group G,
    then G has an element of order p.
    Mathlib: `exists_prime_orderOf_dvd_card`
    in `Mathlib.GroupTheory.Perm.Cycle.Type` (proved via the McKay action). -/
theorem cauchy_thm {G : Type*} [Group G] [Fintype G] {p : ℕ} [Fact p.Prime]
    (h : p ∣ Fintype.card G) : ∃ g : G, orderOf g = p :=
  exists_prime_orderOf_dvd_card p h
```
