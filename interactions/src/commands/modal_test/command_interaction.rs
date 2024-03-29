use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::InteractionResponseType;

use serenity::model::application::component::InputTextStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn demo_modal(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::Modal);
    resp.interaction_response_data(|cmd| {
        cmd.custom_id("echo_modal");
        cmd.title("Modal Example Title");
        cmd.components(|c| {
            c.create_action_row(|ar| {
                ar.create_input_text(|it| {
                    it.style(InputTextStyle::Paragraph)
                        .label("Input the text!")
                        .custom_id("custom_text")
                })
            })
        });
        cmd
    });

    Ok(resp)
}
