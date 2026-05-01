+++
title = "Cayley's Theorem"
description = "Every group embeds into a symmetric group"
weight = 20
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "G \\hookrightarrow S_{|G|}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Every group $G$ is isomorphic to a subgroup of the symmetric group $\text{Sym}(G)$ (permutations of the underlying set of $G$). In other words, **every abstract group is concretely a group of permutations**.

$$G \hookrightarrow \text{Sym}(G), \quad g \mapsto (x \mapsto gx)$$

## Visualization

Take $G = \mathbb{Z}/3\mathbb{Z} = \{0, 1, 2\}$ under addition mod 3. Left multiplication by each element gives a permutation of $\{0,1,2\}$:

```
Element 0 acts as: 0→0, 1→1, 2→2   ≅  (1)(2)(3)  — identity permutation
Element 1 acts as: 0→1, 1→2, 2→0   ≅  (0 1 2)    — 3-cycle
Element 2 acts as: 0→2, 1→0, 2→1   ≅  (0 2 1)    — inverse 3-cycle
```

As permutation matrices (rows = inputs, columns = outputs, entry 1 where $g \cdot \text{row} = \text{col}$):

```
ρ(0):             ρ(1):             ρ(2):
[ 1  0  0 ]       [ 0  0  1 ]       [ 0  1  0 ]
[ 0  1  0 ]       [ 1  0  0 ]       [ 0  0  1 ]
[ 0  0  1 ]       [ 0  1  0 ]       [ 1  0  0 ]
```

The map $0 \mapsto \rho(0),\; 1 \mapsto \rho(1),\; 2 \mapsto \rho(2)$ is an injective group homomorphism into $S_3$. Its image $\{(),(012),(021)\}$ is the unique cyclic subgroup of order 3 in $S_3$.

Cayley table of $\mathbb{Z}/3$:

```
+ | 0  1  2
--+--------
0 | 0  1  2
1 | 1  2  0
2 | 2  0  1
```

Reading rows as permutations gives exactly the three permutation matrices above.

## Proof Sketch

Define $\phi : G \to \text{Sym}(G)$ by $\phi(g)(x) = gx$.

1. $\phi(g)$ is a bijection: left multiplication by $g$ has inverse $x \mapsto g^{-1}x$.
2. $\phi$ is a homomorphism: $\phi(gh)(x) = (gh)x = g(hx) = \phi(g)(\phi(h)(x))$.
3. $\phi$ is injective: if $\phi(g) = \phi(g')$ then $g = ge = \phi(g)(e) = \phi(g')(e) = g'e = g'$.

## Connections

Cayley's theorem shows that abstract group theory is no more general than the theory of permutation groups — every result proved for $\text{Sym}(X)$ has a structural reading for general groups. The embedding is used in [[Lagrange's Theorem]] proofs (cosets as orbits), feeds into [[Sylow's Theorems]] (embedding into symmetric groups to count fixed points), and motivates the representation theory route toward the [[Impossibility of the Quintic Formula]] (Galois groups act faithfully on roots, making them concrete permutation groups). The [[Fundamental Theorem of Galois Theory]] is in part a story about which permutation subgroups fix which subfields.

## Lean4 Proof

```lean4
/-- **Cayley's Theorem**: every group embeds injectively into a symmetric group
    via left multiplication. The action of `G` on itself by left multiplication
    is faithful, so `MulAction.toPermHom` gives an injective group hom
    `G →* Equiv.Perm G`.
    Mathlib: `MulAction.toPermHom` and `MulAction.toPerm_injective`
    in `Mathlib.Algebra.Group.Action.Basic`. -/
theorem cayley {G : Type*} [Group G] :
    ∃ f : G →* Equiv.Perm G, Function.Injective f :=
  ⟨MulAction.toPermHom G G, MulAction.toPerm_injective⟩
```
