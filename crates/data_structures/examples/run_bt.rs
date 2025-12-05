use data_structures::binary_tree::{
    bt,
    nodes::{BasicNode, Node},
    trees::Tree,
};

fn demo_single_node() {
    println!("=== Node demo ===");
    let mut root = Node::new(1);
    let _left = root.assign_left(2).expect("Failed to assign root.left");

    println!("root.value = {}", root.value());
    println!("root.left.value = {}", root.left().unwrap().value());

    match root.right() {
        Some(node) => println!("root.right.value = {}", node.value()),
        None => println!("root.right is empty"),
    }

    println!("height(root) = {}", root.height());
    println!("=== End of Node demo ===");
}

fn right_right_right(start: Option<&Node<i64>>) -> Option<&Node<i64>> {
    start
        .and_then(|n| n.right())
        .and_then(|n| n.right())
        .and_then(|n| n.right())
}

fn demo_traversals(tree: &bt::BinaryTree<i64>) {
    println!("=== Tree traversals ===");
    let inorder_vals: Vec<&i64> = tree.inorder().unwrap().collect();
    let preorder_vals: Vec<&i64> = tree.preorder().unwrap().collect();
    let postorder_vals: Vec<&i64> = tree.postorder().unwrap().collect();

    println!("In-order: {:?}", inorder_vals);
    println!("Pre-order: {:?}", preorder_vals);
    println!("Post-order: {:?}", postorder_vals);
}

fn main() {
    demo_single_node();

    let bt = bt::build_sample_tree();

    // Just printing some values from the tree
    println!("Height of the BinaryTree: {}", bt.root.height());

    match bt.root.left() {
        Some(node) => println!("tree.root.left: {}", node.value()),
        _ => println!("tree.root.left = <none>"),
    };

    let deep_right = right_right_right(bt.root.right());
    match deep_right {
        Some(node) => println!("tree.root.right.right.right: {}", node.value()),
        _ => println!("tree.root.right.right.right = <none>"),
    };

    // Depth of a node
    match bt
        .root
        .right()
        .and_then(|n| n.right())
        .and_then(|n| bt.depth(n))
    {
        Some(depth) => println!("depth(root.right.right) = {}", depth),
        _ => println!("depth(root.right.right) = <none>"),
    };

    demo_traversals(&bt);
}
