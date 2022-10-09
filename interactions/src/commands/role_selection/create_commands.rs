use anyhow::{Error, Result};
use log::error;
use serenity::http::{self};
use serenity::model::application::command::Command;
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
                .name("role_select")
                .description("Add or remove Discord roles")
        })
        .await;

    match new_command {
        Ok(command) => commands.push(command),
        Err(error) => error!("Unable to create slash command: {error}"),
    }
    Ok(commands)
}
