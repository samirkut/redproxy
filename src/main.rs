use anyhow::Result;
use clap::{Parser};
use log::info;
use pretty_env_logger;
use std::path::Path;

mod args;
use args::{Cli, Commands, default_script_path};

mod evaluator;
use evaluator::{evaluate};

fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        pretty_env_logger::init();
    } else {
        // in release mode enable timestamps
        pretty_env_logger::init_timed();
    }

    info!("starting redproxy");

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
    info!("Validate script: {}", script);

    let p = Path::new(&script);
    let m = evaluate(p, "OnInitialize")?;

    for (k,v) in m {
        println!("{}: {}", k, v);
    }

    Ok(())
}

fn run(mut script: String) -> Result<()> {
    if script == "" {
        script = default_script_path()?;
    }
    info!("Run script: {}", script);

    Ok(())
}
