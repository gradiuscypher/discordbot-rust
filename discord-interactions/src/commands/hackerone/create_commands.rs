use anyhow::{anyhow, Error, Result};
use serenity::http::{self};
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};

pub async fn install_commands(
    token: &str,
    app_id: u64,
    guild_id: u64,
) -> Result<Vec<ApplicationCommand>, Error> {
    let client = http::Http::new_with_application_id(&token, app_id);
    let target_guild = GuildId(guild_id);

    let commands = target_guild
        .set_application_commands(client, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("bounty")
                    .description("Start a poll for H1 bounty amount")
                    .create_option(|option| {
                        option
                            .name("ticket_id")
                            .description("The H1 ticket ID")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
        })
        .await;

    match commands {
        Ok(commands) => Ok(commands),
        Err(e) => Err(anyhow!("Unable to create commands: {e}")),
    }
}
