use crate::{Context, Error};
use poise::serenity_prelude::{ButtonStyle, CreateButton, CreateComponents};
use poise::CreateReply;
use serenity::builder::CreateActionRow;

/// adds numbers together
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "An operand"] a: i8,
    #[description = "An operand"] b: u64,
    #[description = "An operand"]
    #[min = 0_i64]
    #[max = 1234567890987654_i64]
    c: i64,
) -> Result<(), Error> {
    ctx.say(format!("Result: {}", a as i128 + b as i128 + c as i128))
        .await?;
    Ok(())
}

// ref: https://github.com/serenity-rs/poise/blob/develop/examples/framework_usage/commands/general.rs#L103
/// spawns a clickable button
#[poise::command(slash_command)]
pub async fn spawnbutton(ctx: Context<'_>) -> Result<(), Error> {
    // create the reply where we're going to put all our components together
    let mut reply = CreateReply::default();

    // create the components object
    let mut components = CreateComponents::default();

    // create the action row that will contain our button
    let mut action_row = CreateActionRow::default();

    // create the button we'll add to our reply
    let mut button = CreateButton::default();
    button.custom_id("custom_button_id");
    button.label("custom_button_label");
    button.style(ButtonStyle::Danger);

    // glue everything together
    action_row.add_button(button);
    components.add_action_row(action_row);
    reply.components = Some(components);

    ctx.send(|data| {
        *data = reply;
        data
    })
    .await
    .unwrap();

    Ok(())
}
