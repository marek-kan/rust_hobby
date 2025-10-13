type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub left: Link<T>,
    pub right: Link<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node{value: value, left: None, right: None}
    }

    pub fn assign_left(&mut self, value: T) -> &mut Node<T> {
        self.left = Some(Box::new(Node { value: value, left: None, right: None }));
        self.left.as_mut().unwrap()
    }

    pub fn assign_right(&mut self, value: T) -> &mut Node<T> {
        self.right = Some(Box::new(Node { value: value, left: None, right: None }));
        self.right.as_mut().unwrap()
    }

    pub fn height(&self) -> i32 {
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

    fn _depth(&self, node: &Node<T>, target: &Node<T>, current_depth: i64) -> i64 {
        if std::ptr::eq(node, target) {
            return current_depth
        }

        let left_depth = node.left.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));
        let right_depth = node.right.as_ref().map_or(0, |n| self._depth(n, target, current_depth + 1));

        left_depth.max(right_depth)
    }

    pub fn depth(&self, target: &Node<T>) -> i64 {
        self._depth(&self.root, target, 0)
    }
}
