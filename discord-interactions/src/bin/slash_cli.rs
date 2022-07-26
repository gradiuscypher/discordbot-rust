use clap::{Parser, Subcommand};
use discord_interactions::commands::{button_test, modal_test, selectmenu_test, slash_test};
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
    match modal_test::create_commands::install_commands(token, app_id, guild_id).await {
        Ok(commands) => info!("Created commands: {:?}", commands),
        Err(e) => error!("{e}"),
    }
    match slash_test::create_commands::install_commands(token, app_id, guild_id).await {
        Ok(commands) => info!("Created commands: {:?}", commands),
        Err(e) => error!("{e}"),
    }
    match button_test::create_commands::install_commands(token, app_id, guild_id).await {
        Ok(commands) => info!("Created commands: {:?}", commands),
        Err(e) => error!("{e}"),
    }
    match selectmenu_test::create_commands::install_commands(token, app_id, guild_id).await {
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
