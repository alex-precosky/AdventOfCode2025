use std::{fs, path::Path};

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Couldn't parse string: {0}")]
    ParseError(String),
}

/// Load an input file from the src/ directory of the calling crate into a string.
#[macro_export]
macro_rules! load_input_file_in_src_dir_to_string {
    ($filename:expr) => {{ $crate::load_input_file_in_src_dir_to_string_impl($filename, file!()) }};
}

/// The function that does the work of the above macro. Probably don't call this directly.
pub fn load_input_file_in_src_dir_to_string_impl<P: AsRef<Path> + std::fmt::Display>(
    path: P,
    caller_file: &str,
) -> anyhow::Result<String> {
    // The file!() macro returns a path relative to the workspace root (e.g., "day02/src/main.rs")
    // We need to find the workspace root and join with the caller's directory
    let caller_dir = Path::new(caller_file)
        .parent()
        .with_context(|| format!("Unable to read input file {:?}", caller_file))?;

    // Find the workspace root by walking up from current directory until we find Cargo.toml with [workspace]
    let mut workspace_root = std::env::current_dir()?;
    loop {
        let cargo_toml = workspace_root.join("Cargo.toml");
        if cargo_toml.exists() {
            let contents = fs::read_to_string(&cargo_toml)?;
            if contents.contains("[workspace]") {
                break;
            }
        }
        workspace_root = workspace_root
            .parent()
            .with_context(|| "Could not find workspace root")?
            .to_path_buf();
    }

    let input_path = workspace_root.join(caller_dir).join(&path);

    Ok(fs::read_to_string(&input_path)?)
}
