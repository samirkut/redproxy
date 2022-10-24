use anyhow::Result;
use std::fs;

fn read_script(script: &str) -> Result<String> {
    let content = fs::read_to_string(script)?;
    Ok(content)
}

pub fn evaluate(script: &str) -> Result<()> {
    let content = read_script(script)?;
    println!("Script: {}", content);
    Ok(())
}
