use log::{error, info};
use serenity::http::{self};

async fn install_to_guild(token: &str, app_id: u64, guild_id: u64) {
    let client = http::Http::new_with_application_id(&token, app_id);
    let target_guild = client.get_guild(guild_id).await;

    match target_guild {
        Ok(guild) => guild.set_application_commands(client, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("echo_modal")
                    .description("Creates an example echo modal.")
            })
        }),
        Err(e) => error!("Unabled to fetch guild: {}", e),
    }
}
