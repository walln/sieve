use sieve::ann_index::ApproximateNearestNeighborsIndex;
use sieve::vector::Vector;

#[test]
fn test_simple_index() {
    let vectors = vec![Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])];
    let ids: Vec<i32> = (0..vectors.len()).map(|i| i as i32).collect();

    let index = ApproximateNearestNeighborsIndex::build(2, 2, &vectors, &ids);

    let query = Vector::new([1.0, 2.0]);
    let results = index.search(query, 2);

    assert!(results.len() == 2, "Should only return 2 results");

    let top_1 = results.get(0).unwrap();
    assert!(
        top_1.vector_id == 0,
        "Top search result should be the first vector"
    );

    assert!(
        top_1.distance == 0.0,
        "Top search result should be the first vector"
    );
}

#[test]
fn test_top_k_index() {
    let vectors = vec![Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])];
    let ids: Vec<i32> = (0..vectors.len()).map(|i| i as i32).collect();

    let index = ApproximateNearestNeighborsIndex::build(2, 2, &vectors, &ids);

    let query = Vector::new([1.0, 2.0]);
    let results = index.search(query, 1);

    assert!(results.len() == 1, "Should only return 1 result");

    let top_1 = results.get(0).unwrap();
    assert!(
        top_1.vector_id == 0,
        "Top search result should be the first vector"
    );

    assert!(
        top_1.distance == 0.0,
        "Top search result should be the first vector"
    );
}
