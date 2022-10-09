use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn make_buttons(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|cmd| {
        cmd.content("This is a simple slash command");
        cmd.components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(ButtonStyle::Primary);
                    b.label("Button One");
                    b.custom_id("button_1")
                });
                ar.create_button(|b| {
                    b.style(ButtonStyle::Primary);
                    b.label("Button Two");
                    b.custom_id("button_2")
                });
                ar.create_button(|b| {
                    b.style(ButtonStyle::Primary);
                    b.label("Button Three");
                    b.custom_id("button_3")
                })
            })
        });
        cmd
    });

    Ok(resp)
}
