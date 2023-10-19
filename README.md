# Sieve

Sieve is a toy implementation of Approximate Nearest Neighbors search. This index works by building an in-memory
tree of the vectorsm randomly selecting vectors during the construction of the index and creating hyperplanes to use to quickly search the vector space. Vectors can then be queried using squared euclidian distance to find similar vectors within the vector space.

To get started just install this crate and create an index.

```rust
    let vectors = vec![Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])];
    let ids: Vec<i32> = (0..vectors.len()).map(|i| i as i32).collect();

    let index = ApproximateNearestNeighborsIndex::build(2, 2, &vectors, &ids);

    let query = Vector::new([1.0, 2.0]);
    let top_k = 1;
    let results = index.search(query, top_k);
```
