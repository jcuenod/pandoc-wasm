use once_cell::sync::{Lazy, OnceCell};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use wasmer::{Engine, Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

static WASM_BYTES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pandoc.wasm"));
static MODULE_CACHE: OnceCell<Module> = OnceCell::new();
static ENGINE: Lazy<Engine> = Lazy::new(|| Engine::default());

/// Calls the pandoc wasm module with the given arguments and input bytes. The input is passed to pandoc via stdin, and the output is read from stdout (stdio is captured).
///
/// # Arguments
///
/// * `args` - A vector of strings representing the arguments to pass to pandoc.
/// * `input` - A vector of bytes representing the input to pass to pandoc.
///
/// # Returns
///
/// A Future Result that contains either the string representing the output of pandoc or an std:error:Error.
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
/// let output = pandoc(&args, &input).await.unwrap();
///
/// assert_eq!(output, "<h1 id=\"hello-world\">Hello, world!</h1>\n");
/// ```
static PANDOC_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub async fn pandoc(
    args: &Vec<String>,
    input: &Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    let _guard = PANDOC_LOCK.lock().await;

    let mut store = Store::new(ENGINE.clone());
    let module = MODULE_CACHE.get_or_try_init(|| Module::new(&store, WASM_BYTES))?;

    let (mut stdin_sender, stdin_reader) = Pipe::channel();
    let (stdout_sender, mut stdout_reader) = Pipe::channel();

    stdin_sender.write_all(input).await?;
    drop(stdin_sender);

    WasiEnv::builder("pandoc")
        .args(args)
        .stdout(Box::new(stdout_sender))
        .stdin(Box::new(stdin_reader))
        .run_with_store(module.clone(), &mut store)?;

    let mut buf = String::new();
    stdout_reader.read_to_string(&mut buf).await.unwrap();

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pandoc() {
        let args = vec!["--from=markdown".to_string(), "--to=html".to_string()];
        let input = "# Hello, world!".as_bytes().to_vec();
        let output = pandoc(&args, &input).await.unwrap();

        assert_eq!(output, "<h1 id=\"hello-world\">Hello, world!</h1>\n");
    }
}
