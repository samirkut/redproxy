use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(target_family = "unix")]
pub fn default_script_path() -> Result<String> {
    let home = std::env::var("HOME")?;
    return Ok(home + "/.config/redproxy.star");
}

#[cfg(target_family = "windows")]
pub fn default_script_path() -> Result<String> {
    let userProfile = std::env::var("USERPROFILE")?;
    return Ok(userProfile + "\\.config\\redproxy.star");
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate script
    Validate {
        /// script to validate
        #[arg(required = false, default_value_t = String::from(""))]
        script: String,
    },
    /// Run the command with the specified script
    Run {
        /// script to run
        #[arg(required = false, default_value_t = String::from(""))]
        script: String,
    },
}
