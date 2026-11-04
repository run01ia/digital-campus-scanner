mod config;

use anyhow::Result;
use config::Config;

fn main() -> Result<()> {

    let config = Config::load("config.toml")?;

    println!("Config.toml loaded.");
    println!("Output dir: {}", config.general.output_dir);

    Ok(())
}
