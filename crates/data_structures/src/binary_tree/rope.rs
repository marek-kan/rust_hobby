use crate::binary_tree::nodes::{BasicNode, Link, RopeNode};
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
    fn new(text: &str) -> Rope {
        Rope {
            root: Some(Box::new(RopeNode::new(text))),
        }
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
            root.recalculate();

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
            root.recalculate();

            let size2 = root.size();

            assert_eq!(size2, &4);
        }
    }
}
