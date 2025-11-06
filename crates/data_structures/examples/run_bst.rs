use data_structures::binary_tree::{bst::{self, SearchTree}, bt::{self, Tree}};

fn main() {
    let mut tree = bst::example_bst();
    let search_result = tree.search(&bt::Node::new(6));
    println!("Result from search: {}", search_result.value);

    tree.insert(bt::Node::new(0));

    let traversal_result: Vec<&i64> = match tree.inorder() {
        Ok(v) => v.collect(),
        Err(_) => panic!("Failed to traverse the tree!")
    };
    println!("Tree nodes after insert: {:?}", traversal_result);
}
