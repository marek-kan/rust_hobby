use data_structures::binary_tree::{bst, nodes::Node, trees::SearchTree, trees::Tree};

fn main() {
    let mut tree = bst::build_sample_tree();
    let node_to_search = &Node::new(3);

    println!("=== Search demo ===");
    {
        let search_result = tree.search(node_to_search);
        println!(
            "Result from search: {}",
            search_result.as_deref().unwrap().value
        );
    }
    println!("=== End of search demo ===");

    println!("=== Insert demo ===");

    tree.insert(Node::new(0))
        .unwrap_or_else(|e| println!("{:?} - Failed to insert", e));

    let traversal_result: Vec<&i64> = match tree.inorder() {
        Ok(v) => v.collect(),
        Err(_) => panic!("Failed to traverse the tree!"),
    };
    println!("Tree nodes after insertion: {:?}", traversal_result);

    println!("=== End of insert demo ===");

    println!("=== Delete demo ===");

    tree.delete(&Node::new(6))
        .unwrap_or_else(|e| println!("{:?} - Failed to delete", e));

    let traversal_result: Vec<&i64> = match tree.preorder() {
        Ok(v) => v.collect(),
        Err(_) => panic!("Failed to traverse the tree!"),
    };
    println!("Tree nodes after deletion: {:?}", traversal_result);

    println!("=== End of delete demo ===");
}
