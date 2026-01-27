# Rust Playground 🦀

This repository is a personal learning playground for Rust. Since these are hobby / learning projects, the APIs, structure, and behavior may change freely.

---

## Repository Structure

```text
.
├── crates
│   ├── data_structures     # Core data structures library
│   └── text_editor         # Terminal-based text editor using sata structures
├── src                     # Root
└── README.md
```

---

## Crates

### `data_structures`

A library crate containing hand-written data structure implementations, mainly for learning and experimentation.

**Current focus:**

* Binary trees
* Binary search trees
 * Iterators and traversal patterns
 * Error handling around tree operations
* Rope-like structures for text manipulation

**Layout:**

```text
data_structures
├── examples
│   ├── run_bst.rs
│   ├── run_bt.rs
│   └── run_rope.rs
└── src
    ├── binary_tree
    │   ├── bst.rs
    │   ├── bt.rs
    │   ├── rope.rs
    │   ├── iterators.rs
    │   ├── nodes.rs
    │   └── errors.rs
    └── lib.rs
```

The `examples/` directory contains small runnable demos for experimenting with individual structures.

#### Run data structure examples

```bash
cargo run -p data_structures --example run_bst
cargo run -p data_structures --example run_rope
```

---

### `text_editor`

A terminal-based text editor built on top of `crossterm` and the `data_structures` crate.

**Scope & limitations:**

* ASCII-only text, will feel broken using non-ascii characters
* Focused on internal architecture, not feature completeness
* Built to explore cursor movement, text buffers, and rendering loops

**Key modules:**

* `management.rs`

  * Core editor state; `TextBuffer`, `Cursor`, `Viewport`
  * Buffer mutations and cursor logic

* `ui.rs`

  * Terminal rendering helpers / screen refresh logic

**Dependencies:**

```toml
[dependencies]
crossterm = "0.29.0"
data_structures = { path = "../data_structures" }
```

---

**Run the text editor:**

```bash
cargo run -p text_editor
```

**If you downloaded binary from a release:**

**MacOS**:
```
chmod +x text_editor-macOS
xattr -d com.apple.quarantine text_editor-macOS
./text_editor-macOS
```

**Linux**
```
chmod +x text_editor-Linux
./text_editor-Linux
```

**Windows**
If Windows prevents execution:

 1) Right-click the .exe
 2) Select Properties
 3) Check Unblock
 4) Click OK

Alternatively, you may see a “Windows protected your PC” warning, click More info → Run anyway.

#### Controls overview

*Text input:*

**Character keys:** insert characters at the cursor

**Enter:** insert a newline

**Backspace:** delete character before the cursor

**Delete:** delete character at the cursor

*Cursor movement:*

**←** / **→**: move cursor left / right

**↑** / **↓**: move cursor up / down between lines

*File operations:*

**Ctrl + S:** save file
Prompts for a filepath (only backspace allowed while typing).
Saving exits the editor.

**Ctrl + O:** open file
Prompts for a filepath and loads the file into the editor.

*Exit:*

**Esc:** exit the editor without saving

---

**Goals of the Project:**

* Learn Rust through non-trivial data structures
* Understand ownership, borrowing, and lifetimes in practice
* Experiment with terminal UI loops

This is not intended to be:

* A production-ready editor
* A reusable data structures crate
* A polished CLI tool

---

## Notes

Expect rough edges, broken things, refactors.

---

## License

MIT (or whatever you prefer, feel free to change)
