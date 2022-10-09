use super::super::command_parser::InteractionHandleError;
use log::info;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;

pub fn run_buttons(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();

    info!("{:?}", cmd.data.custom_id);

    match cmd.data.custom_id.as_str() {
        "button_1" => {
            resp.interaction_response_data(|rdata| {
                rdata.content("You pressed Button 1!");
                rdata
            });
        }

        "button_2" => {
            resp.interaction_response_data(|rdata| {
                rdata.content("You pressed Button 2!");
                rdata
            });
        }

        "button_3" => {
            resp.interaction_response_data(|rdata| {
                rdata.content("You pressed Button 3!");
                rdata
            });
        }
        _ => {
            resp.interaction_response_data(|rdata| {
                rdata.content(format!(
                    "Woah, never heard of {} before",
                    cmd.data.custom_id
                ));
                rdata
            });
        }
    }

    Ok(resp)
}
