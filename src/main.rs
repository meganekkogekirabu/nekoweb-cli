use std::process;

use clap::{Parser, Subcommand};
use nekoweb_rs::{Authenticated, Client};
use serde::{Deserialize, Serialize};

mod account;
mod files;
mod info;
mod push;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    api_key: String,
}

struct Neko {
    client: Client<Authenticated>,
    config: Config,
    config_folder: String,
    config_file: String,
}

#[derive(Parser, Debug)]
#[command(name = "neko", version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Import current directory to your site")]
    Push { directory: Option<String> },

    #[command(
        about = "Get info about site",
        long_about = "Get info about site. Defaults to your site."
    )]
    Info { site: Option<String> },

    #[command(about = "Get API key and save to config file")]
    Login,

    #[command(about = "Remove API key from config file")]
    Logout,

    #[command(about = "Move file")]
    Mv { from: String, to: String },

    #[command(about = "List files in directory")]
    Ls { dir: String },

    #[command(about = "Create empty file")]
    Touch { path: String },

    #[command(about = "Create empty directory")]
    Mkdir { path: String },

    #[command(about = "Remove file or directory")]
    Rm { path: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config_folder = String::from("nekoweb-cli");
    let config_file = String::from("config");
    let config: Config = confy::load(&config_folder, Some(&*config_file))?;
    let client = Client::new("nekoweb-cli")?.authenticate(config.api_key.clone());

    let mut neko = Neko {
        client,
        config,
        config_folder,
        config_file,
    };

    if neko.config.api_key.is_empty() {
        neko.login()?;
        process::exit(0);
    }

    match args.command {
        Commands::Push { directory } => {
            let directory = directory.unwrap_or(".".to_string());
            neko.push(directory).await?;
        }
        Commands::Info { site } => neko.info(site).await?,
        Commands::Login => neko.login()?,
        Commands::Logout => neko.logout()?,
        Commands::Mv { from, to } => neko.rename(from, to).await?,
        Commands::Ls { dir } => neko.list(dir).await?,
        Commands::Touch { path } => neko.touch(path).await?,
        Commands::Mkdir { path } => neko.mkdir(path).await?,
        Commands::Rm { path } => neko.remove(path).await?,
    }

    Ok(())
}
