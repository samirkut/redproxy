use anyhow::Result;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    // path to config file
    #[arg(short, long)]
    #[clap(default_value="~/.config/redproxy.yaml")]
    config: String,
}

fn expand_path(p: &str) -> Result<String> {
    if p.starts_with("~/") {
        let home = std::env::var("HOME")?;
        let ret = p.replacen('~', home.as_str(), 1);
        Ok(ret)
    }else{
        Ok(String::from(p))
    }
}

fn main() -> Result<()> {
    let mut args = Cli::parse();
    args.config = expand_path(&args.config)?;
    
    println!("Config file {}", args.config);

    Ok(())
}
