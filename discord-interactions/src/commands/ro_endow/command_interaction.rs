use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::utils::Color;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn endow(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|cmd| {
        cmd.embed(|embed| {
            embed.title("Endow Request - Fire")
                .description("Your endow request has been recieved. There are currently 5 Sages online and one will PM you soon!")
                .thumbnail("https://file5s.ratemyserver.net/items/large/990.gif")
                .fields(vec![("Element", "Fire", true), ("Your Stone", "Yes", true)])
                .footer(|f| {
                    f.text("This service is provided by the Star Ocean guild. Sign up to be an endow Sage with the /sage command.")
                })
                .color(Color::RED)
        });
        cmd.ephemeral(true);
        cmd
    });

    Ok(resp)
}

pub fn sage(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|cmd| {
        cmd.content(format!("Hello sage."));
        cmd
    });

    Ok(resp)
}
