use std::error;
use std::path::PathBuf;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Parser)]
struct Cli {
    // path to config file
    #[arg(short, long, required = false)]
    config: String,
}

fn get_default_config() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(format!("{}/.config/redproxy.yaml", home)))
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let mut cfg = args.config;
    if cfg == "" {
        cfg = get_default_config()?.into_os_string().into_string().unwrap();
    }
    println!("Config file {}", args.config);

    Ok(())
}
