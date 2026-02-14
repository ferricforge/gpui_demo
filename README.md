# GPUI Demo

A demo application built with [GPUI](https://crates.io/crates/gpui) and [gpui-component](https://crates.io/crates/gpui-component).

## Building

```bash
cargo build
```

## Running

Run the main application (empty window):

```bash
cargo run
```

## Examples

Run the example with a button:

```bash
cargo run --example with_button
```

## Project Structure
* src/lib.rs - Library entry point with app setup utilities
* src/main.rs - Main binary entry point
* src/components/ - UI components (Window, etc.)
* src/preferences/ - Application preferences (window size, etc.)
* examples/ - Example applications demonstrating usage
