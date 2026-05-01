+++
title = "L'Hôpital's Rule"
description = "Evaluate indeterminate-form limits by differentiating numerator and denominator"
weight = 50
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\lim_{x \\to a} \\frac{f(x)}{g(x)} = \\lim_{x \\to a} \\frac{f'(x)}{g'(x)}"
prerequisites = ["mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f$ and $g$ be differentiable near $a$ with $g'(x) \neq 0$ near $a$. If the limit $\lim_{x \to a} \frac{f(x)}{g(x)}$ is an indeterminate form $\frac{0}{0}$ or $\frac{\infty}{\infty}$, and the limit $L = \lim_{x \to a} \frac{f'(x)}{g'(x)}$ exists (or is $\pm\infty$), then:

$$\lim_{x \to a} \frac{f(x)}{g(x)} = \lim_{x \to a} \frac{f'(x)}{g'(x)} = L$$

The rule also applies when $a = \pm\infty$.

## Visualization

**Recipe for indeterminate forms:**

| Form | Strategy | Example | Result |
|------|----------|---------|--------|
| $\frac{0}{0}$ | Apply L'Hôpital directly | $\lim_{x\to 0}\frac{\sin x}{x}$ | $\frac{\cos 0}{1} = 1$ |
| $\frac{\infty}{\infty}$ | Apply L'Hôpital directly | $\lim_{x\to\infty}\frac{\ln x}{x}$ | $\frac{1/x}{1} \to 0$ |
| $0 \cdot \infty$ | Rewrite as $\frac{0}{1/\infty}$ | $\lim_{x\to 0^+} x \ln x$ | $\frac{\ln x}{1/x} \to 0$ |
| $\infty - \infty$ | Common denominator first | $\lim_{x\to 0}(\csc x - \cot x)$ | $\to 0$ |
| $1^\infty,\; 0^0,\; \infty^0$ | Take $\ln$, reduce to $0/0$ | $\lim_{x\to 0^+} x^x$ | $e^0 = 1$ |

**Worked example:** $\displaystyle\lim_{x \to 0} \frac{1 - \cos x}{x^2}$

| Step | Expression | Value |
|------|-----------|-------|
| Direct substitution | $\frac{1-\cos 0}{0^2} = \frac{0}{0}$ | indeterminate |
| L'Hôpital once | $\frac{\sin x}{2x}$ at $x=0$ | $\frac{0}{0}$ again |
| L'Hôpital twice | $\frac{\cos x}{2}$ at $x=0$ | $\boxed{1/2}$ |

## Proof Sketch

The $0/0$ case at $a$ from the right uses **Cauchy's Mean Value Theorem** (a generalized MVT):

1. **Extend $f, g$ continuously:** Set $f(a) = g(a) = 0$.
2. **Apply Cauchy MVT on $[a, x]$:** For each $x$ near $a$, there exists $c_x \in (a, x)$ with $\frac{f(x)}{g(x)} = \frac{f(x) - f(a)}{g(x) - g(a)} = \frac{f'(c_x)}{g'(c_x)}$.
3. **Squeeze:** As $x \to a^+$, $c_x$ is sandwiched between $a$ and $x$, so $c_x \to a$.
4. **Conclude:** $\lim_{x \to a^+} \frac{f(x)}{g(x)} = \lim_{c_x \to a} \frac{f'(c_x)}{g'(c_x)} = L$.

Step 2 relies on the [[Mean Value Theorem]]; step 3 uses the [[Intermediate Value Theorem]] to trap $c_x$.

## Connections

- [[Mean Value Theorem]] — Cauchy's MVT (the engine of L'Hôpital) is a direct generalisation of MVT
- [[Intermediate Value Theorem]] — used to squeeze $c_x \to a$ in the proof
- [[Chain Rule]] — many L'Hôpital computations require differentiating composite functions
- [[Taylor's Theorem]] — Taylor expansions give an alternative shortcut for $0/0$ limits at $0$
- [[Fundamental Theorem of Calculus]] — FTC evaluations often produce $0/0$ forms that L'Hôpital resolves

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.LHopital

open Filter Set

/-- L'Hôpital's rule for the 0/0 form from the right on an open interval (a, b).
    Wraps Mathlib's `HasDerivAt.lhopital_zero_right_on_Ioo`. -/
theorem lhopital_zero_right {f g f' g' : ℝ → ℝ} {a b : ℝ} {l : Filter ℝ}
    (hab : a < b)
    (hff' : ∀ x ∈ Ioo a b, HasDerivAt f (f' x) x)
    (hgg' : ∀ x ∈ Ioo a b, HasDerivAt g (g' x) x)
    (hg' : ∀ x ∈ Ioo a b, g' x ≠ 0)
    (hfa : Filter.Tendsto f (nhdsWithin a (Ioi a)) (nhds 0))
    (hga : Filter.Tendsto g (nhdsWithin a (Ioi a)) (nhds 0))
    (hdiv : Filter.Tendsto (fun x => f' x / g' x) (nhdsWithin a (Ioi a)) l) :
    Filter.Tendsto (fun x => f x / g x) (nhdsWithin a (Ioi a)) l :=
  HasDerivAt.lhopital_zero_right_on_Ioo hab hff' hgg' hg' hfa hga hdiv
```
