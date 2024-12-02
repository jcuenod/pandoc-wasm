use reqwest;
use std::{fs::File, io::{Read, Write}};

// This build script downloads the pandoc.wasm file from the internet and saves it to the target directory.
// The build script will only run if the pandoc.wasm file does not exist in the target directory.
const SOURCE_URL: &str = "https://tweag.github.io/pandoc-wasm/pandoc.wasm";

// error that could be std:io::Error or reqwest::Error
type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let target_file = std::env::var("OUT_DIR")? + "/pandoc.wasm";
    if std::path::Path::new(&target_file).exists() {
        return Ok(());
    }

    let mut pandoc_wasm = reqwest::blocking::get(SOURCE_URL)?;

    let mut pandoc_wasm_bytes = Vec::new();
    pandoc_wasm.read_to_end(&mut pandoc_wasm_bytes)?;
    let mut pandc_wasm_file = File::create(target_file)?;
    pandc_wasm_file.write_all(&pandoc_wasm_bytes)?;
    Ok(())
}