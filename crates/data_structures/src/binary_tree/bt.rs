use crate::binary_tree::errors::AlreadyExists;
use crate::binary_tree::nodes::{BasicNode, Link};
use crate::binary_tree::trees::Tree;

pub struct Node<T> {
    pub value: T,
    pub left: Link<Node<T>>,
    pub right: Link<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BasicNode for Node<T> {
    type N = Node<T>;
    type V = T;

    fn assign_left(&mut self, value: Self::V) -> Result<&mut Self::N, AlreadyExists> {
        if self.left.is_some() {
            return Err(AlreadyExists::LeftTreeExists);
        }

        self.left = Some(Box::new(Node::new(value)));

        Ok(self.left.as_mut().unwrap())
    }

    fn assign_right(&mut self, value: Self::V) -> Result<&mut Self::N, AlreadyExists> {
        if self.right.is_some() {
            return Err(AlreadyExists::LeftTreeExists);
        }

        self.right = Some(Box::new(Node::new(value)));

        Ok(self.right.as_mut().unwrap())
    }

    fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }

    fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    fn value(&self) -> &Self::V {
        &self.value
    }
}

pub struct BinaryTree<T> {
    pub root: Node<T>,
}

impl<T> BinaryTree<T> {
    pub fn new(node: Node<T>) -> BinaryTree<T> {
        BinaryTree { root: node }
    }
}

impl<T> Tree for BinaryTree<T> {
    type Node = Node<T>;

    fn get_root(&self) -> Option<&Self::Node> {
        Some(&self.root)
    }

    fn get_mut_root(&mut self) -> Option<&mut Self::Node> {
        Some(&mut self.root)
    }
}

/// Builds sample BinaryTree for examples. Shouldn't be run at any other context
pub fn build_sample_tree() -> BinaryTree<i64> {
    let mut tree = BinaryTree::new(Node::new(1));

    // left branch
    {
        let left = tree.root.assign_left(2).unwrap();
        left.assign_left(3).unwrap();
        left.assign_right(4).unwrap();
    }

    // right branch
    {
        let right = tree.root.assign_right(5).unwrap();
        let right = right.assign_right(6).unwrap();
        right.assign_right(7).unwrap();
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_simple_tree() -> BinaryTree<i64> {
        let mut tree = BinaryTree::new(Node::new(1));

        {
            let root = tree
                .get_mut_root()
                .expect("Failed to retrieve the root node!");
            let l2 = root.assign_left(2).expect("root.left(2)");
            let l3 = l2.assign_left(3).expect("root.left.left(3)");
            let _r4 = l3.assign_right(4).expect("root.left.left.right(4)");
        }

        tree
    }

    #[test]
    fn root_depth() {
        let tree = build_sample_tree();
        let root = tree.get_root().expect("root exists");
        assert_eq!(tree.depth(root), 0);
    }

    #[test]
    fn height_of_tree() {
        let tree = build_simple_tree();
        let height = tree.get_root().expect("root exists").height();
        assert_eq!(height, 3);
    }

    #[test]
    fn second_left_value_depth() {
        let tree = build_simple_tree();

        let second_left = tree
            .get_root()
            .and_then(|n| n.left.as_deref())
            .and_then(|n| n.left.as_deref())
            .expect("Failed to retrieve second left node");

        let depth = tree.depth(second_left);

        assert_eq!(second_left.value, 3);
        assert_eq!(depth, 2);
    }

    #[test]
    fn inorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .inorder()
            .expect("Failed to traverse in-order")
            .copied()
            .collect();

        assert_eq!(result, vec![3, 4, 2, 1]);
    }

    #[test]
    fn preorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .preorder()
            .expect("Failed to traverse pre-order")
            .copied()
            .collect();

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn postorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .postorder()
            .expect("Failed to traverse post-order")
            .copied()
            .collect();

        assert_eq!(result, vec![4, 3, 2, 1]);
    }
}
