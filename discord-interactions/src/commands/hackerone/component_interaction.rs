use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::interactions::{
    message_component::MessageComponentInteraction, InteractionResponseType,
};

pub fn button_handler(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    let button_value = match cmd.data.custom_id.as_str() {
        "hackerone_100" => 100,
        "hackerone_500" => 500,
        "hackerone_1000" => 1000,
        "hackerone_2500" => 2500,
        "hackerone_5000" => 5000,
        _ => 0,
    };

    resp.interaction_response_data(|rdata| {
        rdata.content(format!("```$100 - 5 \n$500 - 3\n$1000 - 1```"));
        rdata.ephemeral(true)
    });
    Ok(resp)
}
