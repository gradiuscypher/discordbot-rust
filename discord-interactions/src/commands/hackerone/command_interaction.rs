use super::super::command_parser::InteractionHandleError;
use log::info;
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateInteractionResponse},
    model::interactions::{
        application_command::ApplicationCommandInteraction, message_component::ButtonStyle,
        InteractionResponseType,
    },
};

pub fn bounty(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let bounty_values = vec![100, 500, 1000, 2500, 5000];
    let mut resp = CreateInteractionResponse::default();
    let mut action_row = CreateActionRow::default();

    for value in bounty_values {
        let mut button1 = CreateButton::default();
        button1.custom_id(format!("hackerone_{}", value));
        button1.label(format!("${}", value));
        button1.style(ButtonStyle::Success);
        action_row.add_button(button1);
    }

    info!("Doing bounty");
    resp.kind(InteractionResponseType::ChannelMessageWithSource);
    resp.interaction_response_data(|rdata| {
        rdata.content("Click the bounty you feel is appropriate");
        rdata.components(|c| c.add_action_row(action_row));
        rdata
    });

    Ok(resp)
}
