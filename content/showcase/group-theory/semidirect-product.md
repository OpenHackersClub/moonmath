+++
title = "Semidirect Product"
description = "A group extension N ⋊ G where G acts on N by automorphisms, generalizing the direct product."
weight = 120
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "N \\rtimes_{\\varphi} G,\\quad (n_1,g_1)(n_2,g_2) = (n_1\\varphi(g_1)(n_2),\\, g_1 g_2)"
prerequisites = ["first-isomorphism-theorem", "cayley-theorem"]
lean4_status = "complete"
+++

## Statement

Given groups $N$ and $G$ and a homomorphism $\varphi : G \to \mathrm{Aut}(N)$, the **semidirect product** $N \rtimes_\varphi G$ is the Cartesian product $N \times G$ with multiplication

$$(n_1, g_1) \cdot (n_2, g_2) = \bigl(n_1 \cdot \varphi(g_1)(n_2),\; g_1 g_2\bigr).$$

The neutral element is $(e_N, e_G)$ and $(n, g)^{-1} = (\varphi(g^{-1})(n^{-1}), g^{-1})$.

When $\varphi$ is trivial ($\varphi(g) = \mathrm{id}$ for all $g$), this reduces to the direct product $N \times G$.

**Universal property.** $N \rtimes_\varphi G$ is the unique (up to isomorphism) group $H$ containing $N$ as a normal subgroup and $G$ as a subgroup, with $H = NG$, $N \cap G = \{e\}$, and conjugation by $g$ on $N$ given by $\varphi(g)$.

## Visualization

**Dihedral group $D_3 = \mathbb{Z}/3 \rtimes \mathbb{Z}/2$.**

Let $N = \mathbb{Z}/3 = \{0,1,2\}$ (rotations), $G = \mathbb{Z}/2 = \{0,1\}$ (reflections), and $\varphi(1)(k) = -k \pmod 3$ (the nontrivial automorphism flips the rotation direction).

Elements: $(0,0), (1,0), (2,0), (0,1), (1,1), (2,1)$ — 6 total.

Multiplication table for selected products:

| $(a,s) \cdot (b,t)$ | Result | Plain name |
|---|---|---|
| $(1,0)\cdot(1,0)$ | $(1+1,0)=(2,0)$ | $r^2$ |
| $(1,0)\cdot(0,1)$ | $(1,1)$ | $rs$ |
| $(0,1)\cdot(1,0)$ | $(0+\varphi(1)(1),1)=(-1,1)=(2,1)$ | $sr^{-1}=sr^2$ |
| $(0,1)\cdot(0,1)$ | $(\varphi(1)(0),0)=(0,0)$ | $e$ |

The relation $srs^{-1} = r^{-1}$ (i.e., $\varphi(1)(r) = r^{-1}$) is built into the multiplication — this is exactly the dihedral relation.

## Proof Sketch

1. **Associativity.** Direct calculation: $(n_1,g_1)\bigl((n_2,g_2)(n_3,g_3)\bigr) = \bigl((n_1,g_1)(n_2,g_2)\bigr)(n_3,g_3)$ using that $\varphi$ is a homomorphism.
2. **Inverses.** $(n,g)^{-1} = (\varphi(g^{-1})(n^{-1}), g^{-1})$: check $(n,g)\cdot(\varphi(g^{-1})(n^{-1}),g^{-1}) = (e_N, e_G)$ using $\varphi(g)\circ\varphi(g^{-1}) = \mathrm{id}$.
3. **Normal subgroup.** $N = \{(n, e_G)\}$ is normal: $(e_N,g)(n,e_G)(e_N,g)^{-1} = (\varphi(g)(n), e_G) \in N$.
4. **Complement.** $G = \{(e_N,g)\}$ is a subgroup isomorphic to $G$, $N \cap G = \{(e_N,e_G)\}$, and $NG = N \rtimes G$.

## Connections

The semidirect product unifies many familiar constructions: dihedral groups ($\mathbb{Z}/n \rtimes \mathbb{Z}/2$), affine groups, and Frobenius groups. It gives the splitting of short exact sequences $1 \to N \to H \to G \to 1$ when a section $G \to H$ exists — a consequence of the [[First Isomorphism Theorem]]. The structure also appears in [[Sylow Theorems]]: when $G$ has a normal Sylow $p$-subgroup $P$, any complement $Q$ makes $G \cong P \rtimes Q$.

## Lean4 Proof

```lean4
/-- The semidirect product multiplication law.
    Mathlib: `SemidirectProduct` in `Mathlib.GroupTheory.SemidirectProduct`.
    N ⋊[φ] G has carrier N × G with the twisted product. -/
theorem semidirect_mul_law {N G : Type*} [Group N] [Group G]
    (φ : G →* MulAut N) (a b : N ⋊[φ] G) :
    a * b = ⟨a.left * φ a.right b.left, a.right * b.right⟩ :=
  SemidirectProduct.mul_def a b

/-- The natural embedding of N into N ⋊[φ] G is injective. -/
theorem semidirect_inl_injective {N G : Type*} [Group N] [Group G]
    (φ : G →* MulAut N) : Function.Injective (SemidirectProduct.inl (φ := φ)) :=
  SemidirectProduct.inl_injective
```
