use anyhow::{Context, Result};
use radius_virtual::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        parse(from_os_str),
        short = "c",
        long = "config",
        default_value = Config::system_path()
    )]
    config: std::path::PathBuf,

    #[structopt(short = "u", long = "username")]
    username: String,

    #[structopt(short = "p", long = "password")]
    password: String,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let config = Config::read_file(&args.config).context(format!(
        "Cannot read configuration from {}",
        args.config.to_string_lossy()
    ))?;

    let client = Client::try_with_config(&config)
        .context("Cannot initialize client with config")?;

    let cred =
        Credentials::with_username_password(args.username, args.password);
    let user = client
        .authenticate(&cred)
        .context("Authentication failure")?;

    let j = serde_json::to_string(&user)?;
    println!("{}", j);

    Ok(())
}
