use bot::{commands, on_error, Data};
use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex};

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![commands::examples::add(), commands::examples::spawnbutton()],
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN env var."))
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}
