// Refactor to some common abstraction with BT and BST?

use crate::binary_tree::bst::{DeleteError, InsertError, ParentError, SearchTree};
use crate::binary_tree::bt::{AlreadyExists, Tree};

pub enum Color {
    Red,
    Black,
}

pub type RBLink<T> = Option<Box<RBNode<T>>>;

struct RBNode<T> {
    pub value: T,
    pub left: RBLink<T>,
    pub right: RBLink<T>,
    pub parent: RBLink<T>,
    pub color: Color,
}

impl<T> RBNode<T> {
    // basically just RBTree should create RBNode because it needs parent link and decide on color
    fn new(value: T, color: Color, parent: RBLink<T>) -> RBNode<T> {
        RBNode {
            value,
            left: None,
            right: None,
            color: color,
            parent: parent,
        }
    }

    fn assign_left(&mut self, value: T) -> Result<&mut RBNode<T>, AlreadyExists> {
        if self.left.is_some() {
            return Err(AlreadyExists::LeftTreeExists);
        }

        self.left = Some(Box::new(RBNode {
            value,
            left: None,
            right: None,
            color: Color::Red,
            parent: None,
        }));
        Ok(self.left.as_mut().unwrap())
    }

    fn assign_right(&mut self, value: T) -> Result<&mut RBNode<T>, AlreadyExists> {
        if self.right.is_some() {
            return Err(AlreadyExists::RightTreeExists);
        }

        self.right = Some(Box::new(RBNode {
            value,
            left: None,
            right: None,
            color: Color::Red,
            parent: None,
        }));
        Ok(self.right.as_mut().unwrap())
    }

    pub fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |node| node.height() + 1);
        let right_height = self.right.as_ref().map_or(0, |node| node.height() + 1);

        left_height.max(right_height)
    }
}
