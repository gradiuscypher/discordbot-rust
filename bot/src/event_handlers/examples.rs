use serenity::client::Context;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;

pub async fn handle_spawned_button(ctx: &Context, c: &MessageComponentInteraction) {
    println!(
        "MessageComponent {} was clicked by {}",
        c.data.custom_id, c.user.name
    );

    c.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|data| data.content("Thanks for pressing that."))
    })
    .await
    .unwrap();
}
