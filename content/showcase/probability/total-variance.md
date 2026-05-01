+++
title = "Law of Total Variance"
description = "Var(X) decomposes as the sum of the expected conditional variance and the variance of the conditional mean"
weight = 90
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\text{Var}(X) = E[\\text{Var}(X|Y)] + \\text{Var}(E[X|Y])"
prerequisites = ["total-expectation"]
lean4_status = "complete"
+++

## Statement

For a square-integrable real random variable $X$ and a sub-$\sigma$-algebra $\mathcal{G}$:

$$\text{Var}(X) = E\!\left[\text{Var}(X \mid \mathcal{G})\right] + \text{Var}\!\left(E[X \mid \mathcal{G}]\right)$$

The two terms on the right are called:
- **within-group variance:** $E[\text{Var}(X \mid \mathcal{G})]$ — average spread around conditional means.
- **between-group variance:** $\text{Var}(E[X \mid \mathcal{G}])$ — spread of the conditional means themselves.

## Visualization

**Fair die, grouped by parity.** $X \in \{1,2,3,4,5,6\}$, $Y = \mathbf{1}[\text{even}]$.

**Within each group:**

| Group $Y=y$ | Outcomes | Cond. mean $\mu_y$ | Cond. variance $\sigma^2_y$ |
|-------------|----------|--------------------|----------------------------|
| Even ($y=1$) | $\{2,4,6\}$ | $4$ | $\frac{(2-4)^2+(4-4)^2+(6-4)^2}{3} = \frac{8}{3}$ |
| Odd ($y=0$)  | $\{1,3,5\}$ | $3$ | $\frac{(1-3)^2+(3-3)^2+(5-3)^2}{3} = \frac{8}{3}$ |

**Expected within-group variance:**
$$E[\text{Var}(X \mid Y)] = \frac{8}{3} \cdot \frac{1}{2} + \frac{8}{3} \cdot \frac{1}{2} = \frac{8}{3}$$

**Between-group variance (variance of conditional means):**
$$E[X \mid Y] = \begin{cases} 4 & Y=1 \\ 3 & Y=0 \end{cases}, \quad E[E[X|Y]] = 3.5$$
$$\text{Var}(E[X|Y]) = (4-3.5)^2 \cdot \frac{1}{2} + (3-3.5)^2 \cdot \frac{1}{2} = 0.25$$

**Total variance check:**
$$\text{Var}(X) = \frac{35}{12} \approx 2.917, \quad \frac{8}{3} + 0.25 = 2.667 + 0.25 = 2.917 \quad \checkmark$$

## Proof Sketch

1. **Expand** using $\text{Var}(X) = E[X^2] - (E[X])^2$.
2. **Expand conditionally:** $\text{Var}(X \mid \mathcal{G}) = E[X^2 \mid \mathcal{G}] - (E[X \mid \mathcal{G}])^2$.
3. **Apply total expectation** to the first term: $E[\text{Var}(X \mid \mathcal{G})] = E[X^2] - E[(E[X \mid \mathcal{G}])^2]$.
4. **Add the between-group term:** $\text{Var}(E[X \mid \mathcal{G}]) = E[(E[X \mid \mathcal{G}])^2] - (E[X])^2$.
5. **Sum.** $E[X^2] - E[(E[X|\mathcal{G}])^2] + E[(E[X|\mathcal{G}])^2] - (E[X])^2 = E[X^2] - (E[X])^2 = \text{Var}(X)$.

## Connections

The law of total variance extends the [[Law of Total Expectation]] to second moments. It is the probabilistic analogue of the ANOVA decomposition: the total sum of squares equals within-group plus between-group sums of squares. It also connects to [[Jensen's Inequality]]: the between-group variance is non-negative by convexity of squaring, explaining why $\text{Var}(X) \geq E[\text{Var}(X \mid \mathcal{G})]$.

## Lean4 Proof

Mathlib provides `integral_condVar_add_variance_condExp` in `Mathlib.Probability.CondVar`, which is exactly the law of total variance.

```lean4
import Mathlib.Probability.CondVar

namespace MoonMath

open MeasureTheory ProbabilityTheory

/-- **Law of Total Variance**.
    For a square-integrable `X` in a probability space,
    `E[Var(X|m)] + Var(E[X|m]) = Var(X)`. -/
theorem total_variance {Ω : Type*} {m₀ m : MeasurableSpace Ω}
    {μ : Measure Ω} [IsProbabilityMeasure μ]
    (hm : m ≤ m₀) {X : Ω → ℝ} (hX : MemLp X 2 μ) :
    μ[Var[X; μ | m]] + Var[μ[X | m]; μ] = Var[X; μ] :=
  integral_condVar_add_variance_condExp hm hX

end MoonMath
```

`integral_condVar_add_variance_condExp` is proved in Mathlib by expanding both sides via $E[X^2 \mid m] - (E[X \mid m])^2$ and applying `integral_condExp` to collapse the tower.

