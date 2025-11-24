use crate::binary_tree::errors::AlreadyExists;

pub trait BasicNode {
    type V;
    type N;

    fn assign_left(&mut self, value: Self::V) -> Result<&mut Self::N, AlreadyExists>;
    fn assign_right(&mut self, value: Self::V) -> Result<&mut Self::N, AlreadyExists>;
    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn value(&self) -> &Self::V;

    fn height(&self) -> usize {
        let left_height = self.left().map_or(0, |node| node.height() + 1);
        let right_height = self.right().map_or(0, |node| node.height() + 1);

        left_height.max(right_height)
    }
}

pub type Link<Node> = Option<Box<Node>>;
