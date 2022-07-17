use super::super::command_parser::InteractionHandleError;
use crate::commands::hackerone::api_wrapper::get_report;
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse},
    model::interactions::{
        application_command::ApplicationCommandInteraction, message_component::ButtonStyle,
        InteractionResponseType,
    },
};

pub async fn bounty(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let bounty_values = vec![100, 500, 1000, 2500, 5000];
    let mut resp = CreateInteractionResponse::default();
    let mut action_row = CreateActionRow::default();

    let report_id = cmd
        .data
        .options
        .iter()
        .find(|o| o.name == "ticket_id")
        .ok_or(InteractionHandleError::MissingRequiredField("ticket_id"))?
        .value
        .as_ref()
        .map(|v| v.as_str())
        .ok_or("failed to parse ticket_id option")
        .unwrap()
        .unwrap_or("failed to unwrap ticket_id value");

    let report = get_report(report_id).await.unwrap();
    let report_severity = match report.data.relationships.severity {
        Some(severity) => severity.data.attributes.rating.unwrap(),
        None => "None".to_string(),
    };

    for value in bounty_values {
        let mut button1 = CreateButton::default();
        button1.custom_id(format!("hackerone_{}", value));
        button1.label(format!("${}", value));
        button1.style(ButtonStyle::Success);
        action_row.add_button(button1);
    }

    let mut bounty_embed = CreateEmbed::default();
    bounty_embed.title(format!("{}", report.data.attributes.title));
    bounty_embed.url(format!("https://hackerone.com/bugs?report_id={report_id}"));
    bounty_embed.field("Severity", format!("{}", report_severity), false);

    resp.kind(InteractionResponseType::ChannelMessageWithSource);
    resp.interaction_response_data(|rdata| {
        rdata
            .content("")
            .add_embed(bounty_embed)
            .components(|c| c.add_action_row(action_row))
    });

    Ok(resp)
}
