+++
title = "Quadratic Formula"
date = "2026-01-10"
tags = ["algebra", "polynomials", "equations"]
category = "algebra"
difficulty = "beginner"
interactive = "formula_viz::quadratic"
latex = "x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}"
+++

The **quadratic formula** gives the solutions to any quadratic equation $ax^2 + bx + c = 0$ where $a \neq 0$:

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

## The discriminant

The expression under the square root, $\Delta = b^2 - 4ac$, is called the **discriminant**:

- $\Delta > 0$: two distinct real roots
- $\Delta = 0$: one repeated real root
- $\Delta < 0$: two complex conjugate roots

## Derivation by completing the square

Starting from $ax^2 + bx + c = 0$:

1. Divide by $a$: $x^2 + \frac{b}{a}x + \frac{c}{a} = 0$
2. Move constant: $x^2 + \frac{b}{a}x = -\frac{c}{a}$
3. Complete the square: $\left(x + \frac{b}{2a}\right)^2 = \frac{b^2 - 4ac}{4a^2}$
4. Take square root: $x + \frac{b}{2a} = \pm\frac{\sqrt{b^2 - 4ac}}{2a}$
5. Solve for $x$: $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$

## Vieta's formulas

For roots $x_1, x_2$:

$$x_1 + x_2 = -\frac{b}{a}, \qquad x_1 \cdot x_2 = \frac{c}{a}$$
