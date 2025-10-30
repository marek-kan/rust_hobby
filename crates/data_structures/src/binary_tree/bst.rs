use crate::binary_tree::bt::{self, Tree};

trait SearchTree<T>: bt::Tree<T> {
    fn search<'a>(&'a mut self, node: &'a bt::Node<T>) -> &'a mut bt::Node<T>;
    fn insert(&mut self, node: bt::Node<T>);
    fn delete(&mut self, node: &bt::Node<T>);
}

pub struct BinarySearchTree<T> {
    root: bt::Node<T>
}

impl<T> BinarySearchTree<T> {
    fn _search<'a>(node: &'a mut bt::Node<T>, target: &'a bt::Node<T>) -> &'a mut bt::Node<T> {
        node
    }
}

impl<T> bt::Tree<T> for BinarySearchTree<T> {
    fn get_root(&self) -> Option<&bt::Node<T>> {
        Some(&self.root)
    }
    fn get_mut_root(&mut self) -> Option<&mut bt::Node<T>> {
        Some(&mut self.root)
    }
}

impl<T> SearchTree<T> for BinarySearchTree<T> {

    fn search<'a>(&'a mut self, node: &'a bt::Node<T>) -> &'a mut bt::Node<T> {
       
       let root = self.get_mut_root().unwrap();
       Self::_search(root, node)
    }
    
    fn delete(&mut self, node: &bt::Node<T>) {
        
    }

    fn insert(&mut self, node: bt::Node<T>) {
        
    }
}



pub fn bst() {
    println!("Binary Search Tree")
}
