use std::io::{Read, Write};

use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

static WASM_BYTES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pandoc.wasm"));

/// Calls the pandoc wasm module with the given arguments and input bytes. The input is passed to pandoc via stdin, and the output is read from stdout (stdio is captured).
/// 
/// # Arguments
/// 
/// * `args` - A vector of strings representing the arguments to pass to pandoc.
/// * `input` - A vector of bytes representing the input to pass to pandoc.
/// 
/// # Returns
/// 
/// A string representing the output of pandoc.
/// 
/// # Errors
/// 
/// If there is an error reading or writing to the pipes, or if there is an error running the wasm module, an error is returned.
/// 
/// # Example
/// 
/// ```rust
/// use pandoc_wasm_wrapper::pandoc;
/// 
/// let args = vec!["--from=markdown".to_string(), "--to=html".to_string()];
/// let input = "# Hello, world!".as_bytes().to_vec();
/// let output = pandoc(&args, &input).unwrap();
/// 
/// assert_eq!(output, "<h1 id=\"hello-world\">Hello, world!</h1>\n");
/// ```
pub fn pandoc(
    args: &Vec<String>,
    input: &Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {    
    let mut store = Store::default();
    let module = Module::new(&store, WASM_BYTES)?;

    let (mut stdin_sender, stdin_reader) = Pipe::channel();
    let (stdout_sender, mut stdout_reader) = Pipe::channel();

    stdin_sender.write_all(input)?;
    drop(stdin_sender);

    WasiEnv::builder("pandoc")
        .args(args)
        .stdout(Box::new(stdout_sender))
        .stdin(Box::new(stdin_reader))
        .run_with_store(module, &mut store)?;

    let mut buf = String::new();
    stdout_reader.read_to_string(&mut buf).unwrap();

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandoc() {
        let args = vec!["--from=markdown".to_string(), "--to=html".to_string()];
        let input = "# Hello, world!".as_bytes().to_vec();
        let output = pandoc(&args, &input).unwrap();

        assert_eq!(output, "<h1 id=\"hello-world\">Hello, world!</h1>\n");
    }
}