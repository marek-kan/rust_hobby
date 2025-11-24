use crate::binary_tree::nodes::BasicNode;


pub struct InOrder<'a, Node: BasicNode> {
    stack: Vec<&'a Node>,
    current: Option<&'a Node>,
}

impl<'a, Node: BasicNode> InOrder<'a, Node> {
    pub fn new(root: &'a Node) -> InOrder<'a, Node> {
        InOrder {
            stack: Vec::new(),
            current: Some(root),
        }
    }
}

impl<'a, Node: BasicNode> Iterator for InOrder<'a, Node> {
    type Item = &'a <Node as BasicNode>::V;

    fn next(&mut self) -> Option<Self::Item> {
        // Dive left as far as possible
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left();
        }

        let node = self.stack.pop()?;
        self.current = node.right();
        Some(node.value())
    }
}

pub struct PreOrder<'a, Node: BasicNode> {
    stack: Vec<&'a Node>,
}

impl<'a, Node: BasicNode> PreOrder<'a, Node> {
    pub fn new(root: &'a Node) -> PreOrder<'a, Node> {
        PreOrder { stack: vec![root] }
    }
}

impl<'a, Node: BasicNode> Iterator for PreOrder<'a, Node> {
    type Item = &'a <Node as BasicNode>::V;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if let Some(right) = node.right() {
            self.stack.push(right);
        }
        if let Some(left) = node.left() {
            self.stack.push(left);
        }

        Some(node.value())
    }
}

pub struct PostOrderPreComputed<'a, Node: BasicNode> {
    stack: Vec<&'a Node>,
}

impl<'a, Node: BasicNode> PostOrderPreComputed<'a, Node> {
    pub fn new(root: &'a Node) -> PostOrderPreComputed<'a, Node> {
        // Pre-compute stack
        let mut s1 = vec![root];
        let mut s2 = Vec::new();

        while let Some(node) = s1.pop() {
            s2.push(node);

            if let Some(left) = node.left() {
                s1.push(left);
            }

            if let Some(right) = node.right() {
                s1.push(right);
            }
        }

        PostOrderPreComputed { stack: s2 }
    }
}

impl<'a, Node: BasicNode> Iterator for PostOrderPreComputed<'a, Node> {
    type Item = &'a <Node as BasicNode>::V;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|n| n.value())
    }
}

pub struct PostOrderStraming<'a, Node: BasicNode> {
    current: Option<&'a Node>,
    stack: Vec<&'a Node>,
    last_visited: Option<&'a Node>,
}

impl<'a, Node: BasicNode> PostOrderStraming<'a, Node> {
    pub fn new(root: &'a Node) -> PostOrderStraming<'a, Node> {
        PostOrderStraming {
            current: Some(root),
            stack: Vec::new(),
            last_visited: None,
        }
    }
}

impl<'a, Node: BasicNode> Iterator for PostOrderStraming<'a, Node> {
    type Item = &'a <Node as BasicNode>::V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(node) = self.current {
                self.stack.push(node);
                self.current = node.left();
            }

            let top = self.stack.last().copied();

            if let Some(node) = top && let Some(right) = node.right() {
                    let visited_right =
                        matches!(self.last_visited, Some(v) if std::ptr::eq(v, right));

                    if !visited_right {
                        self.current = Some(right);
                        continue;
                    }
            };

            let node = self.stack.pop()?;
            self.last_visited = Some(node);

            return Some(node.value());
        }
    }
}
