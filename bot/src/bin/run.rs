use bot::{commands, event_handlers, on_error, Data};
use poise::event::Event;
use poise::serenity_prelude::GatewayIntents;
use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use std::{collections::HashMap, env::var, sync::Mutex};

type Error = Box<dyn std::error::Error + Send + Sync>;

async fn event_handler(
    ctx: &Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::InteractionCreate { interaction } => match interaction {
            // TODO: need to find a clean way to route component handling - might do what I did in the interactions implementation
            // this means prepending a unique name to each custom_id to help route to the right handler.
            Interaction::MessageComponent(component) => {
                event_handlers::examples::handle_spawned_button(ctx, component).await;
            }
            _ => {
                println!("Unhandled InteractionCreate: {:?}", interaction.kind());
            }
        },

        _ => {
            println!("Unhandled event: {}", event.name());
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![commands::examples::add(), commands::examples::spawnbutton()],
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
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
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .run()
        .await
        .unwrap();
}
