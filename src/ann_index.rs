use dashmap::DashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashSet;

use crate::hyperplane::HyperPlane;
use crate::tree::{InnerNode, LeafNode, TreeNode};
use crate::vector::Vector;
use rand::prelude::SliceRandom;

/// A search result from an approximate nearest neighbors search
/// Each result contains the vector id, the distance from the query vector, and the vector itself
#[derive(Debug, Clone)]
pub struct ApproximateNearestNeighborsSearchResult<const N: usize> {
    pub vector_id: i32,
    pub distance: f32,
    pub vector: Vector<N>,
}

/// An index of vectors that can be searched for approximate nearest neighbors
/// The index constructs an in-memory tree of the vectors, and searches the tree for the nearest neighbors
pub struct ApproximateNearestNeighborsIndex<const N: usize> {
    vectors: Vec<Vector<N>>,
    ids: Vec<i32>,
    trees: Vec<TreeNode<N>>,
}

impl<const N: usize> ApproximateNearestNeighborsIndex<N> {
    /// Build an index of vectors by constructing a tree of the vectors
    /// The index will contain `num_trees` trees, each with a maximum of `max_size` vectors
    /// The index will deduplicate vectors with the same hashkey
    pub fn build(
        num_trees: i32,
        max_size: i32,
        vectors: &Vec<Vector<N>>,
        vector_ids: &[i32],
    ) -> ApproximateNearestNeighborsIndex<N> {
        let (mut unique_vecs, mut ids) = (vec![], vec![]);
        Self::deduplicate(vectors, vector_ids, &mut unique_vecs, &mut ids);
        let all_indexes: Vec<usize> = (0..unique_vecs.len()).collect();

        let trees = (0..num_trees)
            .into_par_iter()
            .map(|_| Self::build_tree(max_size, &all_indexes, &unique_vecs))
            .collect();

        ApproximateNearestNeighborsIndex {
            trees,
            ids,
            vectors: unique_vecs,
        }
    }

    /// Search the index for the `top_k` approximate nearest neighbors of the `query` vector
    /// Returns a vector of `ApproximateNearestNeighborsSearchResult` structs
    /// The vector is sorted by distance from the query vector (ascending) and limited to `top_k` results
    /// using the squared euclidian distance as the distance metric.
    ///
    /// NOTE:
    /// Search is an approximate nearest neighbors search, and may not return the true nearest neighbors
    /// The search is approximate because the index builds a tree of the vectors, and searches the tree
    /// by constructing hyperplanes during the construction of the tree. The hyperplanes are constructed
    /// based on the number and size of trees allowed. The hyperplanes are constructed by randomly
    /// sampling vectors. Therefore the construction of a tree is not deterministic and for large indicies,
    /// the search may not return the true nearest neighbors.
    pub fn search(
        &self,
        query: Vector<N>,
        top_k: i32,
    ) -> Vec<ApproximateNearestNeighborsSearchResult<N>> {
        let candidates = DashSet::new();
        self.trees.par_iter().for_each(|tree| {
            Self::query_tree(query, top_k, tree, &candidates);
        });
        candidates
            .into_iter()
            .map(|idx| (idx, self.vectors[idx].squared_euclidian_distance(&query)))
            .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .take(top_k as usize)
            .map(|(ids, dis)| ApproximateNearestNeighborsSearchResult {
                vector_id: self.ids[ids],
                distance: dis,
                vector: self.vectors[ids],
            })
            .collect_vec()
    }

    /// Retrieve all vectors in the index, the id of the vector is its index within the returned Vec
    /// since the index is immutable after construction, the id of a vector will not change
    pub fn all_vectors(&self) -> Vec<Vector<N>> {
        self.vectors.clone()
    }

    fn build_tree(max_size: i32, indexes: &Vec<usize>, all_vecs: &Vec<Vector<N>>) -> TreeNode<N> {
        if indexes.len() <= (max_size as usize) {
            return TreeNode::Leaf(Box::new(LeafNode::new(indexes.clone())));
        }
        let (plane, above, below) = Self::build_hyperplane(indexes, all_vecs);
        let node_above = Self::build_tree(max_size, &above, all_vecs);
        let node_below = Self::build_tree(max_size, &below, all_vecs);

        TreeNode::Branch(Box::new(InnerNode::new(plane, node_below, node_above)))
    }

    fn build_hyperplane(
        indexes: &[usize],
        all_vecs: &[Vector<N>],
    ) -> (HyperPlane<N>, Vec<usize>, Vec<usize>) {
        let sample: Vec<_> = indexes
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect();

        // cartesian eq for hyperplane n * (x - x_0) = 0
        // n (normal vector) is the coefs x_1 to x_n
        let (a, b) = (*sample[0], *sample[1]);
        let coefficients = all_vecs[b].sub(&all_vecs[a]);
        let point_on_plane = all_vecs[a].avg(&all_vecs[b]);
        let constant = -coefficients.dot(&point_on_plane);
        let hyperplane = HyperPlane::new(coefficients, constant);

        let (mut above, mut below) = (vec![], vec![]);
        for &id in indexes.iter() {
            if hyperplane.is_point_above(&all_vecs[id]) {
                above.push(id)
            } else {
                below.push(id)
            };
        }
        (hyperplane, above, below)
    }

    fn deduplicate(
        vectors: &Vec<Vector<N>>,
        ids: &[i32],
        dedup_vectors: &mut Vec<Vector<N>>,
        dedup_ids: &mut Vec<i32>,
    ) {
        let mut hashes_seen = HashSet::new();
        for i in 0..vectors.len() {
            let hash_key = vectors[i].hashkey();
            if !hashes_seen.contains(&hash_key) {
                hashes_seen.insert(hash_key);
                dedup_vectors.push(vectors[i]);
                dedup_ids.push(ids[i]);
            }
        }
    }

    fn query_tree(
        query: Vector<N>,
        n: i32,
        tree: &TreeNode<N>,
        candidates: &DashSet<usize>,
    ) -> i32 {
        match tree {
            TreeNode::Leaf(box_leaf) => {
                let leaf_values = &(box_leaf.value());
                let num_candidates_found = min(n as usize, leaf_values.len());
                for item in leaf_values.iter().take(num_candidates_found) {
                    candidates.insert(*item);
                }
                num_candidates_found as i32
            }
            TreeNode::Branch(inner) => {
                let above = (*inner).hyperplane().is_point_above(&query);
                let (main, backup) = match above {
                    true => {
                        let main = inner.right();
                        let backup = inner.left();
                        (main, backup)
                    }
                    false => {
                        let main = inner.left();
                        let backup = inner.right();
                        (main, backup)
                    }
                };
                match Self::query_tree(query, n, main, candidates) {
                    k if k < n => k + Self::query_tree(query, n - k, backup, candidates),
                    k => k,
                }
            }
        }
    }
}
