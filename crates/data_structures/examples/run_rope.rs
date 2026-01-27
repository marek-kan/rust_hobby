use data_structures::binary_tree::rope::Rope;

fn main() {
    let mut rope = Rope::new("This is my text");

    println!("Rope text: {}", &rope);

    // insert at the end
    let mut max_index = rope.tree_size().unwrap();
    rope = rope.insert("\nSecond row", max_index).unwrap();

    println!("Rope text after insert at the end: {}", &rope);

    // insert in the middle
    max_index = rope.tree_size().unwrap();
    let middle = max_index / 2;

    rope = rope.insert("**INSERT IN THE MIDDLE**", middle).unwrap();

    println!("Rope text after insert in the middle: {}", &rope);

    // delete last 10 characters
    max_index = rope.tree_size().unwrap();
    let n = 5;

    rope = rope.delete(max_index - n, n).unwrap();

    println!("Rope text after delete {}: {}", n, &rope);
}
