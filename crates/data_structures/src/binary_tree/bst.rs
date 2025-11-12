use std::{cmp::Ordering, fmt::Display};

use crate::binary_tree::bt::{AlreadyExists, Link, Node, Tree};

#[derive(Debug)]
pub enum ParentError {
    ParentNodeNotFound,
}

#[derive(Debug)]
pub enum InsertError {
    LeftAlreadyExists,
    RightAlreadyExists,
    ParentHasSameValue,
    ParentNotFound,
}

impl From<AlreadyExists> for InsertError {
    fn from(error: AlreadyExists) -> Self {
        match error {
            AlreadyExists::LeftTreeExists => InsertError::LeftAlreadyExists,
            AlreadyExists::RightTreeExists => InsertError::RightAlreadyExists,
        }
    }
}

#[derive(Debug)]
pub enum DeleteError {
    FailedToDeleteNode,
}

enum SearchStep {
    Left,
    Right,
    Here,
}

pub trait SearchTree<T>: Tree<T> {
    fn search<'a>(&'a mut self, node: &Node<T>) -> &'a mut Link<T>;
    fn insert(&mut self, node: Node<T>) -> Result<(), InsertError>;
    fn delete(&mut self, node: &Node<T>) -> Result<(), DeleteError>;
}

pub struct BinarySearchTree<T> {
    pub root: Link<T>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new(node: Node<T>) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Some(Box::new(node)),
        }
    }

    fn _search<'a>(mut link: &'a mut Link<T>, key: &T) -> &'a mut Link<T> {
        loop {
            // Immutable peek so we don't risk & and &mut overlapping
            let step = match link.as_ref() {
                None => SearchStep::Here,

                Some(n) => {
                    if key < &n.value {
                        if n.left.is_none() {
                            SearchStep::Here
                        } else {
                            SearchStep::Left
                        }
                    } else if key > &n.value {
                        if n.right.is_none() {
                            SearchStep::Here
                        } else {
                            SearchStep::Right
                        }
                    } else {
                        SearchStep::Here
                    }
                }
            };

            // mutable action
            match step {
                SearchStep::Here => return link,
                SearchStep::Left => {
                    let n = link.as_mut().unwrap();
                    link = &mut n.left;
                }
                SearchStep::Right => {
                    let n = link.as_mut().unwrap();
                    link = &mut n.right;
                }
            }
        }
    }
}

impl<T> Tree<T> for BinarySearchTree<T> {
    fn get_root(&self) -> Option<&Node<T>> {
        self.root.as_deref()
    }

    fn get_mut_root(&mut self) -> Option<&mut Node<T>> {
        self.root.as_deref_mut()
    }
}

impl<T: Ord + Display> SearchTree<T> for BinarySearchTree<T> {
    fn search<'a>(&'a mut self, node: &Node<T>) -> &'a mut Link<T> {
        Self::_search(&mut self.root, &node.value)
    }

    fn delete(&mut self, node: &Node<T>) -> Result<(), DeleteError> {
        let link = self.search(node);

        match link {
            Some(node) => {
                let leaf_states = (node.left.is_none(), node.right.is_none());

                match leaf_states {
                    (true, true) => {
                        *link = None;
                        Ok(())
                    }

                    (false, false) => {
                        let successor = Self::_search(&mut node.right, &node.value);

                        match successor {
                            Some(s) => {
                                // swap values without copy/clone.
                                std::mem::swap(&mut node.value, &mut s.value);

                                *successor = s.right.take(); // None or Some

                                Ok(())
                            }
                            None => Err(DeleteError::FailedToDeleteNode),
                        }
                    }

                    (false, true) => {
                        *link = node.left.take();
                        Ok(())
                    }

                    (true, false) => {
                        *link = node.right.take();
                        Ok(())
                    }
                }
            }

            None => Err(DeleteError::FailedToDeleteNode),
        }
    }

    fn insert(&mut self, node: Node<T>) -> Result<(), InsertError> {
        let parent = self.search(&node);

        if let Some(p) = parent.as_ref() {
            println!("About to insert node under {}", p.value);
        }

        match parent {
            Some(n) => match node.value.cmp(&n.value) {
                Ordering::Equal => Err(InsertError::ParentHasSameValue),

                Ordering::Greater => {
                    let _right: &mut Node<T> = n.assign_right(node.value)?;
                    Ok(())
                }

                Ordering::Less => {
                    let _left: &mut Node<T> = n.assign_left(node.value)?;
                    Ok(())
                }
            },
            None => Err(InsertError::ParentNotFound),
        }
    }
}

/// Builds sample BinarySearchTree for examples.
pub fn example_bst() -> BinarySearchTree<i64> {
    let mut tree = BinarySearchTree::new(Node::new(4));

    {
        let left = tree.get_mut_root().unwrap().assign_left(2).unwrap();
        left.assign_left(1).unwrap();
        left.assign_right(3).unwrap();
    }

    {
        let right = tree.get_mut_root().unwrap().assign_right(6).unwrap();
        let _left = right.assign_left(5).unwrap();
        let right = right.assign_right(7).unwrap();
        right.assign_right(8).unwrap();
    }

    tree
}
