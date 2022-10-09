use clap::{Parser, Subcommand};
// use discord_interactions::commands::{button_test, modal_test, selectmenu_test, slash_test};
use discord_interactions::commands::role_selection;
use log::{error, info};
use serenity::http;
use serenity::model::id::GuildId;

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
    // examples for installing commands
    // match modal_test::create_commands::install_commands(token, app_id, guild_id).await {
    //     Ok(commands) => info!("Created commands: {:?}", commands),
    //     Err(e) => error!("{e}"),
    // }
    // match slash_test::create_commands::install_commands(token, app_id, guild_id).await {
    //     Ok(commands) => info!("Created commands: {:?}", commands),
    //     Err(e) => error!("{e}"),
    // }
    // match button_test::create_commands::install_commands(token, app_id, guild_id).await {
    //     Ok(commands) => info!("Created commands: {:?}", commands),
    //     Err(e) => error!("{e}"),
    // }
    // match selectmenu_test::create_commands::install_commands(token, app_id, guild_id).await {
    //     Ok(commands) => info!("Created commands: {:?}", commands),
    //     Err(e) => error!("{e}"),
    // }

    // clear current commands from the target guild
    let http = http::Http::new_with_application_id(&token, app_id);
    let target_guild = GuildId(guild_id);
    let current_commands = target_guild.get_application_commands(&http).await.unwrap();

    for command in current_commands {
        match target_guild
            .delete_application_command(&http, command.id)
            .await
        {
            Ok(_) => info!("Deleted command: {:?}", command.name),
            Err(e) => error!("{e}"),
        }
    }

    // Install the various commands
    match role_selection::create_commands::install_commands(token, app_id, guild_id).await {
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
