use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(target_family = "unix")]
fn default_script_path() -> Result<String> {
    let home = std::env::var("HOME")?;
    return Ok(home + "/.config/redproxy.star");
}

#[cfg(target_family = "windows")]
fn default_script_path() -> Result<String> {
    let userProfile = std::env::var("USERPROFILE")?;
    return Ok(userProfile + "\\.config\\redproxy.star");
}

#[derive(Subcommand)]
enum Commands {
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

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Validate { script } => validate(script),
        Commands::Run { script } => run(script),
    }
}

fn validate(mut script: String) -> Result<()> {
    if script == "" {
        script = default_script_path()?;
    }
    println!("Validate script: {}", script);

    Ok(())
}

fn run(mut script: String) -> Result<()> {
    if script == "" {
        script = default_script_path()?;
    }
    println!("Run script: {}", script);
    
    Ok(())
}
