use data_structures::binary_tree::bt;

fn main() {
    let mut root = bt::Node::new(1);

    root.left = Some(Box::new(bt::Node::new(2)));

    println!("{}", &root.value);
    println!("{}", &root.left.as_ref().unwrap().value);
    
    match &root.right {
        Some(node) => println!("Right node value: {}", node.value),
        _ => println!("Right node is empty")
    }

    let mut bt = bt::BinaryTree::new(bt::Node::new(1));
    {
        let left = bt.root.assign_left(2);
        left.assign_left(3);
        left.assign_right(4);
    };

    {
        let right = bt.root.assign_right(5);
        let right = right.assign_right(6);
        right.assign_right(7);
    };

    println!("height of root: {}", root.height());
    
    match bt.root.left.as_deref() {
        Some(node) => println!("BT root.left: {}", node.value),
        _ => println!("No root.left")
    };
    
    match bt.root.right.as_deref()
        .and_then(|n| n.right.as_deref())
        .and_then(|n| n.right.as_deref())
        {
            Some(node) => println!("BT root.right.right.right: {}", node.value),
            _ => println!("No node")
        };

    match bt.root.right.as_deref()
        .and_then(|n| n.right.as_deref())
        // .and_then(|n| n.right.as_deref())
        {
            Some(node) => println!("depth of root.right: {}", bt.depth(node)),
            _ => println!("No node")
        };
    
    println!("Height of the BT: {}", bt.root.height());

}
