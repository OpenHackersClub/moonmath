+++
title = "Galois/Counter Mode (GCM)"
description = "How AES-GCM encrypts and authenticates in one pass: a CTR-mode keystream for secrecy, and GHASH — polynomial evaluation in GF(2^128) — for a tamper-proof tag."
weight = 15
tags = ["cryptography", "aead", "finite-fields", "visualization", "gcm"]
latex = "T = \\mathrm{GHASH}_H(A, C) \\oplus E_K(J_0)"
prerequisites = []
premier = true
+++

## Statement

**Galois/Counter Mode (GCM)** is an *authenticated encryption with associated data* (AEAD) mode built on a block cipher $E_K$ (almost always AES). In a single pass over the data it produces both a **ciphertext** $C$ and a short **authentication tag** $T$. The ciphertext hides the message; the tag proves that the ciphertext *and* some cleartext associated data $A$ were produced by someone holding the key and have not been altered by a single bit.

GCM is two machines sharing one key:

$$
\underbrace{C_i = P_i \oplus E_K(\mathrm{ctr}_i)}_{\text{confidentiality — CTR mode}}
\qquad
\underbrace{T = \mathrm{GHASH}_H(A, C) \oplus E_K(J_0)}_{\text{authenticity — GHASH}}
$$

The first is counter mode: encrypt a running counter, XOR the result into the plaintext. The second is **GHASH**, a universal hash whose only ingredient is multiplication in the finite field $\mathrm{GF}(2^{128})$ — this is the *Galois* in Galois/Counter Mode.

## The field GF(2^128)

Every 128-bit block is read as an element of the field $\mathbb{F}_{2^{128}} = \mathrm{GF}(2^{128})$: a polynomial of degree $< 128$ with coefficients in $\{0,1\}$, where bit $i$ is the coefficient of $x^i$. Addition is bitwise XOR. Multiplication is polynomial multiplication reduced modulo the fixed irreducible polynomial

$$
x^{128} + x^{7} + x^{2} + x + 1 .
$$

That reduction is why the product of two 128-bit blocks is again a single 128-bit block, and why forging a tag means solving equations *in a field* rather than guessing bits. In the GCM bit convention the reduction constant is $R = \texttt{0xe1}\,\|\,0^{120}$; the interactive panel below carries out this exact multiply, bit by bit.

## The two subkeys

Everything is derived from the block cipher, so GCM needs no extra key material:

| Quantity | Definition | Role |
|----------|-----------|------|
| $H$ | $E_K(0^{128})$ | the **hash subkey** — the point at which GHASH's polynomial is evaluated |
| $J_0$ | $\mathrm{IV} \,\|\, 0^{31} \,\|\, 1$ (for a 96-bit nonce) | the **pre-counter** block |
| $E_K(J_0)$ | one cipher call on $J_0$ | the mask that hides the raw hash inside the tag |

The 96-bit nonce (IV) is the recommended length: it maps directly to $J_0$ with a trailing counter of $1$, so the keystream begins at $\mathrm{inc}_{32}(J_0)$ — the same block with its low 32 bits incremented.

## Confidentiality: counter mode

CTR mode never encrypts the plaintext directly. It encrypts a **counter** and uses the output as a one-time pad:

```
J0+1 ── E_K ──▶ keystream_1 ──┐
                              ⊕ P_1  =  C_1
J0+2 ── E_K ──▶ keystream_2 ──┐
                              ⊕ P_2  =  C_2
J0+3 ── E_K ──▶ keystream_3 ──┐
                              ⊕ P_3  =  C_3
```

Only the low 32 bits of the counter advance ($\mathrm{inc}_{32}$), which is exactly why **one key + nonce may safely encrypt at most $2^{32}-2$ blocks** ($\approx 64\ \text{GiB}$) — after that the counter wraps and keystream repeats.

## Authenticity: GHASH

GHASH folds the associated data, the ciphertext, and a length block into a single accumulator $X$, starting from zero. Each 128-bit block $B_i$ is absorbed by XOR-then-multiply:

$$
X_0 = 0, \qquad X_i = (X_{i-1} \oplus B_i)\cdot H \quad\text{in } \mathrm{GF}(2^{128}).
$$

The blocks are absorbed in a fixed order — associated data $A$ (zero-padded to a block boundary), then ciphertext $C$ (zero-padded), then one final block encoding the two bit-lengths:

$$
B_{\text{last}} = \bigl[\,\mathrm{len}(A)\,\bigr]_{64} \,\|\, \bigl[\,\mathrm{len}(C)\,\bigr]_{64}.
$$

Unrolling the recurrence reveals what GHASH really is — **evaluating a polynomial in $H$**:

$$
\mathrm{GHASH}_H(A, C) = \sum_{i=1}^{m} B_i \cdot H^{\,m-i+1},
$$

with all sums and products in $\mathrm{GF}(2^{128})$. The tag is that value, masked by the cipher:

$$
T = \mathrm{GHASH}_H(A, C) \oplus E_K(J_0),
$$

optionally truncated to $t$ bits (128, 120, 112, 104, or 96).

## Worked structure

For associated data $A$ and a two-block message, the full computation threads together like this:

```
H     = E_K(0^128)              (hash subkey)
J0    = IV ‖ 0^31 ‖ 1           (96-bit nonce)

C_1   = P_1 ⊕ E_K(J0+1)
C_2   = P_2 ⊕ E_K(J0+2)

X_1   = (0   ⊕ A_1)                 · H
X_2   = (X_1 ⊕ C_1)                 · H
X_3   = (X_2 ⊕ C_2)                 · H
S     = (X_3 ⊕ [len(A)‖len(C)])     · H

T     = S ⊕ E_K(J0)
```

Change one bit of $C_1$, of $A_1$, or of the tag itself, and $S$ lands on a completely different field element — verification fails. That is the "authenticated" in AEAD.

## Security: what the algebra buys, and what it demands

- **Integrity.** A blind forgery against a $t$-bit tag succeeds with probability about $2^{-t}$ per attempt; the polynomial structure means an attacker who cannot recover $H$ cannot do better than guessing.
- **Nonces must never repeat.** Reusing a nonce under the same key is catastrophic. Two messages with the same $J_0$ share a keystream, so their XOR leaks $P \oplus P'$. Worse, the *difference* of the two GHASH polynomials is a known polynomial in $H$; its roots reveal the hash subkey $H$ itself (the "forbidden attack"), after which an adversary forges tags at will. GCM is fast but **nonce-misuse brittle** — this is precisely what nonce-misuse-resistant modes like AES-GCM-SIV were designed to fix.
- **Data limits are real.** Both the $2^{32}$-block CTR ceiling and the $2^{64}$-block GHASH domain are hard boundaries, not advice.

## Visualization

The panel below runs a *complete, verifiable* AES-128-GCM — real AES and real $\mathrm{GF}(2^{128})$ arithmetic, pinned to the standard McGrew–Viega test vectors — so every hex value is what a production library would emit. Edit the key, nonce, associated data, or plaintext and both machines react live. Press **Play** to watch GHASH absorb one block at a time; when it reaches a ciphertext block, the matching CTR column lights up, tying "encrypt" to "authenticate," until the tag $T$ forms.

## Connections

GCM's confidentiality half is plain counter mode over a block cipher; its authenticity half is a polynomial hash in a finite field, so the same field arithmetic that underlies [[Eisenstein's Criterion]] and [[Polynomial Division]] powers the tag. The nonce-uniqueness requirement echoes the one-time-pad discipline behind every stream cipher, and the "hash then encrypt the hash" shape is the Wegman–Carter authenticator instantiated with a $\mathrm{GF}(2^{128})$ universal hash.
