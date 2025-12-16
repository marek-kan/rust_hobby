use crate::binary_tree::errors::{IndexError, SplitError, TreeCoreError};

pub type Link<T> = Option<Box<T>>;

pub enum Color {
    Red,
    Black,
}

pub trait BasicNode {
    type V;
    type N;

    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn value(&self) -> &Self::V;

    fn height(&self) -> usize {
        let left_height = self.left().map_or(0, |node| node.height() + 1);
        let right_height = self.right().map_or(0, |node| node.height() + 1);

        left_height.max(right_height)
    }
}

pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) left: Link<Node<T>>,
    pub(crate) right: Link<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn assign_left(&mut self, value: T) -> Result<&mut Node<T>, TreeCoreError> {
        if self.left.is_some() {
            return Err(TreeCoreError::LeftTreeExists);
        }

        self.left = Some(Box::new(Node::new(value)));

        Ok(self.left.as_mut().unwrap())
    }

    pub fn assign_right(&mut self, value: T) -> Result<&mut Node<T>, TreeCoreError> {
        if self.right.is_some() {
            return Err(TreeCoreError::RightTreeExists);
        }

        self.right = Some(Box::new(Node::new(value)));

        Ok(self.right.as_mut().unwrap())
    }
}

impl<T> BasicNode for Node<T> {
    type N = Node<T>;
    type V = T;

    fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }

    fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    fn value(&self) -> &Self::V {
        &self.value
    }
}

pub struct RopeNode {
    pub(crate) text: String,
    pub(crate) left: Link<RopeNode>,
    pub(crate) right: Link<RopeNode>,
    pub(crate) priority: f64,
    pub(crate) subtree_size: i64,
}

impl RopeNode {
    pub fn new(text: &str) -> RopeNode {
        RopeNode {
            text: text.to_owned(),
            left: None,
            right: None,
            priority: rand::random(),
            subtree_size: text.chars().count() as i64,
        }
    }

    pub fn priority(&self) -> &f64 {
        &self.priority
    }

    pub fn size(&self) -> &i64 {
        &self.subtree_size
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn check_index_inclusive(&self, index: usize) -> Result<(), IndexError> {
        if index <= *self.size() as usize {
            Ok(())
        } else {
            Err(IndexError::NotInclusiveError)
        }
    }

    pub(crate) fn check_index_exclusive(&self, index: usize) -> Result<(), IndexError> {
        if index < *self.size() as usize {
            Ok(())
        } else {
            Err(IndexError::NotExclusiveError)
        }
    }

    pub(crate) fn recalculate_size(&mut self) {
        let mut sum = 0;

        if let Some(n) = self.left() {
            sum += n.size()
        }
        if let Some(n) = self.right() {
            sum += n.size()
        }

        self.subtree_size = self.text().len() as i64 + sum;
    }

    pub(crate) fn join(left: Link<RopeNode>, right: Link<RopeNode>) -> Link<RopeNode> {
        match (left, right) {
            (l, None) => l,
            (None, r) => r,
            (Some(mut l_node), Some(mut r_node)) => {
                if l_node.priority > r_node.priority {
                    let left_sub = l_node.right.take();

                    l_node.right = Self::join(left_sub, Some(r_node));
                    l_node.recalculate_size();

                    Some(l_node)
                } else {
                    let right_sub = r_node.left.take();

                    r_node.left = Self::join(Some(l_node), right_sub);
                    r_node.recalculate_size();

                    Some(r_node)
                }
            }
        }
    }

    pub(crate) fn split(
        mut root: Link<RopeNode>,
        index: usize,
    ) -> Result<(Link<RopeNode>, Link<RopeNode>), SplitError> {
        if root.is_none() {
            return Ok((None, None));
        }

        let text_len = root.as_ref().unwrap().text().len();

        let left_size = match root.as_ref().unwrap().left() {
            Some(left) => *left.size() as usize,
            None => 0,
        };

        let node = root.as_deref_mut().unwrap();

        if index < left_size {
            let left = node.left.take();
            let (left_sub, right_sub) = Self::split(left, index)?;

            node.left = right_sub;
            node.recalculate_size();

            return Ok((left_sub, root));
        }

        if index >= left_size.strict_add(text_len) {
            let right = node.right.take();
            let right_index = index - left_size - text_len;

            let (left_sub, right_sub) = Self::split(right, right_index)?;

            node.right = left_sub;
            node.recalculate_size();

            return Ok((root, right_sub));
        }

        let offset = 0.max(text_len.min(index - left_size));

        if let Some((left_text, right_text)) = node.text.split_at_checked(offset) {
            let mut left_tree = node.left.take();
            let mut right_tree = node.right.take();

            if !left_text.is_empty() {
                let left_leaf = Some(Box::new(RopeNode::new(left_text)));
                left_tree = Self::join(left_tree, left_leaf);
            }

            if !right_text.is_empty() {
                let right_leaf = Some(Box::new(RopeNode::new(right_text)));
                right_tree = Self::join(right_leaf, right_tree);
            }

            Ok((left_tree, right_tree))
        } else {
            Err(SplitError::FailedToSplitText)
        }
    }
}

impl BasicNode for RopeNode {
    type N = RopeNode;
    type V = String;

    fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }
    fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    fn value(&self) -> &Self::V {
        &self.text
    }
}
