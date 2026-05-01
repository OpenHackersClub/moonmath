+++
title = "Euclidean Algorithm"
description = "The oldest surviving algorithm, computing the greatest common divisor via iterated remainders"
weight = 50
tags = ["lean4-proof", "number-theory", "algorithm", "visualization"]
latex = "\\gcd(m,n) = \\gcd(n \\bmod m, m)"
prerequisites = ["fundamental-theorem-arithmetic"]
lean4_status = "complete"
+++

## Statement

For any natural numbers $m$ and $n$, the **greatest common divisor** satisfies:

$$\gcd(m, n) = \gcd(n \bmod m,\, m)$$

Iterating this recurrence terminates in finitely many steps because the right-hand argument strictly decreases, reaching $\gcd(d, 0) = d$.

Additionally, the gcd is the **largest** common divisor: if $k \mid m$ and $k \mid n$ then $k \mid \gcd(m, n)$.

## Visualization

Step-by-step trace for $\gcd(252, 105)$:

```
Step  Call              Quotient  Remainder
  1   gcd(252, 105)     2         42
  2   gcd(105,  42)     2         21
  3   gcd( 42,  21)     2          0
  4   gcd( 21,   0)     —         base case: 21
```

Divisibility lattice (arrows mean "divides"):

```
                 1
                / \
               3   7
              / \ / \
             21   21
              \   /
               21
               |
      gcd(252,105) = 21
```

Check: $252 = 21 \times 12$ and $105 = 21 \times 5$, with $\gcd(12, 5) = 1$.

| $m$  | $n$  | $n \bmod m$ |
|-----:|-----:|------------:|
| 252  | 105  |  42         |
| 105  |  42  |  21         |
|  42  |  21  |   0         |
|  21  |   0  | base: **21** |

## Proof Sketch

1. **Correctness of the recurrence.** Any common divisor of $m$ and $n$ also divides $n \bmod m = n - \lfloor n/m \rfloor \cdot m$. Conversely, any common divisor of $m$ and $n \bmod m$ divides $n$. So the sets of common divisors are identical, hence the greatest is the same.

2. **Termination.** At each step the pair $(m, n)$ is replaced by $(n \bmod m, m)$. Since $0 \le n \bmod m < m$, the first argument strictly decreases until it reaches 0.

3. **Base case.** $\gcd(0, d) = d$ by definition — $d$ divides $0$ and is the largest divisor of $d$.

4. **Every common divisor divides the gcd.** By induction on the number of steps: once we reach $\gcd(d, 0) = d$, the claim holds trivially, and each step preserves the set of common divisors.

## Connections

The Euclidean algorithm is the computational backbone of [[Bezout's Identity]], which shows the gcd is an integer linear combination of its arguments. It underpins the [[Chinese Remainder Theorem]] (coprimality) and [[Euler's Totient Function]] (counting units). The [[Fundamental Theorem of Arithmetic]] tells us the gcd can also be read off prime factorisations, but the Euclidean algorithm is far more efficient.

## Lean4 Proof

```lean4
/-- The Euclidean recurrence: `gcd m n = gcd (n % m) m`.
    Mathlib calls this `Nat.gcd_rec`. -/
theorem euclidean_rec (m n : ℕ) : Nat.gcd m n = Nat.gcd (n % m) m :=
  Nat.gcd_rec m n

/-- The gcd divides both arguments. -/
theorem gcd_divides (m n : ℕ) : Nat.gcd m n ∣ m ∧ Nat.gcd m n ∣ n :=
  ⟨Nat.gcd_dvd_left m n, Nat.gcd_dvd_right m n⟩

/-- Any common divisor divides the gcd. -/
theorem common_dvd_gcd {m n k : ℕ} (hm : k ∣ m) (hn : k ∣ n) : k ∣ Nat.gcd m n :=
  Nat.dvd_gcd hm hn
```
