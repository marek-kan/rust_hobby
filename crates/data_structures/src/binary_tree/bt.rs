type Link<T> = Option<Box<Node<T>>>;


#[derive(Debug)]
pub enum AlreadyExists {
    LeftTreeExists,
    RightTreeExists
}


#[derive(Debug)]
pub enum DoesntExist {
    NoRootNode,
    NoTargetNode
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

    pub fn assign_left(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExists> {
        if self.left.is_some() {
            return Err(AlreadyExists::LeftTreeExists)
        }

        self.left = Some(Box::new(Node { value: value, left: None, right: None }));
        Ok(self.left.as_mut().unwrap())
    }

    pub fn assign_right(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExists> {
        if self.right.is_some() {
            return Err(AlreadyExists::RightTreeExists)
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


pub trait Tree<T> {
    fn get_root(&self) -> Option<&Node<T>>;

    fn _depth(&self, node: &Node<T>, target: &Node<T>, current_depth: usize) -> usize {
        if std::ptr::eq(node, target) {
            return current_depth
        }

        let left_depth = node.left.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));
        let right_depth = node.right.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));

        left_depth.max(right_depth)
    }

    fn depth(&self, target: &Node<T>) -> usize {
        match self.get_root() {
            Some(root) => self._depth(root, target, 0),
            _ => {
                println!("No root node!");
                0
            }
        }
    }

    fn inorder(&self) -> Result<InOrder<'_, T>, DoesntExist> {

        match self.get_root() {
            Some(root) => Ok(InOrder::new(root)),
            _ => Err(DoesntExist::NoRootNode)
        }       
    }
    
    fn preorder(&self) -> Result<PreOrder<'_, T>, DoesntExist> {

        match self.get_root() {
            Some(root) => Ok(PreOrder::new(root)),
            _ => Err(DoesntExist::NoRootNode)
        }  
    }

    fn postorder(&self) -> Result<PostOrderStraming<'_, T>, DoesntExist> {
        
        match self.get_root() {
            Some(root) => Ok(PostOrderStraming::new(root)),
            _ => Err(DoesntExist::NoRootNode)
        }
    }

    fn postorder_precomp(&self) -> Result<PostOrderPreComputed<'_, T>, DoesntExist> {
        // Precomputes whole stack, next() is very cheap, good if you need index into order repeatedly. However, `postorder`
        // is prefered for most use-cases.
        match self.get_root() {
            Some(root) => Ok(PostOrderPreComputed::new(root)),
            _ => Err(DoesntExist::NoRootNode)
        }
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


pub struct PostOrderPreComputed <'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PostOrderPreComputed<'a, T> {
    pub fn new(root: &'a Node<T>) -> PostOrderPreComputed<'a, T> {
        // Pre-compute stack
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

        PostOrderPreComputed { stack: s2 }
    }  
}

impl<'a, T> Iterator for PostOrderPreComputed<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|n| &n.value)
    }
}


pub struct PostOrderStraming<'a, T> {
    current: Option<&'a Node<T>>,
    stack: Vec<&'a Node<T>>,
    last_visited: Option<&'a Node<T>>
}

impl<'a, T> PostOrderStraming<'a, T> {
    pub fn new(root: &'a Node<T>) -> PostOrderStraming<'a, T> {
        PostOrderStraming { current: Some(root), stack: Vec::new(), last_visited: None }
    }
}

impl<'a, T> Iterator for PostOrderStraming<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        loop {
            while let Some(node) = self.current {
                
                self.stack.push(node);
                self.current = node.left.as_deref();
            }

            let top = self.stack.last().copied();

            if let Some(node) = top {
                if let Some(right) = node.right.as_deref() {
                    
                    let visited_right = matches!(self.last_visited, Some(v) if std::ptr::eq(v, right));
                    
                    if !visited_right {
                        self.current = Some(right);
                        continue;
                    }
                }
            };

            let node = self.stack.pop()?;
            self.last_visited = Some(node);
            
            return Some(&node.value)
        }
    }
}


pub struct BinaryTree<T> {
    pub root: Node<T>
}

impl <T> BinaryTree<T> {
    pub fn new(node: Node<T>) -> BinaryTree<T> {
        BinaryTree { root: node }
    }
}

impl<T> Tree<T> for BinaryTree<T> {
    fn get_root(&self) -> Option<&Node<T>> {
        Some(&self.root)
    }
}
