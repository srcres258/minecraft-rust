use std::fs::File;
use std::io::Read;
use anyhow::{anyhow, Result};

pub fn get_file_contents(file_path: &str) -> Result<String> {
    let mut result = String::new();
    let mut in_file = File::open(file_path)
        .map_err(|e| anyhow!("Unable to open file: {}, the error is: {}",
            file_path, e))?;
    in_file.read_to_string(&mut result)?;
    Ok(result)
}