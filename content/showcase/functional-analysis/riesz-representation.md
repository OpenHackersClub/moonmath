+++
title = "Riesz Representation Theorem"
description = "Every bounded linear functional on a Hilbert space is inner product with a unique vector"
weight = 70
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "\\phi \\in H^* \\Rightarrow \\exists!\\, u \\in H : \\phi(v) = \\langle u, v \\rangle"
prerequisites = ["hahn-banach"]
lean4_status = "complete"
+++

## Statement

Let $H$ be a **Hilbert space** over $\mathbb{K}$ (real or complex). For every **bounded linear functional** $\phi : H \to \mathbb{K}$ there exists a **unique** vector $u \in H$ such that

$$\phi(v) = \langle u, v \rangle \quad \text{for all } v \in H, \quad \|\phi\| = \|u\|.$$

The map $\phi \mapsto u$ is a conjugate-linear isometric isomorphism $H^* \cong H$.

## Visualization

Take $H = \ell^2(\mathbb{N})$ with inner product $\langle x, y \rangle = \sum_{n=1}^\infty \overline{x_n} y_n$.

Define $\phi(v) = \sum_{n=1}^\infty \frac{1}{n^2} v_n$ (a bounded linear functional, since $a = (1/n^2) \in \ell^2$).

The Riesz vector is $u = (1, 1/4, 1/9, 1/16, \ldots) = (1/n^2)_{n \ge 1}$:

```
φ(v) = v₁ + v₂/4 + v₃/9 + v₄/16 + ...
     = 1·v₁ + (1/4)·v₂ + (1/9)·v₃ + ...
     = ⟨u, v⟩  where u = (1, 1/4, 1/9, ...)
```

| $v$ | $\phi(v)$ computed directly | $\langle u, v \rangle$ | Match? |
|---|---|---|---|
| $e_1 = (1,0,0,\ldots)$ | $1$ | $u_1 = 1$ | Yes |
| $e_2 = (0,1,0,\ldots)$ | $1/4$ | $u_2 = 1/4$ | Yes |
| $e_3 = (0,0,1,\ldots)$ | $1/9$ | $u_3 = 1/9$ | Yes |
| $(1,1,1,0,\ldots)$ | $1 + 1/4 + 1/9$ | $\sum_{n=1}^3 u_n$ | Yes |

Norm check: $\|\phi\| = \|u\| = \sqrt{\sum n^{-4}} = \sqrt{\pi^4/90} \approx 1.0823$.

## Proof Sketch

1. **Kernel.** If $\phi = 0$, take $u = 0$. Otherwise, $\ker \phi$ is a proper closed subspace.
2. **Orthogonal complement.** By the Hilbert projection theorem, $H = \ker \phi \oplus (\ker \phi)^\perp$ and $(\ker \phi)^\perp$ is one-dimensional; pick a unit vector $w \in (\ker \phi)^\perp$.
3. **Define $u$.** Set $u = \overline{\phi(w)} \cdot w$. Then for any $v$: write $v = (v - \frac{\phi(v)}{\phi(w)} w) + \frac{\phi(v)}{\phi(w)} w$.
4. **Verify.** The first summand is in $\ker \phi$, so $\langle u, v \rangle = \frac{\phi(v)}{|\phi(w)|^2} |\phi(w)|^2 = \phi(v)$.
5. **Norm.** $|\phi(v)| = |\langle u, v \rangle| \le \|u\| \|v\|$, and equality at $v = u$ gives $\|\phi\| = \|u\|$.

## Connections

- [[Hahn–Banach Theorem]] — in general Banach spaces, Hahn–Banach extends functionals; Riesz strengthens this to an isometric isomorphism in Hilbert spaces.
- [[Cauchy–Schwarz Inequality]] — the bound $|\phi(v)| = |\langle u, v \rangle| \le \|u\| \|v\|$ is Cauchy–Schwarz, giving $\|\phi\| \le \|u\|$.
- [[Spectral Theorem]] — the Riesz theorem identifies $H \cong H^*$; the spectral theorem for self-adjoint operators then characterises $H$-endomorphisms.
- [[Uniform Boundedness Principle]] — a family of bounded functionals $\phi_\iota$ represented by $u_\iota$ is uniformly bounded in $H$-norm iff the UBP condition holds.

## Lean4 Proof

```lean4
import Mathlib.Analysis.InnerProductSpace.Dual

/-- **Riesz Representation Theorem** for Hilbert spaces:
    every bounded linear functional equals inner product with a unique vector.
    Uses the isometric isomorphism `toDual` from `InnerProductSpace.Dual`. -/
theorem riesz_representation (E : Type*) [NormedAddCommGroup E] [InnerProductSpace ℝ E]
    [CompleteSpace E] (φ : E →L[ℝ] ℝ) :
    ∃ u : E, ∀ v : E, φ v = ⟪u, v⟫_ℝ := by
  use (InnerProductSpace.toDual ℝ E).symm φ
  intro v
  rw [InnerProductSpace.toDual_symm_apply]
```
