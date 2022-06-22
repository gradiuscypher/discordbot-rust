use serenity::model::interactions::modal::ModalSubmitInteraction;

use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;

pub fn debug_one(
    cmd: ModalSubmitInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();

    println!("{:?}", cmd.data.components);

    resp.interaction_response_data(|rdata| {
        rdata.content("A debug message!");
        rdata
    });

    Ok(resp)
}
