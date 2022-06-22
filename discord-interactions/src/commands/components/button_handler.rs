use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::interactions::{
    message_component::MessageComponentInteraction, InteractionResponseType,
};

pub fn button_handler(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::UpdateMessage);

    resp.interaction_response_data(|rdata| rdata.content("The button was pressed."));
    Ok(resp)
}
