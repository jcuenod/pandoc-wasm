# Description

There are a few pandoc-related packages in the Rust ecosystem. All of them require pandoc as a separate dependency. This package wraps pandoc.wasm and uses wasmer to provide a pandoc binary that can be used in Rust without requiring pandoc to be installed on the system.

# Usage
    
```rust
use pandoc_wasm_wrapper::pandoc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = vec![
        "--from=markdown".to_string(),
        "--to=html".to_string()
    ];
    let input: Vec<u8> = "# Hello, world!".as_bytes().to_vec();
    let output: String = pandoc(&args, &input).await.unwrap();
    println!("{}", output);
}
```

If you wanted to convert a `docx` to `markdown`, you could do the following:

```rust
use pandoc_wasm_wrapper::pandoc;

#[tokio::main]
fn main() {
    let args: Vec<String> = vec![
        "--from=docx".to_string(),
        "--to=markdown".to_string()
    ];
    let docx_input: Vec<u8> = std::fs::read("path/to/file.docx").unwrap();
    let md_output: String = pandoc(&args, &docx_input).await.unwrap();
    println!("{}", md_output);
}
```

# Credits

This package is completely dependent on the work of [pandoc-wasm](https://github.com/tweag/pandoc-wasm)