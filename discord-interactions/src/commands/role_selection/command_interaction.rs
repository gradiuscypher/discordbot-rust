// Allows a user to self-manage roles with a drop down message

use super::super::command_parser::InteractionHandleError;
use config::Config;
use serenity::builder::CreateInteractionResponse;
use serenity::http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;

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

pub fn role_select(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    // get the client object to fetch roles from the guild
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    let token = settings.get_string("bot.token").unwrap();
    let application_id: u64 = settings.get("bot.application_id").unwrap();
    let client = http::Http::new_with_application_id(&token, application_id);

    // fetch the guild's list of self-assignable roles (starts with .), use the provided metadata file for role descriptions, otherwise default to empty
    // TODO: how can I raise an error here?
    match cmd.guild_id {
        Some(guild_id) => {
            println!("has guild id")
        }
        None => println!("no has guild id"),
    }

    // identify which roles the user already has enabled and mark it as selected

    // build the final message and send as response
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::ChannelMessageWithSource);

    resp.interaction_response_data(|msg| {
        msg.content("This is a select menu example.");
        msg
    });

    Ok(resp)
}
