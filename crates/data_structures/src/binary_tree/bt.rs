pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub enum AlreadyExists {
    LeftTreeExists,
    RightTreeExists,
}

#[derive(Debug)]
pub enum DoesntExist {
    NoRootNode,
    NoTargetNode,
}

pub struct Node<T> {
    pub value: T,
    pub left: Link<T>,
    pub right: Link<T>,
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

        self.left = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
        Ok(self.left.as_mut().unwrap())
    }

    pub fn assign_right(&mut self, value: T) -> Result<&mut Node<T>, AlreadyExists> {
        if self.right.is_some() {
            return Err(AlreadyExists::RightTreeExists);
        }

        self.right = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
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

    fn get_mut_root(&mut self) -> Option<&mut Node<T>>;

    fn _depth(&self, node: &Node<T>, target: &Node<T>, current_depth: usize) -> usize {
        if std::ptr::eq(node, target) {
            return current_depth;
        }

        let left_depth = node
            .left
            .as_ref()
            .map_or(0, |n| self._depth(n, target, current_depth + 1));
        let right_depth = node
            .right
            .as_ref()
            .map_or(0, |n| self._depth(n, target, current_depth + 1));

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
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn preorder(&self) -> Result<PreOrder<'_, T>, DoesntExist> {
        match self.get_root() {
            Some(root) => Ok(PreOrder::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn postorder(&self) -> Result<PostOrderStraming<'_, T>, DoesntExist> {
        match self.get_root() {
            Some(root) => Ok(PostOrderStraming::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }

    fn postorder_precomp(&self) -> Result<PostOrderPreComputed<'_, T>, DoesntExist> {
        // Precomputes whole stack, next() is very cheap, good if you need index into order repeatedly. However, `postorder`
        // is prefered for most use-cases.
        match self.get_root() {
            Some(root) => Ok(PostOrderPreComputed::new(root)),
            _ => Err(DoesntExist::NoRootNode),
        }
    }
}

pub struct InOrder<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> InOrder<'a, T> {
    fn new(root: &'a Node<T>) -> InOrder<'a, T> {
        InOrder {
            stack: Vec::new(),
            current: Some(root),
        }
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

pub struct PostOrderPreComputed<'a, T> {
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
    last_visited: Option<&'a Node<T>>,
}

impl<'a, T> PostOrderStraming<'a, T> {
    pub fn new(root: &'a Node<T>) -> PostOrderStraming<'a, T> {
        PostOrderStraming {
            current: Some(root),
            stack: Vec::new(),
            last_visited: None,
        }
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
                    let visited_right =
                        matches!(self.last_visited, Some(v) if std::ptr::eq(v, right));

                    if !visited_right {
                        self.current = Some(right);
                        continue;
                    }
                }
            };

            let node = self.stack.pop()?;
            self.last_visited = Some(node);

            return Some(&node.value);
        }
    }
}

pub struct BinaryTree<T> {
    pub root: Node<T>,
}

impl<T> BinaryTree<T> {
    pub fn new(node: Node<T>) -> BinaryTree<T> {
        BinaryTree { root: node }
    }
}

impl<T> Tree<T> for BinaryTree<T> {
    fn get_root(&self) -> Option<&Node<T>> {
        Some(&self.root)
    }

    fn get_mut_root(&mut self) -> Option<&mut Node<T>> {
        Some(&mut self.root)
    }
}

/// Builds sample BinaryTree for examples. Shouldn't be run at any other context
pub fn build_sample_tree() -> BinaryTree<i64> {
    let mut tree = BinaryTree::new(Node::new(1));

    // left branch
    {
        let left = tree.root.assign_left(2).unwrap();
        left.assign_left(3).unwrap();
        left.assign_right(4).unwrap();
    }

    // right branch
    {
        let right = tree.root.assign_right(5).unwrap();
        let right = right.assign_right(6).unwrap();
        right.assign_right(7).unwrap();
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_simple_tree() -> BinaryTree<i64> {
        let mut tree = BinaryTree::new(Node::new(1));

        {
            let root = tree
                .get_mut_root()
                .expect("Failed to retrieve the root node!");
            let l2 = root.assign_left(2).expect("root.left(2)");
            let l3 = l2.assign_left(3).expect("root.left.left(3)");
            let _r4 = l3.assign_right(4).expect("root.left.left.right(4)");
        }

        tree
    }

    #[test]
    fn root_depth() {
        let tree = build_sample_tree();
        let root = tree.get_root().expect("root exists");
        assert_eq!(tree.depth(root), 0);
    }

    #[test]
    fn height_of_tree() {
        let tree = build_simple_tree();
        let height = tree.get_root().expect("root exists").height();
        assert_eq!(height, 3);
    }

    #[test]
    fn second_left_value_depth() {
        let tree = build_simple_tree();

        let second_left = tree
            .get_root()
            .and_then(|n| n.left.as_deref())
            .and_then(|n| n.left.as_deref())
            .expect("Failed to retrieve second left node");

        let depth = tree.depth(second_left);

        assert_eq!(second_left.value, 3);
        assert_eq!(depth, 2);
    }

    #[test]
    fn inorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .inorder()
            .expect("Failed to traverse in-order")
            .copied()
            .collect();

        assert_eq!(result, vec![3, 4, 2, 1]);
    }

    #[test]
    fn preorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .preorder()
            .expect("Failed to traverse pre-order")
            .copied()
            .collect();

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn postorder() {
        let tree = build_simple_tree();
        let result: Vec<i64> = tree
            .postorder()
            .expect("Failed to traverse post-order")
            .copied()
            .collect();

        assert_eq!(result, vec![4, 3, 2, 1]);
    }
}
