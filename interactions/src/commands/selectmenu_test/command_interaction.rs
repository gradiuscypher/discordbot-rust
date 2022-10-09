use super::super::command_parser::InteractionHandleError;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::InteractionResponseType;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn select_menu(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|cmd| {
        cmd.content("This is a select menu example.");
        cmd.components(|components| {
            components.create_action_row(|actionrow| {
                actionrow.create_select_menu(|menu| {
                    menu.custom_id("select_menu").options(|options| {
                        options.create_option(|option| {
                            option.label("option1");
                            option.description("This is select menu example option 1");
                            option.value("value1")
                        });
                        options.create_option(|option| {
                            option.label("option2");
                            option.description("This is select menu example option 2");
                            option.value("value2")
                        });
                        options.create_option(|option| {
                            option.label("option3");
                            option.description("This is select menu example option 3");
                            option.value("value3")
                        })
                    })
                })
            })
        });
        cmd
    });

    Ok(resp)
}
