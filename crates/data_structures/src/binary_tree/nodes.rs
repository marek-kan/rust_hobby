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
    pub value: T,
    pub left: Link<Node<T>>,
    pub right: Link<Node<T>>,
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

pub struct RBNode<T> {
    pub value: T,
    pub left: Link<RBNode<T>>,
    pub right: Link<RBNode<T>>,
    pub parent: Link<RBNode<T>>,
    pub color: Color,
}

impl<T> RBNode<T> {
    pub fn new(value: T, color: Color, parent: Link<RBNode<T>>) -> RBNode<T> {
        RBNode {
            value,
            left: None,
            right: None,
            color,
            parent,
        }
    }
}

impl<T> BasicNode for RBNode<T> {
    type N = RBNode<T>;
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
