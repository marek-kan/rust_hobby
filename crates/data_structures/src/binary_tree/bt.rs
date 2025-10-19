type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub enum AlreadyExist {
    LeftTreeExists,
    RightTreeExists
}

pub struct Node<T> {
    pub value: T,
    pub left: Link<T>,
    pub right: Link<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node{value: value, left: None, right: None}
    }

    pub fn assign_left(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExist> {
        if self.left.is_some() {
            return Err(AlreadyExist::LeftTreeExists)
        }

        self.left = Some(Box::new(Node { value: value, left: None, right: None }));
        Ok(self.left.as_mut().unwrap())
    }

    pub fn assign_right(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExist> {
        if self.right.is_some() {
            return Err(AlreadyExist::RightTreeExists)
        }

        self.right = Some(Box::new(Node { value: value, left: None, right: None }));
        Ok(self.right.as_mut().unwrap())
    }

    pub fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |node| node.height() + 1);
        let right_height = self.right.as_ref().map_or(0, |node| node.height() + 1);

        left_height.max(right_height)
    }
}

pub struct BinaryTree<T> {
    pub root: Node<T>
}

impl <T> BinaryTree<T> {
    pub fn new(node: Node<T>) -> BinaryTree<T> {
        BinaryTree { root: node }
    }

    fn _depth(&self, node: &Node<T>, target: &Node<T>, current_depth: usize) -> usize {
        if std::ptr::eq(node, target) {
            return current_depth
        }

        let left_depth = node.left.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));
        let right_depth = node.right.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));

        left_depth.max(right_depth)
    }

    pub fn depth(&self, target: &Node<T>) -> usize {
        self._depth(&self.root, target, 0)
    }

    pub fn inorder(&self) -> InOrder<'_, T> {
        InOrder::new(&self.root)
    }
    
    pub fn preorder(&self) -> PreOrder<'_, T> {
        PreOrder::new(&self.root)
    }

    pub fn postorder(&self) -> PostOrder<'_, T> {
        PostOrder::new(&self.root)
    }
}


pub struct InOrder<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>
}

impl<'a, T> InOrder<'a, T> {
    fn new(root: &'a Node<T>) -> InOrder<'a, T> {
        InOrder { stack: Vec::new(), current: Some(root) }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Dive left as far as possible
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left.as_deref();
        }

        let node = self.stack.pop()?;
        self.current = node.right.as_deref();
        Some(&node.value)
    }
}

pub struct PreOrder<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PreOrder<'a, T> {
    pub fn new(root: &'a Node<T>) -> PreOrder<'a, T> {
        PreOrder { stack: vec![root] }
    }
}

impl<'a, T> Iterator for PreOrder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        let node = self.stack.pop()?;
        
        if let Some(right) = node.right.as_deref() {
            self.stack.push(right);
        }
        if let Some(left) = node.left.as_deref() {
            self.stack.push(left);
        }

        Some(&node.value)
    }
}

pub struct PostOrder <'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PostOrder<'a, T> {
    pub fn new(root: &'a Node<T>) -> PostOrder<'a, T> {
        // Pre-compute stack; TODO: make true stream
        let mut s1 = vec![root];
        let mut s2 = Vec::new();

        while let Some(node) = s1.pop() {
            s2.push(node);

            if let Some(left) = node.left.as_deref() {
                s1.push(left);
            }

            if let Some(right) = node.right.as_deref() {
                s1.push(right);
            }
        }

        PostOrder { stack: s2 }
    }  
}

impl<'a, T> Iterator for PostOrder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|n| &n.value)
    }
}
