# Rust Playground 🦀

This repository is a personal learning playground for Rust. Since these are hobby / learning projects, the APIs, structure, and behavior may change freely.

## Repository Layout

```text
.
├── Cargo.toml
├── README.md
├── crates
│   ├── data_structures
│   │   └── README.md
│   ├── selector
│   │   └── README.md
│   └── text_editor
│       └── README.md
└── src
```

## Crates

- [`data_structures`](crates/data_structures/README.md): learning-oriented implementations of binary trees, binary search trees, iterators, and a rope used by the text editor crate.
- [`text_editor`](crates/text_editor/README.md): a terminal editor prototype built on `crossterm` and the rope implementation from `data_structures`.
- [`selector`](crates/selector/README.md): experimental feature orthogonal-selection, distance-correlation. Ideal for R&D stage of machine-learning projects (can be compiled to *.whl for python).

## Quick Commands

From the repository root:

```bash
cargo run -p data_structures --example run_bst
cargo run -p text_editor
cargo test -p selector
cd crates/selector/ && python -m maturin build --features python
```

## License

MIT (or whatever you prefer, feel free to change)
