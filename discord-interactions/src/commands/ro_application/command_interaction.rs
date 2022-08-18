use super::super::command_parser::InteractionHandleError;
use serenity::builder::{CreateInteractionResponse, CreateSelectMenuOption};
use serenity::model::application::component::InputTextStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;

static CLASSES: &[&str] = &[
    "Priest",
    "Monk",
    "Hunter",
    "Bard",
    "Dancer",
    "Wizard",
    "Sage",
    "Blacksmith",
    "Alchemist",
    "Knight",
    "Crusader",
    "Assassin",
    "Rogue",
];

pub fn apply(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut resp = CreateInteractionResponse::default();
    resp.kind(InteractionResponseType::Modal);

    let mut class_options: Vec<CreateSelectMenuOption> = Vec::new();
    for class in CLASSES {
        let new_option = CreateSelectMenuOption::new(class, class);
        class_options.push(new_option);
    }

    resp.interaction_response_data(|cmd| {
        cmd.custom_id("apply");
        cmd.title("Guild Application");
        cmd.components(|c| {
            c.create_action_row(|ar| {
                ar.create_input_text(|it| {
                    it.style(InputTextStyle::Short)
                        .label("Character Name")
                        .custom_id("character_name")
                })
            })
        });
        cmd
    });

    Ok(resp)
}
