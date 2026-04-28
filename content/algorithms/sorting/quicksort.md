+++
title = "Quicksort"
date = "2026-01-20"
tags = ["sorting", "divide-and-conquer", "comparison-sort"]
category = "sorting"
difficulty = "intermediate"
interactive = "algo_viz::quicksort"
+++

**Quicksort** is a divide-and-conquer sorting algorithm with average-case $O(n \log n)$ time complexity.

## Algorithm

1. Choose a **pivot** element from the array
2. **Partition**: rearrange so elements $\leq$ pivot are on the left, elements $>$ pivot are on the right
3. **Recurse** on left and right sub-arrays

## Complexity

| Case | Time | Space |
|------|------|-------|
| Best | $O(n \log n)$ | $O(\log n)$ |
| Average | $O(n \log n)$ | $O(\log n)$ |
| Worst | $O(n^2)$ | $O(n)$ |

The worst case occurs when the pivot is always the smallest or largest element (already sorted input with naive pivot selection).

## Partition scheme (Lomuto)

```
partition(arr, lo, hi):
    pivot = arr[hi]
    i = lo
    for j in lo..hi:
        if arr[j] <= pivot:
            swap(arr[i], arr[j])
            i += 1
    swap(arr[i], arr[hi])
    return i
```
