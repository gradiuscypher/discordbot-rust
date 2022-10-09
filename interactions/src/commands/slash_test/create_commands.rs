use anyhow::{Error, Result};
use log::error;
use serenity::http::{self};
use serenity::model::application::command::Command;
use serenity::model::application::command::CommandOptionType;
use serenity::model::id::GuildId;

pub async fn install_commands(
    token: &str,
    app_id: u64,
    guild_id: u64,
) -> Result<Vec<Command>, Error> {
    let client = http::Http::new_with_application_id(&token, app_id);
    let target_guild = GuildId(guild_id);

    let mut commands: Vec<Command> = Vec::new();

    let new_command = target_guild
        .create_application_command(&client, |command| {
            command
                .name("basic_slash")
                .description("Sends a message in response to a slash command")
                .create_option(|option| {
                    option
                        .name("ephemeral")
                        .description("Should the message be ephemeral")
                        .kind(CommandOptionType::Boolean)
                        .required(false)
                })
        })
        .await;

    match new_command {
        Ok(command) => commands.push(command),
        Err(error) => error!("Unable to create slash command: {error}"),
    }

    let new_command = target_guild
        .create_application_command(&client, |command| {
            command
                .name("static_slash")
                .description("Sends a message selected from the static options available")
                .create_option(|option| {
                    option
                        .name("static_option")
                        .description("Select from a few static options")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Choice One", "This is Choice 1")
                        .add_string_choice("Choice Two", "This is Choice 2")
                        .add_string_choice("Choice Three", "This is Choice 3")
                        .required(true)
                })
        })
        .await;

    match new_command {
        Ok(command) => commands.push(command),
        Err(error) => error!("Unable to create slash command: {error}"),
    }

    Ok(commands)
}
