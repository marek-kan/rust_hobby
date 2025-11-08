use std::{cmp::Ordering, fmt::Display};

use crate::binary_tree::bt::{Tree, Link, Node};

#[derive(Debug)]
pub enum ParentError { ParentNodeNotFound }


enum SearchStep { Left, Right, Here }


pub trait SearchTree<T>: Tree<T> {
    fn search<'a>(&'a mut self, node: &Node<T>) -> &'a mut Link<T>;
    fn insert(&mut self, node: Node<T>);
    fn delete(&mut self, node: &Node<T>);
}


pub struct BinarySearchTree<T> {
    pub root: Link<T>
}

impl<T: Ord + Display> BinarySearchTree<T> {

    pub fn new(node: Node<T>) -> BinarySearchTree<T> {
        BinarySearchTree { root: Some(Box::new(node)) }
    }

    // fn check_child(child: &Link<T>, reference_node: &Node<T>) -> bool {
    //     match child.as_deref() {
    //         Some(node) => {
    //             if node.value == reference_node.value { true }
    //             else { false }
    //         },
    //         None => false
    //     }
    // }

    // pub fn get_parent(&mut self, node: &Node<T>) -> Result<&mut Node<T>, ParentError> {
    //     let mut stack = vec![&mut self.root];

    //     loop {
    //         if let Some(current) = stack.pop() {
    //             if Self::check_child(&current.left, &node) {
    //                 return Ok(current)
    //             }
                
    //             else if Self::check_child(&current.right, &node) {
    //                 return Ok(current)
    //             }
                
    //             else {
    //                 if let Some(right) = current.right.as_deref_mut() {
    //                     stack.push(right);
    //                 }
                    
    //                 if let Some(left) = current.left.as_deref_mut() {
    //                     stack.push(left);
    //                 }
    //             }
    //         }
    //         else {
    //             return Err(ParentError::ParentNodeNotFound)
    //         }
    //     }
    // }
 
    fn _search<'a>(mut link: &'a mut Link<T>, key: &T) -> &'a mut Link<T> {
        loop {
            // Immutable peek so we don't risk & and &mut overlapping
            let step = match link.as_ref() {
                None => SearchStep::Here,
                
                Some(n) => {
                    if key < &n.value {
                        if n.left.is_none() { SearchStep::Here } else { SearchStep::Left }
                    } 
                    
                    else if key > &n.value {
                        if n.right.is_none() { SearchStep::Here } else { SearchStep::Right }
                    } 
                    
                    else { SearchStep::Here }
                }
            };

            // mutable action
            match step {
                SearchStep::Here  => return link,
                SearchStep::Left  => { let n = link.as_mut().unwrap(); link = &mut n.left;  }
                SearchStep::Right => { let n = link.as_mut().unwrap(); link = &mut n.right; }
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
    
    fn delete(&mut self, node: &Node<T>) {

        let link = self.search(node);

        match link {
            Some(node) => {
                if node.left.is_none() & node.right.is_none() {
                    *link = None
                }
                else {
                    panic!("Not implemented yet!")
                }
            },
            None => println!("Failed to delete node: {}", node.value)
        }
    }

    fn insert(&mut self, node: Node<T>) {
        let parent = self.search(&node);
        
        {
            println!("About to insert node under {}", &parent.as_deref().unwrap().value);
        }

        match parent {
            Some(n) => match node.value.cmp(&n.value) {
                Ordering::Equal => panic!("Parent and inserting node have identical values!"),
                
                Ordering::Greater => {
                    let _right = match n.assign_right(node.value) {
                        Ok(node) => println!("Successfully created right node {}", node.value),
                        Err(_) => panic!("Right node already exists!"),
                    };
                },
                
                Ordering::Less => {
                    let _left = match n.assign_left(node.value) {
                        Ok(node) => println!("Successfully created left node {}", node.value),
                        Err(_) => panic!("Left node already exists!"),
                    };
                }
            },
            None => ()
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
