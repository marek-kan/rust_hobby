use log::info;
use std::{cmp::Ordering, fmt::Display};

use crate::binary_tree::errors::{DeleteError, InsertError};
use crate::binary_tree::nodes::Node;
use crate::binary_tree::nodes::{BasicNode, Color, Link, RBNode};
use crate::binary_tree::trees::{SearchTree, Tree};

pub struct RedBlackTree<T> {
    root: Link<RBNode<T>>,
}

impl<T> RedBlackTree<T> {
    pub fn add_root(&mut self, value: T) {
        self.root = Some(Box::new(RBNode::new(value, Color::Black, None)))
    }
}
