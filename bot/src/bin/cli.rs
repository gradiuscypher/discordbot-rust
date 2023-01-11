use bot::{commands, Data};
use poise::serenity_prelude as serenity;
use std::env::var;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let commands = vec![commands::examples::add(), commands::examples::spawnbutton()];
    let create_commands = poise::builtins::create_application_commands(&commands);
    let http = serenity::http::Http::new_with_application_id(
        &var("DISCORD_TOKEN")
            .expect("Expected DISCORD_TOKEN")
            .to_string(),
        var("APP_ID").expect("Expected APP_ID").parse().unwrap(),
    );
    let target_guild = serenity::GuildId(268239941195137025);
    let result = target_guild
        .set_application_commands(http, |f| {
            *f = create_commands;
            f
        })
        .await;

    match result {
        Ok(commands) => println!("Set commands successfully: {:?}", commands),
        Err(e) => println!("Error setting commands: {e}"),
    }
}
