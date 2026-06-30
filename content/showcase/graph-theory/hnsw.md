+++
title = "Hierarchical Navigable Small World (HNSW)"
description = "A graph-based index for approximate nearest-neighbour search that descends a stack of navigable small-world graphs in logarithmic time."
weight = 60
tags = ["graph-theory", "algorithms", "nearest-neighbor", "visualization", "hnsw"]
latex = "Q(N) = O(\\log N)"
prerequisites = []
premier = true
+++

## Statement

**HNSW** (Malkov & Yashunin, 2016) is the dominant index for **approximate nearest-neighbour (ANN)** search over high-dimensional vectors — the backbone of vector databases, semantic search, and retrieval-augmented generation. Given a set of $N$ points and a query $q$, it returns the (approximate) closest point by traversing a graph instead of scanning all $N$ vectors, achieving an expected query cost of

$$Q(N) = O(\log N)$$

The structure is a **stack of proximity graphs**. Layer $0$ contains every point; each higher layer keeps an exponentially thinner random sample. Searching starts at a single entry point in the **top** layer, greedily walks to the neighbour nearest the query, drops to that same node one layer down, and repeats — coarse long hops up top, fine local refinement at the bottom. It is the graph-theoretic analogue of a **skip list**.

## How it works

HNSW combines two ideas:

- **Navigable small-world (NSW) graphs** — proximity graphs that mix short local links with occasional long-range links, so greedy routing reaches any target in a polylogarithmic number of hops (Kleinberg's small-world result).
- **Hierarchy by exponential sampling** — separating links into layers by *length scale* removes the polylog factor that plagues a flat NSW graph, turning search into $O(\log N)$.

The interactive figure below builds an index over random 2-D points and animates a query descending the stack. Use **New query point** to fire a fresh search, switch between the **Stack 2.5D** and **Flat layer** views, and step through the walk.

## Level assignment

Each inserted node is given a maximum level drawn from a geometric distribution:

$$l = \left\lfloor -\ln(u) \cdot m_L \right\rfloor, \qquad u \sim \mathrm{Uniform}(0,1)$$

The normalization constant $m_L$ controls how fast layers thin out. The choice that minimizes overlap between layers (and the expected number of hops) is

$$m_L = \frac{1}{\ln M}$$

where $M$ is the number of neighbours kept per node. A node at level $l$ participates in layers $0, 1, \dots, l$. Because levels decay geometrically, the expected number of layers is $O(\log N)$ and the top layers stay sparse.

## Search

Searching for the nearest neighbour of $q$ runs in two phases:

1. **Greedy descent (layers $L \to 1$).** Start at the global entry point. At each layer, repeatedly move to whichever neighbour is closer to $q$ until no neighbour improves; then descend to the next layer keeping the current best node as the new start.
2. **Beam search at layer $0$.** Run a best-first search of width $\mathit{ef}$ (the size of the dynamic candidate list). Larger $\mathit{ef}$ explores more of the base graph, trading speed for recall.

The greedy walk turns the global search into a sequence of local decisions; the only state carried between layers is the single best node found so far.

## Complexity

| Quantity | Cost |
|---|---|
| Expected query time | $O(\log N)$ |
| Expected insertion time | $O(\log N)$ |
| Memory | $O(N \cdot M)$ |
| Layers (expected) | $O(\log N)$ |

The key tunables are $M$ (graph degree — higher means better recall and more memory), $\mathit{ef\_construction}$ (build-time candidate list — better graph quality), and $\mathit{ef\_search}$ (query-time candidate list — recall vs. latency). The visualization exposes $N$, $M$, $m_L$, and the search $\mathit{ef}$ directly.

## Connections

HNSW sits at the intersection of small-world network theory and proximity-graph search. The hierarchy mirrors the **skip list**: probabilistic level assignment giving expected-logarithmic traversal, lifted from a linked list onto a graph. The base-layer beam search is a graph-restricted variant of best-first / $k$-nearest-neighbour search. The construction here builds each layer as a symmetric $M$-nearest-neighbour graph for clarity; production implementations (e.g. FAISS, hnswlib, Qdrant) instead insert nodes incrementally with a neighbour-selection heuristic that prunes redundant long edges to keep the graph navigable.

## References

- Malkov, Yu. A. & Yashunin, D. A. (2016). *Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs*. [arXiv:1603.09320](https://arxiv.org/abs/1603.09320) (IEEE TPAMI 2020).
- Malkov, Yu. et al. (2014). *Approximate nearest neighbor algorithm based on navigable small world graphs*. Information Systems.
- Kleinberg, J. (2000). *The small-world phenomenon: an algorithmic perspective*. STOC.
- [Hierarchical navigable small world — Wikipedia](https://en.wikipedia.org/wiki/Hierarchical_navigable_small_world)
