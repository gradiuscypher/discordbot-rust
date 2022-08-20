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
                .name("endow")
                .description("Request an endow from Sages that are currently online")
                .create_option(|option| {
                    option
                        .name("element")
                        .description("Which element endow do you need?")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Fire", "fire")
                        .add_string_choice("Earth", "earth")
                        .add_string_choice("Wind", "wind")
                        .add_string_choice("Water", "water")
                        .required(true)
                })
                .create_option(|option| {
                    option
                        .name("stone")
                        .description("Do you have the stone?")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Yes", "yes")
                        .add_string_choice("No", "no")
                        .required(true)
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
                .name("sage")
                .description("Add or remove yourself from the list of available Sages for endows.")
                .create_option(|option| {
                    option
                        .name("status")
                        .description("Are you online to respond to endow requests?")
                        .kind(CommandOptionType::String)
                        .add_string_choice("Yes", "yes")
                        .add_string_choice("No", "no");
                    option
                        .name("stones")
                        .description(
                            "Can you provide stones for the endow at for an increased cost?",
                        )
                        .kind(CommandOptionType::String)
                        .add_string_choice("Yes", "yes")
                        .add_string_choice("No", "no")
                })
        })
        .await;

    match new_command {
        Ok(command) => commands.push(command),
        Err(error) => error!("Unable to create slash command: {error}"),
    }

    Ok(commands)
}
