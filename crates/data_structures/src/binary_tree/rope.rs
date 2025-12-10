use core::fmt;

use crate::binary_tree::errors::{DeleteError, InsertError, TreeCoreError};
use crate::binary_tree::nodes::{Link, RopeNode};
use crate::binary_tree::trees::Tree;

pub struct Rope {
    root: Link<RopeNode>,
}

impl Tree for Rope {
    type Node = RopeNode;

    fn get_root(&self) -> Option<&Self::Node> {
        self.root.as_deref()
    }
    fn get_mut_root(&mut self) -> Option<&mut Self::Node> {
        self.root.as_deref_mut()
    }
}

impl Rope {
    pub fn new(text: &str) -> Rope {
        Rope {
            root: Some(Box::new(RopeNode::new(text))),
        }
    }

    fn tree_size(&self) -> Option<usize> {
        self.root.as_ref().map(|root| *root.size() as usize)
    }

    pub fn insert(mut self, text: &str, index: usize) -> Result<Self, InsertError> {
        if self.root.is_none() {
            return Err(InsertError::Core(TreeCoreError::NoRootNode));
        }

        self.get_root().unwrap().check_index_inclusive(index)?;

        let middle = Some(Box::new(RopeNode::new(text)));
        let (mut left, right) = RopeNode::split(self.root, index)?;

        left = RopeNode::join(left, middle);
        self.root = RopeNode::join(left, right);

        Ok(self)
    }

    pub fn append(self, text: &str) -> Result<Self, InsertError> {
        let length = self.tree_size();

        if length.is_none() {
            return Err(InsertError::Core(TreeCoreError::NoRootNode));
        }

        let result = self.insert(text, length.unwrap())?;
        Ok(result)
    }

    pub fn delete(mut self, start: usize, end: usize) -> Result<Self, DeleteError> {
        let length = self.tree_size();

        if length.is_none() {
            return Err(DeleteError::Core(TreeCoreError::NoRootNode));
        }

        self.get_root().unwrap().check_index_exclusive(start)?;

        let delete_range = end.min(length.unwrap() - start);

        let (left, rest) = RopeNode::split(self.root, start)?;
        let (_middle, right) = RopeNode::split(rest, delete_range)?;

        self.root = RopeNode::join(left, right);

        Ok(self)
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_parts: Vec<&str> = self
            .inorder()
            .expect("Inorder traversal failure during display in `Rope`")
            .map(|s| s.as_str())
            .collect();

        write!(f, "{}", str_parts.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_root_wo_change() {
        let mut rope = Rope::new("abc");

        if let Some(root) = rope.get_root() {
            let size = root.size();
            assert_eq!(size, &3);
        }

        if let Some(root) = rope.get_mut_root() {
            root.recalculate_size();

            let size = root.size();
            assert_eq!(size, &3);
        }
    }

    #[test]
    fn size_of_root_w_change() {
        let mut rope = Rope::new("abc");

        if let Some(root) = rope.get_root() {
            let size1 = root.size();
            assert_eq!(size1, &3);
        }

        if let Some(root) = rope.get_mut_root() {
            root.text = "abcd".to_string();
            root.recalculate_size();

            let size2 = root.size();

            assert_eq!(size2, &4);
        }
    }

    #[test]
    fn inorder_simple_rope() {
        let tree = Rope::new("abc");

        let got: Vec<&String> = tree.inorder().expect("inorder").collect();

        assert_eq!(got, vec!["abc"]);
    }

    #[test]
    fn append_rope() {
        let mut rope = Rope::new("a");
        rope = rope
            .append("b")
            .expect("Error appending `b` into rope tree");

        let text = rope.to_string();
        assert_eq!(text, "ab")
    }

    #[test]
    fn insert_rope() {
        let mut rope = Rope::new("ac");
        rope = rope
            .insert("b", 1)
            .expect("Error inserting `b` into rope tree");

        let text = rope.to_string();
        assert_eq!(text, "abc")
    }

    #[test]
    fn append_without_root() {
        let rope = Rope { root: None };

        let result = rope.append("");
        assert!(result.is_err())
    }

    #[test]
    fn insert_without_root() {
        let rope = Rope { root: None };

        let result = rope.insert("", 0);
        assert!(result.is_err())
    }

    #[test]
    fn delete_range() {
        let mut rope = Rope::new("Rust is awesome!");
        let start: usize = 7;
        let end = rope.tree_size().unwrap();

        rope = rope.delete(start, end).unwrap();

        assert_eq!(rope.to_string(), "Rust is")
    }
}
