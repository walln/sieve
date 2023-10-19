use crate::hyperplane::HyperPlane;

pub(crate) enum TreeNode<const N: usize> {
    Branch(Box<InnerNode<N>>),
    Leaf(Box<LeafNode<N>>),
}

pub(crate) struct LeafNode<const N: usize> {
    value: Vec<usize>,
}

impl<const N: usize> LeafNode<N> {
    pub fn new(value: Vec<usize>) -> Self {
        Self { value }
    }

    pub fn value(&self) -> Vec<usize> {
        self.value.clone()
    }
}

pub(crate) struct InnerNode<const N: usize> {
    hyperplane: HyperPlane<N>,
    left_node: TreeNode<N>,
    right_node: TreeNode<N>,
}

impl<const N: usize> InnerNode<N> {
    pub fn new(hyperplane: HyperPlane<N>, left_node: TreeNode<N>, right_node: TreeNode<N>) -> Self {
        Self {
            hyperplane,
            left_node,
            right_node,
        }
    }

    pub fn hyperplane(&self) -> &HyperPlane<N> {
        &self.hyperplane
    }

    pub fn left(&self) -> &TreeNode<N> {
        &self.left_node
    }

    pub fn right(&self) -> &TreeNode<N> {
        &self.right_node
    }
}
