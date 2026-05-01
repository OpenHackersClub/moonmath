+++
title = "Merkle Tree"
description = "A binary hash tree commits to a dataset so that any leaf can be verified with O(log n) hashes."
weight = 60
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "h_{\\text{root}} = H(H(h_1 \\| h_2) \\| H(h_3 \\| h_4))"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Given $n$ data blocks $d_1, \ldots, d_n$ and a collision-resistant hash function $H$, a **Merkle tree** assigns each leaf a hash $h_i = H(d_i)$ and builds internal nodes by hashing pairs:

$$h_{ij} = H(h_i \| h_j)$$

The root $h_{\text{root}}$ **commits** to the entire dataset: changing any $d_i$ changes $h_i$, which propagates up to change the root. To prove membership of $d_1$ one supplies the **authentication path** $(h_2, h_{34})$ and the verifier recomputes:

$$H(H(d_1) \| h_2) = h_{12}, \quad H(h_{12} \| h_{34}) \stackrel{?}{=} h_{\text{root}}$$

Two hashes suffice for a 4-leaf tree; in general $\lceil \log_2 n \rceil$ hashes suffice.

## Visualization

Four-leaf Merkle tree with leaves $A, B, C, D$:

```
            root = H(h_AB || h_CD)
           /                      \
    h_AB = H(h_A || h_B)    h_CD = H(h_C || h_D)
         /        \               /        \
    h_A=H(A)  h_B=H(B)     h_C=H(C)  h_D=H(D)
        A         B             C         D
```

Proof of membership for leaf $A$ (path marked with *):

```
Prover sends: (h_B, h_CD)
Verifier:
  step 1:  h_AB = H(h_A || h_B)   -- recompute using provided h_B
  step 2:  H(h_AB || h_CD) =? root -- check against known root
```

Toy example with $H(x) = x \bmod 97$ (for illustration only — not a real hash):

| Node | Value |
|------|-------|
| $h_A = H(10)$ | $10$ |
| $h_B = H(20)$ | $20$ |
| $h_C = H(30)$ | $30$ |
| $h_D = H(40)$ | $40$ |
| $h_{AB} = H(10 \cdot 100 + 20)$ | $1020 \bmod 97 = 50$ |
| $h_{CD} = H(30 \cdot 100 + 40)$ | $3040 \bmod 97 = 80$ |
| root $= H(50 \cdot 100 + 80)$ | $5080 \bmod 97 = 73$ |

To verify $A = 10$: recompute $H(10) = 10$, $H(10 \cdot 100 + 20) = 50$, $H(50 \cdot 100 + 80) = 73$ — matches the root.

## Proof Sketch

1. **Collision resistance implies binding.** If $H$ is collision resistant, no adversary can produce $d_1'$ with $H(d_1') = h_1$ without finding a preimage, and they cannot produce a fake path without finding an internal collision.

2. **Path verification terminates.** Starting from a leaf hash, each step merges two sibling hashes and moves one level up. After $\lceil \log_2 n \rceil$ steps we reach the root.

3. **Soundness.** If the reconstructed root matches the committed root, then (under collision resistance) every hash on the path was correct, which means $d_i$ is the authentic leaf.

4. **Succinct proof size.** Each authentication path has length $\lceil \log_2 n \rceil$, which for $n = 2^{20}$ is only $20$ hash values regardless of dataset size.

## Connections

Merkle trees are a data-structural companion to [[Digital Signature (Schnorr)]] — signatures authenticate public keys while Merkle proofs authenticate data. The collision-resistance assumption is related to [[One-Way Function]] (preimage resistance). The tree structure mirrors [[Binomial Theorem]] coefficient patterns in Pascal's triangle.

## Lean4 Proof

```lean4
/-- We model a toy Merkle tree over ℕ with H(x) = x % 97.
    Verify that the path proof for leaf 0 is correct. -/

def toyHash (x : ℕ) : ℕ := x % 97

def merkleLeaf (x : ℕ) : ℕ := toyHash x

-- Combine two hashes: pack as h1 * 100 + h2 then hash
def merkleNode (l r : ℕ) : ℕ := toyHash (l * 100 + r)

def hA : ℕ := merkleLeaf 10   -- = 10
def hB : ℕ := merkleLeaf 20   -- = 20
def hC : ℕ := merkleLeaf 30   -- = 30
def hD : ℕ := merkleLeaf 40   -- = 40

def hAB : ℕ := merkleNode hA hB  -- = 50
def hCD : ℕ := merkleNode hC hD  -- = 80
def root : ℕ := merkleNode hAB hCD  -- = 73

/-- Path verification for leaf A = 10, given sibling h_B and uncle h_CD. -/
theorem merkle_verify_A :
    merkleNode (merkleNode (merkleLeaf 10) hB) hCD = root := by
  decide
```
