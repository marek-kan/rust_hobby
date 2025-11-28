use crate::binary_tree::errors::AlreadyExists;

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

    pub fn assign_left(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExists> {
        if self.left.is_some() {
            return Err(AlreadyExists::LeftTreeExists);
        }

        self.left = Some(Box::new(Node::new(value)));

        Ok(self.left.as_mut().unwrap())
    }

    pub fn assign_right(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExists> {
        if self.right.is_some() {
            return Err(AlreadyExists::RightTreeExists);
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
            subtree_size: text.len() as i64,
        }
    }
}

impl RopeNode {
    pub fn priority(&self) -> &f64 {
        &self.priority
    }

    pub fn size(&self) -> &i64 {
        &self.subtree_size
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn recalculate(&mut self) {
        let mut sum = 0;

        if let Some(n) = self.left() {
            sum += n.size()
        }
        if let Some(n) = self.right() {
            sum += n.size()
        }

        self.subtree_size = self.text().len() as i64 + sum;
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
