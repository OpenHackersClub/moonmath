# Wave 2 Showcase Audit (May 2026)

This document records the QA audit performed after Wave 2 of the showcase content
expansion (250 total notes across 23 categories). Five reviewer agents ran in parallel
covering Lean4 compilation, frontmatter integrity, wikilink coverage, content quality,
and dedup/category coherence.

The follow-up PRs in this stack address the actionable findings:

- **PR 2** — `fix/normalize-wikilinks-tags`: typography sweep (Cauchy–Schwarz dash,
  possessives, accents) + missing `visualization` / `lean4-proof` tags + the
  `fractal` → `fractal-geometry` tag fix.
- **PR 3** — `fix/lean-compile`: corrects the 16 sample failures uncovered by the
  `lake env lean` audit.
- **PR 4** — `refactor/content-dedup-and-quality`: merges duplicate Baire / Burnside /
  generalized-MVT pages; moves Liouville + Fundamental Theorem of Algebra from
  `analysis/` to `complex-analysis/`; rewrites weak Lean blocks; adds the cross-links
  the network analysis flagged.

## Headline numbers (post-Wave-2, pre-fix)

| Metric | Value |
|---|---|
| Showcase notes | 263 |
| Categories | 23 |
| Lean static-clean rate | 257 / 263 (97.7%) |
| Lean real-compile pass rate (30-note sample) | 14 / 30 (47%) |
| Wikilinks total | 896 |
| Broken wikilinks | 83 (9.3% — collapses to 7 typography patterns) |
| Notes with ≥ 2 wikilinks | 260 / 263 (98.9%) |
| Mean content quality (40-note sample, 0–15) | 14.25 |
| Dedup clusters | 14 (4 merge / 2 rename / 5 cross-link / 3 keep) |

## A — Lean4 compilation review

Tier 1 (static, all 263 notes): 257 clean. The 6 outstanding items are pre-existing
content (`algebra/quadratic`, `calculus/chain-rule` predate the Lean-block convention;
the four `lean4_status="sorry"` notes in `fractal-geometry/` and `galois-theory/`
are deliberate placeholders).

Tier 2 (real `lake env lean` on a 30-note stratified sample):

```
total: 30 | pass: 14 | fail: 16 | timeout: 0
```

The 16 failures cluster into three modes:

1. **Wrong lemma name / type mismatch** (6) — most common cause. Examples:
   `Metric.hausdorffDist_singleton_singleton` is not in Mathlib v4.28.0;
   `two_mul_le_add_sq` proves `2*a*b ≤ a²+b²` but `am_gm_two` in our note
   binds it to `2*(a*b) ≤ a²+b²` — different parenthesization.
2. **Unbound name / malformed syntax** (4) — `VectorField` namespace absent;
   `Complex.abs` is a function not a constant; `#eval` accidentally embedded
   inside a `theorem` block in `cryptography/diffie-hellman`.
3. **Missing `noncomputable` / typeclass binder errors** (4) —
   `fractal-geometry/koch-snowflake` defines a real-valued recurrence without
   `noncomputable`; `[ ]` instance-binders used for non-typeclass hypotheses.

Hardest-hit categories: `cryptography` (2/2), `differential-geometry` (2/2),
`fractal-geometry` (3/5), `galois-theory` (2/3). Cleanest:
`number-theory`, `linear-algebra`, `group-theory`, `graph-theory`, `field-theory`,
`information-theory`, `calculus`, `category-theory`.

PR 3 ships per-file fixes for all 16.

## B — Frontmatter & weights

39 frontmatter issues across 26 files, all in pre-existing pre-Wave-1 notes plus
the original 5 fractal-geometry pages:

- 18 notes missing `visualization` tag.
- 10 notes missing `lean4-proof` tag.
- All 6 fractal-geometry notes use `fractal` instead of `fractal-geometry`.
- `algebra/quadratic` and `calculus/chain-rule` predate the tagging schema entirely.

7 weight "collisions" are false positives — each is a content note whose `weight`
matches its category `_index.md` weight, which orders different scopes (sections vs.
pages within a section) and is harmless. PR 2 normalizes the weights anyway so the
indexer can be simpler.

PR 2 ships these fixes.

## C — Wikilink integrity

896 wikilinks total. 83 broken (9.3%) collapse into seven typography patterns:

| Pattern | Count | Example |
|---|---|---|
| en-dash vs hyphen | 22 | `[[Cauchy–Schwarz Inequality]]` vs title `Cauchy-Schwarz Inequality` |
| Possessive mismatches | 30 | `[[Urysohn's Lemma]]` vs title `Urysohn Lemma` |
| Hyphenation | 6 | `[[Brouwer Fixed Point Theorem]]` vs title `Brouwer Fixed-Point Theorem` |
| Unicode accents | 4 | `[[L'Hopital's Rule]]` vs title `L'Hôpital's Rule` |
| Misc | 21 | various |

98.9% of notes have ≥ 2 wikilinks. Only 2 zero-link notes and 3 missing
`## Connections` headings.

Top hub notes by incoming-wikilinks: Heine–Borel (24), Galois Fundamental (21),
Fermat's Little (20), Mean Value Theorem (19), Iterated Function Systems (19).

PR 2 normalizes both link text and titles to match.

## D — Content quality (40-note sample, seed=42)

Mean 14.25 / 15. 90% of sampled notes scored ≥ 13. Eleven categories averaged a
perfect 15.0. Weakest categories: `galois-theory` (11.0, pre-existing),
`combinatorics` (12.5), `fractal-geometry` (12.5).

Most common weakness: Lean blocks that prove a trivial side-claim (e.g.
`norm_num`/`ring` on numeric literals) instead of the headline theorem. Three
specific outliers flagged for rewrite: `combinatorics/chu-vandermonde`,
`differential-geometry/green-theorem`, `information-theory/mutual-information`.
PR 4 rewrites these.

## E — Dedup & navigation

14 dedup clusters identified:

| Verdict | Count | Example |
|---|---|---|
| MERGE | 4 | Baire (analysis vs topology), Burnside (group vs combinatorics), Generalized MVT vs Cauchy MVT, Chu-Vandermonde restatement |
| RENAME | 2 | `galois-theory/fundamental-theorem` → `fundamental-theorem-galois` |
| CROSS-LINK | 5 | weierstrass-approximation ↔ stone-weierstrass, etc. |
| KEEP BOTH | 3 | distinct concepts that share names (Cauchy's Theorem in groups vs. complex analysis) |

Coherence gaps: `set-theory-logic` lacks Incompleteness + AC; Liouville and FTA
are misplaced in `analysis/` and belong in `complex-analysis/` (where Cauchy's
theorem itself is also missing). PR 4 fixes these.

## Reading tracks (proposed by Reviewer E)

1. **Why Primes Never End** — Infinitude → FTA → Euclidean Algorithm → Fermat's
   Little → Euler's Totient → Quadratic Residues → Quadratic Reciprocity (7 notes).
2. **Algebra's Backbone** — Lagrange → First Iso → Sylow → Euclidean Domain →
   Eisenstein → Splitting Field → Galois Fundamental → Quintic (8 notes).
3. **Infinite-Dimensional Analysis** — Heine–Borel → Baire → Hahn-Banach → Uniform
   Boundedness → Open Mapping → Closed Graph → Riesz Representation → Spectral
   Radius (8 notes).
4. **Geometry Meets Analysis** — FTC → Exterior Derivative → Poincaré Lemma →
   Stokes → Green → Divergence → Gauss-Bonnet → Brouwer Fixed-Point (8 notes).

These are referenced in `_index.md` files added in PR 4.
