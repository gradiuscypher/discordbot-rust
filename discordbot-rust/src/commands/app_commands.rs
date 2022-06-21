use anyhow::{anyhow, Error, Result};
use log::{error, info};
use serenity::http::{self};
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::{builder::CreateApplicationCommand, model::id::GuildId};

pub async fn install_commands(
    token: &str,
    app_id: u64,
    guild_id: u64,
) -> Result<Vec<ApplicationCommand>, Error> {
    let application_commands: Vec<CreateApplicationCommand> = Vec::new();
    let client = http::Http::new_with_application_id(&token, app_id);
    // let target_guild = client.get_guild(guild_id).await.unwrap();
    let target_guild = GuildId(guild_id);

    let commands = target_guild
        .set_application_commands(client, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("echo_modal")
                    .description("Creates an echo modal example")
            })
        })
        .await;

    match commands {
        Ok(commands) => Ok(commands),
        Err(e) => Err(anyhow!("Unable to create commands: {e}")),
    }
}
