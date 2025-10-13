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

    let left = bt.root.assign_left(2);
    left.assign_left(3);
    left.assign_right(4);

    let right = bt.root.assign_right(5);
    let right = right.assign_right(6);
    right.assign_right(7);

    println!("height of root: {}", root.height());
    println!("BT root.left: {}", bt.root.left.unwrap().value);
    println!("BT root.right.right.right: {}", bt.root.right.unwrap().right.unwrap().right.unwrap().value);
    // println!("depth of root.right: {}", root.depth(&root.right).unwrap_or(-999));
}
