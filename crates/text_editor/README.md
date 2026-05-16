# text_editor

[Back to repository root](../../README.md)

`text_editor` is a terminal editor prototype built to exercise text-buffer management, cursor movement, and rendering loops in Rust. It depends on `crossterm` for terminal IO and on the `data_structures` crate for its rope-backed buffer.

## Current Scope

- Interactive editing in a terminal alternate screen
- Rope-backed text storage through `data_structures::binary_tree::rope::Rope`
- Cursor movement and line bookkeeping
- File open and save prompts
- Architecture exploration over feature completeness

## Layout

```text
text_editor
├── Cargo.toml
├── README.md
└── src
    ├── core.rs
    ├── core
    │   ├── management.rs
    │   └── ui.rs
    ├── debug_main.rs
    ├── lib.rs
    └── main.rs
```

## What Lives Where

- `src/lib.rs` exposes the crate's `core` module.
- `src/core.rs` wires the management and UI layers.
- `src/core/management.rs` contains `TextBuffer`, `Cursor`, `Viewport`, file IO helpers, and buffer mutation logic.
- `src/core/ui.rs` contains terminal rendering and prompt helpers.
- `src/main.rs` is the interactive event loop and key-handling entry point.
- `src/debug_main.rs` is a scratch/debug entry point.

## Running

From the repository root:

```bash
cargo run -p text_editor
```

If you downloaded a release binary instead of building from source:

### macOS

```bash
chmod +x text_editor-macOS
xattr -d com.apple.quarantine text_editor-macOS
./text_editor-macOS
```

### Linux

```bash
chmod +x text_editor-Linux
./text_editor-Linux
```

### Windows

If Windows blocks execution:

1. Right-click the `.exe`.
2. Select **Properties**.
3. Check **Unblock**.
4. Click **OK**.

You may also see a "Windows protected your PC" warning. If so, click **More info**, then **Run anyway**.

## Controls

### Text Input

- Character keys insert characters at the cursor.
- `Enter` inserts a newline.
- `Backspace` deletes the character before the cursor.
- `Delete` deletes the character at the cursor.

### Cursor Movement

- `Left` and `Right` move within a line.
- `Up` and `Down` move between lines.

### File Operations

- `Ctrl+S` prompts for a file path, saves the current buffer, and exits.
- `Ctrl+O` prompts for a file path and loads that file into the buffer.

### Exit

- `Esc` exits without saving.

## Limitations

- The editor is still ASCII-oriented and will behave poorly with non-ASCII input.
- Prompt editing is minimal.
- Saving currently exits the editor.
- The project is primarily for architecture and data-structure exploration, not for building a production editor.
