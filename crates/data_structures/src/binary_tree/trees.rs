use crate::binary_tree::errors::{DeleteError, InsertError, TreeCoreError};
use crate::binary_tree::iterators::*;
use crate::binary_tree::nodes::{BasicNode, Link};

pub trait Tree {
    type Node: BasicNode;

    fn get_root(&self) -> Option<&Self::Node>;

    fn get_mut_root(&mut self) -> Option<&mut Self::Node>;

    fn _depth(&self, node: &Self::Node, target: &Self::Node, current_depth: usize) -> usize {
        if std::ptr::eq(node, target) {
            return current_depth;
        }

        let left_depth = node
            .left()
            .map_or(0, |n| self._depth(n, target, current_depth + 1));
        let right_depth = node
            .right()
            .map_or(0, |n| self._depth(n, target, current_depth + 1));

        left_depth.max(right_depth)
    }

    fn depth(&self, target: &Self::Node) -> Option<usize> {
        self.get_root().map(|root| self._depth(root, target, 0))
    }

    fn inorder(&self) -> Result<InOrder<'_, Self::Node>, TreeCoreError> {
        match self.get_root() {
            Some(root) => Ok(InOrder::new(root)),
            _ => Err(TreeCoreError::NoRootNode),
        }
    }

    fn preorder(&self) -> Result<PreOrder<'_, Self::Node>, TreeCoreError> {
        match self.get_root() {
            Some(root) => Ok(PreOrder::new(root)),
            _ => Err(TreeCoreError::NoRootNode),
        }
    }

    fn postorder(&self) -> Result<PostOrderStreaming<'_, Self::Node>, TreeCoreError> {
        match self.get_root() {
            Some(root) => Ok(PostOrderStreaming::new(root)),
            _ => Err(TreeCoreError::NoRootNode),
        }
    }

    fn postorder_precomp(&self) -> Result<PostOrderPreComputed<'_, Self::Node>, TreeCoreError> {
        // Precomputes whole stack, next() is very cheap, good if you need index into order repeatedly. However, `postorder`
        // is prefered for most use-cases.
        match self.get_root() {
            Some(root) => Ok(PostOrderPreComputed::new(root)),
            _ => Err(TreeCoreError::NoRootNode),
        }
    }
}

pub trait SearchTree: Tree {
    type Value: Ord;

    fn search<'a>(&'a mut self, value: &Self::Value) -> &'a mut Link<<Self as Tree>::Node>;
    fn insert(&mut self, value: Self::Value) -> Result<(), InsertError>;
    fn delete(&mut self, value: &Self::Value) -> Result<(), DeleteError>;
}
