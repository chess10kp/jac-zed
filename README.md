# Jac Zed Extension

Jac language support for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-jac](https://github.com/jaseci-labs/tree-sitter-jac)
- Language Server: [jaclang](https://github.com/jaseci-labs/jaseci) (`jac lsp`)

## Installation

Install the `jac` CLI and ensure it is on your `PATH`:

```bash
pip install jaclang
```

Zed will automatically start `jac lsp` for `.jac` files.

## Configuration

You can override the language server binary or arguments in your Zed settings:

```json
{
  "lsp": {
    "jac-lsp": {
      "binary": {
        "path": "/path/to/jac",
        "arguments": ["lsp"]
      }
    }
  }
}
```

Any settings placed under `lsp.jac-lsp.settings` are forwarded to the language server as workspace configuration.

## Development

Build the extension as a WebAssembly module:

```bash
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2
```

Then open Zed, run the **"zed: extensions"** command, click **Install Dev Extension**, and select this directory.

## Project Structure

- `extension.toml` — extension manifest and language/grammar declarations
- `Cargo.toml` / `src/jac.rs` — Rust extension entry point that launches `jac lsp`
- `languages/jac/` — language configuration and Tree-sitter highlights
