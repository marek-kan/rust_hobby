# data_structures

[Back to repository root](../../README.md)

`data_structures` is a learning-focused library crate for hand-written data structure implementations. The code here is meant for experimentation and iteration rather than API stability.

## Scope

- Binary trees and binary search trees
- Traversal patterns and iterator implementations
- Error handling around tree operations
- Rope-style text storage for editing workloads

## Layout

```text
data_structures
├── Cargo.toml
├── README.md
├── examples
│   ├── run_bst.rs
│   ├── run_bt.rs
│   └── run_rope.rs
└── src
    ├── binary_tree.rs
    ├── binary_tree
    │   ├── bst.rs
    │   ├── bt.rs
    │   ├── errors.rs
    │   ├── iterators.rs
    │   ├── merge_explained.md
    │   ├── nodes.rs
    │   ├── rope.rs
    │   └── trees.rs
    └── lib.rs
```

## What Lives Where

- `src/lib.rs` exposes the crate's `binary_tree` module.
- `src/binary_tree.rs` wires the tree-related submodules together.
- `src/binary_tree/bt.rs` contains the more general binary tree implementation surface.
- `src/binary_tree/bst.rs` contains binary search tree behavior such as search, insertion, deletion, and sample-tree helpers.
- `src/binary_tree/iterators.rs` holds traversal iterators.
- `src/binary_tree/nodes.rs` defines shared node types.
- `src/binary_tree/trees.rs` contains shared tree traits and abstractions.
- `src/binary_tree/errors.rs` collects operation-specific errors.
- `src/binary_tree/rope.rs` implements the rope used by the `text_editor` crate.
- `examples/` contains small runnable demos for the main structures.

## Where To Start

- Start with `src/binary_tree/bst.rs` if you want the clearest path through search-tree behavior.
- Start with `src/binary_tree/rope.rs` and `examples/run_rope.rs` if you want to understand the text-buffer side of the repository.
- Read `src/binary_tree/merge_explained.md` if you want design notes around tree merge behavior.

## Running Examples

From the repository root:

```bash
cargo run -p data_structures --example run_bt
cargo run -p data_structures --example run_bst
cargo run -p data_structures --example run_rope
```

## Notes

- This crate is built for learning, so internal structure may change freely.
- The rope implementation is currently the bridge between this crate and the `text_editor` crate.
