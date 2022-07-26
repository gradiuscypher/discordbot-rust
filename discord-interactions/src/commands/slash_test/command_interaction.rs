use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::utils::Color;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn basic_slash(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    let ephemeral_arg = cmd.data.options.iter().find(|o| o.name == "ephemeral");

    let ephemeral = match ephemeral_arg {
        Some(argument) => argument
            .value
            .as_ref()
            .map(|v| v.as_bool())
            .ok_or("failed to parse ephemeral option")
            .unwrap()
            .unwrap_or(false),
        None => false,
    };

    resp.interaction_response_data(|cmd| {
        cmd.content("This is a simple slash command");
        cmd.embed(|embed| {
            embed.title("This is an embed title.")
                .description("This is the embed description field")
                .image("https://assets-global.website-files.com/5f9072399b2640f14d6a2bf4/62d0746010f70706d1341f68_image3.png")
                .fields(vec![("First Field", "This is first field's body, not inline", false), ("Second Field", "Second field body, inline", true), ("Third Field", "Third Field Body, inline", true)])
                .footer(|f| {
                    f.text("This is footer text").icon_url("https://assets-global.website-files.com/5f9072399b2640f14d6a2bf4/619429f77dd279b5a44f82d8_Author-Clyde-Webflow.png")
                })
                .author(|author| {
                    author.icon_url("https://assets-global.website-files.com/5f9072399b2640f14d6a2bf4/6238e57867278890ad005904_Policy%20%26%20Safety%20Team%20Icon.png")
                          .name("Author Name")
                })
                .color(Color::BLITZ_BLUE)
        });
        cmd.ephemeral(ephemeral);
        cmd
    });

    Ok(resp)
}

pub fn static_slash(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    let static_option = cmd
        .data
        .options
        .iter()
        .find(|o| o.name == "static_option")
        .ok_or(InteractionHandleError::MissingRequiredField(
            "static_option",
        ))?
        .value
        .as_ref()
        .map(|v| v.as_str())
        .ok_or("failed to parse static_option")
        .unwrap()
        .unwrap_or("");

    resp.interaction_response_data(|cmd| {
        cmd.content(format!("The static data you provided was: {static_option}"));
        cmd
    });

    Ok(resp)
}
