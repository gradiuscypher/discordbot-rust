use clap::{Parser, Subcommand};
use log::{error, info};
use serenity::http::{self};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        token: String,
        app_id: u64,
        guild_id: u64,
    },
}

async fn install_to_guild(token: &str, app_id: u64, guild_id: u64) {
    let client = http::Http::new_with_application_id(&token, app_id);
    let target_guild = client.get_guild(guild_id).await;

    match target_guild {
        Ok(guild) => {
            // guild.set_application_commands(http, |commands| {
            //     commands.create_application_command(|command| {
            //         command.name()
            //     })
            // })
        }
        Err(e) => error!("Unabled to fetch guild: {}", e),
    }
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let args = Args::parse();

    match args.command {
        Commands::Install {
            token,
            app_id,
            guild_id,
        } => install_to_guild(&token, app_id, guild_id).await,
    }
}
