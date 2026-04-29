+++
title = "Chain Rule"
date = "2026-01-15"
tags = ["calculus", "derivatives"]
category = "calculus"
interactive = "formula_viz::chain_rule"
latex = "\\frac{d}{dx}[f(g(x))] = f'(g(x)) \\cdot g'(x)"
+++

The **chain rule** allows us to differentiate composite functions. If we have a function $h(x) = f(g(x))$, then its derivative is:

$$\frac{d}{dx}[f(g(x))] = f'(g(x)) \cdot g'(x)$$

## Intuition

Think of it as peeling layers: differentiate the outer function, then multiply by the derivative of the inner function.

## Step-by-step example

Given $h(x) = (x^2 + 1)^3$, we identify:

- Outer function: $f(u) = u^3$, so $f'(u) = 3u^2$
- Inner function: $g(x) = x^2 + 1$, so $g'(x) = 2x$

Applying the chain rule:

$$h'(x) = 3(x^2 + 1)^2 \cdot 2x = 6x(x^2 + 1)^2$$

## General form

For a composition of $n$ functions:

$$\frac{d}{dx}[f_1 \circ f_2 \circ \cdots \circ f_n](x) = \prod_{k=1}^{n} f_k'(f_{k+1} \circ \cdots \circ f_n(x))$$
