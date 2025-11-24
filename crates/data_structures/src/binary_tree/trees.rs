use crate::binary_tree::errors::DoesntExist;
use crate::binary_tree::iterators::*;
use crate::binary_tree::nodes::BasicNode;

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

    fn depth(&self, target: &Self::Node) -> usize {
        match self.get_root() {
            Some(root) => self._depth(root, target, 0),
            _ => {
                println!("No root node!");
                0
            }
        }
    }

    fn inorder(&self) -> Result<InOrder<'_, Self::Node>, DoesntExist> {
        match self.get_root() {
            Some(root) => Ok(InOrder::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn preorder(&self) -> Result<PreOrder<'_, Self::Node>, DoesntExist> {
        match self.get_root() {
            Some(root) => Ok(PreOrder::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn postorder(&self) -> Result<PostOrderStraming<'_, Self::Node>, DoesntExist> {
        match self.get_root() {
            Some(root) => Ok(PostOrderStraming::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn postorder_precomp(&self) -> Result<PostOrderPreComputed<'_, Self::Node>, DoesntExist> {
        // Precomputes whole stack, next() is very cheap, good if you need index into order repeatedly. However, `postorder`
        // is prefered for most use-cases.
        match self.get_root() {
            Some(root) => Ok(PostOrderPreComputed::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }
}
