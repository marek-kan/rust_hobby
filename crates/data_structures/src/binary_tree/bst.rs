use log::info;
use std::{cmp::Ordering, fmt::Display};

use crate::binary_tree::errors::{DeleteError, InsertError};
use crate::binary_tree::nodes::{BasicNode, Link, Node};
use crate::binary_tree::trees::{SearchTree, Tree};

enum SearchStep {
    Left,
    Right,
    Here,
}

pub struct BinarySearchTree<T> {
    pub root: Link<Node<T>>,
}

impl<T> Tree for BinarySearchTree<T> {
    type Node = Node<T>;

    fn get_root(&self) -> Option<&Self::Node> {
        self.root.as_deref()
    }

    fn get_mut_root(&mut self) -> Option<&mut Node<T>> {
        self.root.as_deref_mut()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new(node: Node<T>) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Some(Box::new(node)),
        }
    }

    fn _search<'a>(mut link: &'a mut Link<Node<T>>, key: &T) -> &'a mut Link<Node<T>> {
        loop {
            // Immutable peek so we don't risk & and &mut overlapping
            let step = match link.as_ref() {
                None => SearchStep::Here,

                Some(n) => {
                    if key < n.value() {
                        if n.left().is_none() {
                            SearchStep::Here
                        } else {
                            SearchStep::Left
                        }
                    } else if key > n.value() {
                        if n.right().is_none() {
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

impl<T: Ord + Display> SearchTree for BinarySearchTree<T> {
    type Value = T;

    fn search<'a>(&'a mut self, value: &T) -> &'a mut Link<Node<T>> {
        Self::_search(&mut self.root, value)
    }

    fn delete(&mut self, value: &T) -> Result<(), DeleteError> {
        let link = self.search(value);

        match link {
            Some(node) => {
                let leaf_states = (node.left().is_none(), node.right().is_none());

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

    fn insert(&mut self, value: T) -> Result<(), InsertError> {
        let parent = self.search(&value);

        if let Some(p) = parent.as_ref() {
            info!("About to insert node under {}", p.value());
        }

        match parent {
            Some(n) => match value.cmp(n.value()) {
                Ordering::Equal => Err(InsertError::ParentHasSameValue),

                Ordering::Greater => {
                    let _right: &mut Node<T> = n.assign_right(value)?;
                    Ok(())
                }

                Ordering::Less => {
                    let _left: &mut Node<T> = n.assign_left(value)?;
                    Ok(())
                }
            },
            None => Err(InsertError::ParentNotFound),
        }
    }
}

/// Builds sample BinarySearchTree for examples/tests. Shouldn't be run at any other context
pub fn build_sample_tree() -> BinarySearchTree<i64> {
    let mut tree = BinarySearchTree::new(Node::new(4));

    {
        let left = tree
            .get_mut_root()
            .expect("No root!")
            .assign_left(2)
            .expect("Failed to assign node");
        left.assign_left(1).expect("Failed to assign node");
        left.assign_right(3).expect("Failed to assign node");
    }

    {
        let right = tree
            .get_mut_root()
            .expect("No root!")
            .assign_right(6)
            .expect("Failed to assign node");
        let _left = right.assign_left(5).expect("Failed to assign node");
        let right = right.assign_right(7).expect("Failed to assign node");
        right.assign_right(8).expect("Failed to assign node");
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_3() {
        let mut tree = build_sample_tree();
        let result = tree.search(&3).as_ref().expect("Search failed");

        assert_eq!(result.value, 3);
    }

    #[test]
    fn find_100() {
        let mut tree = build_sample_tree();
        let result = tree.search(&100).as_ref().expect("Search failed");

        assert_eq!(result.value, 8);
    }

    #[test]
    fn inorder_after_insert_0() {
        let mut tree = build_sample_tree();

        tree.insert(0).expect("insert(0)");

        let got: Vec<i64> = tree.inorder().expect("inorder").copied().collect();

        assert_eq!(got, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn preorder_after_delete_6() {
        let mut tree = build_sample_tree();

        tree.delete(&6).expect("delete(6)");

        let got: Vec<i64> = tree.preorder().expect("preorder").copied().collect();

        assert_eq!(got, vec![4, 2, 1, 3, 7, 5, 8]);
    }

    #[test]
    fn inorder_after_delete_6_is_sorted_without_6() {
        let mut tree = build_sample_tree();
        tree.delete(&6).expect("delete(6)");

        let got: Vec<i64> = tree.inorder().expect("inorder").copied().collect();

        assert_eq!(got, vec![1, 2, 3, 4, 5, 7, 8]);
    }
}
