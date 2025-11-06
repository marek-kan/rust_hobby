use std::{cmp::Ordering, fmt::Display};

use crate::binary_tree::bt::{Tree, Link, Node};

pub trait SearchTree<T>: Tree<T> {
    fn search<'a>(&'a mut self, node: &Node<T>) -> &'a mut Node<T>;
    fn insert(&mut self, node: Node<T>);
    fn delete(&mut self, node: &Node<T>);
}

pub struct BinarySearchTree<T> {
    root: Node<T>
}

impl<T: Ord> BinarySearchTree<T> {

    pub fn new(node: Node<T>) -> BinarySearchTree<T> {
        BinarySearchTree { root: node }
    }
 
    fn _search<'a>(link: &'a mut Link<T>, key: &T) -> Option<&'a mut Node<T>> {

        match link {
            Some(node) => match key.cmp(&node.value) {
                
                Ordering::Less => {
                    if node.left.is_none() {
                        return Some(node.as_mut())
                    } 
                    else {
                        Self::_search(&mut node.left, key)
                    }
                },
                
                Ordering::Greater => {
                    if node.right.is_none() {
                        return Some(node.as_mut())
                    }
                    else {
                        Self::_search(&mut node.right, key)
                    }
                },

                Ordering::Equal => Some(node.as_mut()),
            },
            None => None,
        }
    }
}

impl<T> Tree<T> for BinarySearchTree<T> {
    fn get_root(&self) -> Option<&Node<T>> {
        Some(&self.root)
    }
    fn get_mut_root(&mut self) -> Option<&mut Node<T>> {
        Some(&mut self.root)
    }
}

impl<T: Ord + Display> SearchTree<T> for BinarySearchTree<T> {
    
    fn search<'a>(&'a mut self, node: &Node<T>) -> &'a mut Node<T> {
        let key = &node.value;
        
        let result = match key.cmp(&self.root.value) {
            Ordering::Equal => Some(&mut self.root),
            
            Ordering::Less => {
                if self.root.left.is_none() {
                    Some(&mut self.root)
                } 
                else {
                   Self::_search(&mut self.root.left, key)
                }
            },
            
            Ordering::Greater => {
                if self.root.right.is_none() {
                    Some(&mut self.root)
                }
                else {
                    Self::_search(&mut self.root.right, key)
                }
            },
        };

        result.unwrap()
    }
    
    fn delete(&mut self, node: &Node<T>) {
        
    }

    fn insert(&mut self, node: Node<T>) {
        let parent: &mut Node<T> = self.search(&node);
        {
            println!("About to insert node under {}", &parent.value);
        }

        match node.value.cmp(&parent.value) {
            Ordering::Equal => panic!("Parent and inserting node have identical values!"),
            
            Ordering::Less => {
                let _left = match parent.assign_left(node.value) {
                    Ok(node) => println!("Successfully created left node {}", node.value),
                    Err(_) => panic!("Left node already exists!"),
                };
            },
            
            Ordering::Greater => {
                let _right = match parent.assign_right(node.value) {
                    Ok(node) => println!("Successfully created right node {}", node.value),
                    Err(_) => panic!("Right node already exists!"),
                };
            },
        }
    }
}


/// Builds sample BinarySearchTree for examples.
pub fn example_bst() -> BinarySearchTree<i64> {

    let mut tree = BinarySearchTree::new(Node::new(4));

    {
        let left = tree.root.assign_left(2).unwrap();
        left.assign_left(1).unwrap();
        left.assign_right(3).unwrap();
    }

    {
        let right = tree.root.assign_right(6).unwrap();
        let _left = right.assign_left(5).unwrap();
        let right = right.assign_right(7).unwrap();
        right.assign_right(8).unwrap();
    }

    tree
}
