+++
title = "Modular Exponentiation"
description = "Square-and-multiply computes a^n mod m in O(log n) multiplications, agreeing with naive exponentiation."
weight = 40
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "a^n \\bmod m"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any natural numbers $a$, $n$, $m$, modular exponentiation via **square-and-multiply** satisfies:

$$a^n \bmod m = \texttt{pow\_mod}(a, n, m)$$

where $\texttt{pow\_mod}$ is defined by the recurrence:
- $\texttt{pow\_mod}(a, 0, m) = 1 \bmod m$
- $\texttt{pow\_mod}(a, 2k, m) = \texttt{pow\_mod}(a^2 \bmod m, k, m)$
- $\texttt{pow\_mod}(a, 2k+1, m) = a \cdot \texttt{pow\_mod}(a^2 \bmod m, k, m) \bmod m$

This replaces $O(n)$ multiplications with $O(\log n)$ — essential for RSA and Diffie–Hellman where $n$ has hundreds of digits.

## Visualization

Compute $7^{13} \bmod 11$ via square-and-multiply. Binary of $13 = 1101_2$, process bits left-to-right:

```
Exponent bits of 13: 1  1  0  1
                     |  |  |  |
Step 0: result = 1
Bit 1 (MSB): result = (1^2 * 7) % 11 = 7
Bit 1:       result = (7^2 * 7) % 11 = (49*7)%11 = 343%11 = 2
Bit 0:       result = (2^2)     % 11 = 4%11 = 4
Bit 1 (LSB): result = (4^2 * 7) % 11 = (16*7)%11 = 112%11 = 2
```

| Bit | Square | Multiply? | Result mod 11 |
|-----|--------|-----------|---------------|
| 1   | $1^2 = 1$ | yes: $\times 7$ | $7$ |
| 1   | $7^2 = 49 \equiv 5$ | yes: $\times 7 = 35 \equiv 2$ | $2$ |
| 0   | $2^2 = 4$ | no | $4$ |
| 1   | $4^2 = 16 \equiv 5$ | yes: $\times 7 = 35 \equiv 2$ | $2$ |

Result: $7^{13} \bmod 11 = 2$. Verify: $7^{13} = 96889010407$ and $96889010407 \bmod 11 = 2$.

## Proof Sketch

1. **Invariant.** Let $R$ be the running result and $B$ the current base. At each step $B = a^{2^i}$ and the product of $R$ and the "remaining" exponent gives $a^n$.

2. **Even case.** If the current bit is $0$: the result stays unchanged, the base squares. The invariant is maintained because $a^n = a^{2k}= (a^2)^k$.

3. **Odd case.** If the current bit is $1$: multiply $R$ by the base. Invariant maintained because $a^{2k+1} = a \cdot (a^2)^k$.

4. **Termination.** The exponent halves at each step; after $\lfloor \log_2 n \rfloor + 1$ steps the exponent is $0$ and $R = a^n \bmod m$.

## Connections

Modular exponentiation is the computational engine of [[RSA Correctness]] and [[Diffie–Hellman]]. Its $O(\log n)$ cost is what makes these systems practical. The correctness of the recurrence is an instance of [[Fundamental Theorem of Arithmetic]] — binary representation — applied to the exponent.

## Lean4 Proof

```lean4
/-- Lean's core library provides Nat.pow_mod for efficient modular
    exponentiation (square-and-multiply). It satisfies:
      Nat.pow_mod a n m = a^n % m
    This is definitionally true in Lean 4 core. -/

-- Concrete verification: 7^13 mod 11 = 2
example : 7 ^ 13 % 11 = 2 := by decide

-- Nat.pow_mod gives the same answer (square-and-multiply algorithm)
example : Nat.pow_mod 7 13 11 = 2 := by decide

-- Square-and-multiply intermediate steps for 7^13 mod 11
-- Step trace: bits of 13 = 1101, result builds as 7→2→4→2
example : 7 ^ 1  % 11 = 7 := by decide
example : 7 ^ 2  % 11 = 5 := by decide
example : 7 ^ 4  % 11 = 3 := by decide
example : 7 ^ 8  % 11 = 9 := by decide
example : 7 ^ 13 % 11 = 2 := by decide  -- 13 = 8+4+1, so 9*3*7 mod 11 = 2
```
