use data_structures::binary_tree::bt::{self, Tree};

fn demo_single_node() {
    println!("=== Node demo ===");
    let mut root = bt::Node::new(1);
    root.left = Some(Box::new(bt::Node::new(2)));

    println!("root.value = {}", root.value);
    println!("root.left.value = {}", root.left.as_ref().unwrap().value);

    match &root.right {
        Some(node) => println!("root.right.value = {}", node.value),
        None => println!("root.right is empty"),
    }

    println!("height(root) = {}", root.height());
    println!("=== End of Node demo ===");
}

fn right_right_right(start: Option<&bt::Node<i64>>) -> Option<&bt::Node<i64>> {
    start
        .and_then(|n| n.right.as_deref())
        .and_then(|n| n.right.as_deref())
        .and_then(|n| n.right.as_deref())
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

    match bt.root.left.as_deref() {
        Some(node) => println!("tree.root.left: {}", node.value),
        _ => println!("tree.root.left = <none>"),
    };

    let deep_right = right_right_right(bt.root.right.as_deref());
    match deep_right {
        Some(node) => println!("tree.root.right.right.right: {}", node.value),
        _ => println!("tree.root.right.right.right = <none>"),
    };

    // Depth of a node
    match bt.root.right.as_deref().and_then(|n| n.right.as_deref()) {
        Some(node) => println!("depth(root.right.right) = {}", bt.depth(node)),
        _ => println!("depth(root.right.right) = <none>"),
    };

    demo_traversals(&bt);
}
