use clap::{Parser, Subcommand};
use discord_interactions::commands::app_commands::install_commands;
use log::{error, info};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        #[clap(value_parser)]
        token: String,
        #[clap(value_parser)]
        app_id: u64,
        #[clap(value_parser)]
        guild_id: u64,
    },
}

async fn install_to_guild(token: &str, app_id: u64, guild_id: u64) {
    match install_commands(token, app_id, guild_id).await {
        Ok(commands) => info!("Created commands: {:?}", commands),
        Err(e) => error!("{e}"),
    }
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let args = Args::parse();

    match args.command {
        Commands::Install {
            token,
            app_id,
            guild_id,
        } => install_to_guild(&token, app_id, guild_id).await,
    }
}