use super::command_parser::InteractionHandleError;
use serenity::{
    builder::CreateInteractionResponse,
    model::interactions::{
        application_command::ApplicationCommandInteraction, message_component::ButtonStyle,
        InteractionResponseType,
    },
};

pub fn bounty(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|rdata| {
        rdata.content("Click the bounty you feel is appropriate");
        rdata.components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(ButtonStyle::Primary);
                    b.label("Button1")
                });
                ar.create_button(|b| {
                    b.style(ButtonStyle::Primary);
                    b.label("Button2")
                })
            })
        });
        rdata
    });

    Ok(resp)
}
