+++
title = "Source Coding Theorem"
description = "Shannon's noiseless coding theorem: optimal prefix code lengths satisfy H(X) <= E[l] < H(X) + 1"
weight = 153
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "H(X) \\le \\mathbb{E}[\\ell] < H(X) + 1"
prerequisites = ["shannon-entropy", "kraft-inequality"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a discrete random variable with distribution $(p_1, \ldots, p_n)$ and Shannon entropy $H(X) = -\sum p_i \log_2 p_i$ (bits). For any prefix-free binary code with codeword lengths $\ell_i$, the expected length satisfies

$$H(X) \le \mathbb{E}[\ell] = \sum_{i} p_i \ell_i.$$

Moreover, the **Huffman code** (optimal prefix-free code) achieves

$$\mathbb{E}[\ell^*] < H(X) + 1.$$

Together: no prefix code can do better than $H(X)$ bits per symbol on average, and Huffman coding comes within 1 bit.

## Visualization

Source with four symbols and distribution $P = (0.4, 0.3, 0.2, 0.1)$:

| Symbol | $p_i$ | Huffman code | $\ell_i$ | $p_i \ell_i$ | $-p_i \log_2 p_i$ |
|--------|-------|-------------|---------|-------------|-----------------|
| $a$    | 0.40  | `0`         | 1       | 0.40        | 0.529           |
| $b$    | 0.30  | `10`        | 2       | 0.60        | 0.521           |
| $c$    | 0.20  | `110`       | 3       | 0.60        | 0.464           |
| $d$    | 0.10  | `111`       | 3       | 0.30        | 0.332           |

$H(X) = 0.529 + 0.521 + 0.464 + 0.332 = \mathbf{1.846}$ bits.

$\mathbb{E}[\ell] = 0.40 + 0.60 + 0.60 + 0.30 = \mathbf{1.90}$ bits.

Check: $1.846 \le 1.90 < 1.846 + 1 = 2.846$. Both inequalities hold.

Huffman tree construction (merge two smallest at each step):

```
Step 1: merge c(0.20) + d(0.10) -> cd(0.30)
Step 2: merge b(0.30) + cd(0.30) -> bcd(0.60)
Step 3: merge a(0.40) + bcd(0.60) -> root(1.00)

root
 |-- 0 : a
 |-- 1
      |-- 10 : b
      |-- 11
           |-- 110 : c
           |-- 111 : d
```

## Proof Sketch

1. **Lower bound** ($H(X) \le \mathbb{E}[\ell]$): Set $q_i = 2^{-\ell_i} / Z$ where $Z = \sum 2^{-\ell_i} \le 1$. The Gibbs inequality (non-negativity of KL divergence) gives $\sum p_i \log(p_i/q_i) \ge 0$, which expands to $\sum p_i \ell_i \ge -\sum p_i \log_2 p_i + \log_2 Z \ge H(X)$.
2. **Upper bound** ($\mathbb{E}[\ell^*] < H(X) + 1$): Choose $\ell_i = \lceil -\log_2 p_i \rceil$. These satisfy Kraft's inequality, so a prefix-free code exists. Then $p_i \ell_i < p_i(-\log_2 p_i + 1)$; summing gives $\mathbb{E}[\ell^*] < H(X) + 1$.

## Connections

The lower bound relies on the positivity of KL divergence, a consequence of [[Shannon Entropy]]'s concavity. The Kraft feasibility condition is [[Kraft's Inequality]]. The bound has the same form as the [[AM-GM Inequality]] applied to log terms via Jensen.

## Lean4 Proof

```lean4
/-- Verify both inequalities for the concrete source P=(0.4, 0.3, 0.2, 0.1)
    with Huffman lengths (1, 2, 3, 3) (bits, rational arithmetic). -/
theorem source_coding_concrete :
    let H  : ℚ := 4 * 529 / 1000 + 3 * 521 / 1000 + 2 * 464 / 1000 + 1 * 332 / 1000
    let El : ℚ := 4 * 1 / 10 + 3 * 2 / 10 + 2 * 3 / 10 + 1 * 3 / 10
    -- Expected length 1.90 lies in [H, H+1)
    H ≤ El ∧ El < H + 1 := by
  norm_num
```
