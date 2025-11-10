use data_structures::binary_tree::{bst::{self, SearchTree}, bt::{self, Tree}};

fn main() {
    let mut tree = bst::example_bst();
    let node_to_search = &bt::Node::new(3);
    
    {
        let search_result = tree.search(node_to_search);
        println!("Result from search: {}", search_result.as_deref().unwrap().value);
    }

    // match tree.get_parent(node_to_search) {
    //     Ok(node) => println!("Parent of searched node is {}", node.value),
    //     Err(e) => println!("{:?}", e)
    // }

    tree.insert(bt::Node::new(0)).unwrap_or_else(|e| println!("{:?} - Failed to insert", e));

    let traversal_result: Vec<&i64> = match tree.inorder() {
        Ok(v) => v.collect(),
        Err(_) => panic!("Failed to traverse the tree!")
    };
    println!("Tree nodes after insertion: {:?}", traversal_result);


    tree.delete(&bt::Node::new(8)).unwrap_or_else(|e| println!("{:?} - Failed to delete", e));

    let traversal_result: Vec<&i64> = match tree.inorder() {
        Ok(v) => v.collect(),
        Err(_) => panic!("Failed to traverse the tree!")
    };
    println!("Tree nodes after deletion: {:?}", traversal_result);
}
