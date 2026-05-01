+++
title = "Liouville's Theorem"
description = "A bounded entire function on ℂ must be constant — complex differentiability is far more rigid than real differentiability"
weight = 40
tags = ["lean4-proof", "analysis", "visualization"]
latex = "f : \\mathbb{C} \\to \\mathbb{C} \\text{ entire},\\; |f| \\leq C \\implies f \\equiv \\text{const}"
prerequisites = []
lean4_status = "complete"
+++

**Liouville's theorem** is one of the most striking rigidity results in mathematics. On the real line, bounded smooth functions abound: $\sin x$, $\cos x$, $\tanh x$. Over $\mathbb{C}$, however, a function that is differentiable everywhere and stays bounded has no room to move — it must be a constant.

## Statement

Let $f : \mathbb{C} \to \mathbb{C}$ be **entire** (complex differentiable on all of $\mathbb{C}$) and **bounded**, meaning there exists $C \geq 0$ such that $|f(z)| \leq C$ for all $z \in \mathbb{C}$. Then $f$ is constant.

More generally, the same conclusion holds for entire functions $f : E \to F$ between complex Banach spaces.

## Visualization

Why does boundedness collapse $f$ to a constant? The key is Cauchy's estimate: if $f$ is analytic on the disc $D(c, R)$ and $|f| \leq C$ on the boundary circle, then

$$|f'(c)| \leq \frac{C}{R}.$$

For an entire, bounded $f$ we can take $R \to \infty$ on any centre $c$:

```
  R   |  Cauchy bound on |f'(c)|  |  conclusion
------|---------------------------|-------------------
    1 |  C / 1   = C              |  no info yet
   10 |  C / 10  = 0.1 C          |  shrinking fast
  100 |  C / 100 = 0.01 C         |  nearly zero
 1000 |  C / 1000 = 0.001 C       |  ···
  ∞   |  C / ∞   = 0              |  f'(c) = 0
```

Since $f'(c) = 0$ at every point $c \in \mathbb{C}$, and $f$ is connected, $f$ is constant. The real analogue fails because $\sin x$ is bounded and smooth but not complex-analytic on all of $\mathbb{C}$ (it grows exponentially in the imaginary direction).

A heuristic picture: an entire function is determined by its Taylor coefficients at the origin. Boundedness forces every coefficient of degree $\geq 1$ to zero — leaving only the constant term.

## Proof Sketch

Fix any $z, w \in \mathbb{C}$. We show $f(z) = f(w)$.

Define $g : \mathbb{C} \to \mathbb{C}$ by $g(t) = f(t(w-z) + z)$. Then $g$ is entire and $|g| \leq C$. Cauchy's estimate applied on $D(0, R)$ gives $|g'(0)| \leq C/R$ for every $R > 0$. Letting $R \to \infty$ shows $g'(0) = 0$. Since $g'(t) = (w-z)f'(t(w-z)+z) = 0$ for all $t$ (by the same argument shifted), $g$ is constant, so $f(z) = g(0) = g(1) = f(w)$.

## Connections

Liouville's theorem has an immediate corollary: the [[Fundamental Theorem of Algebra]]. If a non-constant polynomial $p \in \mathbb{C}[z]$ had no root, then $1/p(z)$ would be entire and bounded (since $|p(z)| \to \infty$ as $|z| \to \infty$), contradicting Liouville. The theorem also shows why the [[Cauchy Criterion|Cauchy integral formula]] is so powerful: it turns local analyticity into global constraints. In the study of [[Hausdorff Distance|metric completeness]] and fixed-point theory, the rigidity of entire functions provides examples of maps that cannot contract without being trivial.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.Liouville

open Set

/-- Liouville's theorem: a bounded entire function ℂ → ℂ is constant. -/
theorem liouville {f : ℂ → ℂ} (hf : Differentiable ℂ f)
    (hb : IsBounded (range f)) :
    ∃ c, ∀ z, f z = c :=
  hf.exists_const_forall_eq_of_bounded hb
```
