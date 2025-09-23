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

    println!("height of root: {}", root.height());
    println!("depth of root.left: {}", root.depth(&root.left).unwrap_or(-9));
    println!("depth of root.right: {}", root.depth(&root.right).unwrap_or(-999));
}
