type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub left: Link<T>,
    pub right: Link<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node{value:value, left: None, right: None}
    }

    pub fn height(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |node| node.height() + 1);
        let right_height = self.right.as_ref().map_or(0, |node| node.height() + 1);

        left_height.max(right_height)
    }
    // TODO: Implement proper `depth` function
    // - Depth = number of edges from ROOT -> specific NODE (not value).
    // - Given the root (self), find this exact node in the tree.
    // - Must search by node identity (pointer), not by value, since multiple nodes can hold the same value 
    //   (e.g., two distinct nodes with value 4). Use `std::ptr::eq` to check.
    pub fn depth(&self, node: &Link<T>) -> Option<i32> {
        match node {
            Some(_) => return Some(-99),
            None => return None
        };
    }
}