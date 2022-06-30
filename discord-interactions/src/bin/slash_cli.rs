use clap::{Parser, Subcommand};
use discord_interactions::commands::hackerone;
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
    Rest {
        #[clap(value_parser)]
        report_id: String,
        #[clap(value_parser)]
        username: String,
        #[clap(value_parser)]
        api_key: String,
    },
}

async fn install_to_guild(token: &str, app_id: u64, guild_id: u64) {
    match hackerone::create_commands::install_commands(token, app_id, guild_id).await {
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
        Commands::Rest {
            report_id,
            username,
            api_key,
        } => {
            discord_interactions::commands::hackerone::api_wrapper::get_report(&report_id)
                .await
                .unwrap();
        }
    }
}
