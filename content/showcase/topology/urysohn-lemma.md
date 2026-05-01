+++
title = "Urysohn Lemma"
description = "In a normal space, disjoint closed sets can be separated by a continuous function"
weight = 40
tags = ["lean4-proof", "topology", "separation"]
latex = "\\exists\\, f : X \\to [0,1],\\quad f|_s = 0,\\; f|_t = 1"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $X$ be a **normal** topological space (T4: disjoint closed sets can be separated by open sets). For any two disjoint closed sets $s, t \subseteq X$, there exists a continuous function $f : X \to [0,1]$ such that

$$f(x) = 0 \text{ for all } x \in s, \qquad f(x) = 1 \text{ for all } x \in t.$$

## Visualization

Two disjoint closed discs $s$ (left) and $t$ (right) in $\mathbb{R}^2$, with the separating function drawn as altitude:

```
ℝ² top view (f is height, shown as contour levels):

       s                        t
   ●●●●●●●     0.25   0.5   0.75  ●●●●●●●
  ●●●●●●●●●  ──┄┄┄───┄┄┄───┄┄┄──  ●●●●●●●●●
  ●●●●●●●●●  f=0      ↑       f=1 ●●●●●●●●●
   ●●●●●●●    contour lines of f   ●●●●●●●●
              evenly spaced in the gap

f = 0 everywhere on s (left disc, solid)
f = 1 everywhere on t (right disc, solid)
f rises smoothly from 0 to 1 across the gap
f is continuous — no jumps allowed
```

The function $f$ acts as a "landscape" separating the two closed islands. Normality of $X$ is what guarantees the gap between $s$ and $t$ is wide enough (in the topological sense) to fill with a continuous slope.

## Proof Sketch

**Step 1.** Since $X$ is normal and $s \cap t = \varnothing$, find disjoint open sets $U_0 \supseteq s$ and $V_0 \supseteq t$.

**Step 2.** Using normality repeatedly on dyadic rationals $D = \{k/2^n : 0 \le k \le 2^n,\, n \ge 1\}$, construct a family of open sets $\{U_r : r \in D\}$ with $\overline{U_r} \subseteq U_{r'}$ whenever $r < r'$.

**Step 3.** Define $f(x) = \inf\{r \in D : x \in U_r\}$ (with $\inf \varnothing = 1$). Verifying continuity uses the chain $\{U_r\}$; the boundary conditions $f|_s = 0$ and $f|_t = 1$ follow by construction.

This inductive "dyadic filling" is the core of Urysohn's construction.

## Connections

- **[[Heine–Borel Theorem]]** — compact Hausdorff spaces are normal, so Urysohn applies; in $\mathbb{R}^n$ normality is automatic.
- **[[Bolzano–Weierstrass Theorem]]** — Urysohn functions give explicit test functions used to separate limit points of sequences.
- **[[Brouwer Fixed-Point Theorem]]** — Urysohn's lemma underlies the partition-of-unity construction, which feeds into degree theory.
- **[[Tychonoff Theorem]]** — Tychonoff spaces (completely regular) are precisely those where Urysohn-type separation by continuous functions is available; normality strengthens this to full separation.
- **[[Hausdorff Distance]]** — Urysohn-type functions appear in the proof that the Hausdorff metric topology on compact subsets of a normal space behaves well.
- **[[Iterated Function Systems]]** — Partition of unity (an extension of Urysohn) allows gluing local definitions of IFS maps into globally continuous ones.

## Lean4 Proof

```lean4
import Mathlib.Topology.UrysohnsLemma

open Set ContinuousMap

/-- **Urysohn's Lemma**: in a normal topological space, any two disjoint closed sets
    can be separated by a continuous function with values in [0, 1].
    Mathlib: `exists_continuous_zero_one_of_isClosed`. -/
theorem urysohn_separation {X : Type*} [TopologicalSpace X] [NormalSpace X]
    {s t : Set X} (hs : IsClosed s) (ht : IsClosed t) (hst : Disjoint s t) :
    ∃ f : C(X, ℝ), EqOn f 0 s ∧ EqOn f 1 t ∧ ∀ x, f x ∈ Icc (0 : ℝ) 1 :=
  exists_continuous_zero_one_of_isClosed hs ht hst
```
