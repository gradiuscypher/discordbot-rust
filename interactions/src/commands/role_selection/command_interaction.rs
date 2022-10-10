// Allows a user to self-manage roles with a drop down message

use super::super::command_parser::InteractionHandleError;
use config::Config;
use serenity::builder::{
    CreateActionRow, CreateComponents, CreateInteractionResponse, CreateSelectMenu,
    CreateSelectMenuOption,
};
use serenity::http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;

pub async fn role_select(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    // get the client object to fetch roles from the guild
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    let token = settings.get_string("bot.token").unwrap();
    let application_id: u64 = settings.get("bot.application_id").unwrap();
    let http = http::Http::new_with_application_id(&token, application_id);

    // TODO: we could send a message to the channel from the command with channel_id.send_message, then use tokio::spawn to spawn it in another thread and respond to the original slash command with a "we're thinking" message
    // TODO: see banpool bot logic from past commits
    // fetch the guild's list of self-assignable roles (starts with .), use the provided metadata file for role descriptions, otherwise default to empty
    match cmd.guild_id {
        Some(user_guild) => {
            // identify which roles the user already has enabled and mark it as selected
            let mut guild_roles = user_guild.roles(&http).await.unwrap();

            // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
            guild_roles.retain(|_, role| role.name.starts_with("."));

            let mut user_roles = user_guild.member(&http, cmd.user.id).await.unwrap().roles;
            user_roles.retain(|role_id| guild_roles.contains_key(role_id));

            // build the final message and send as response
            let mut resp = CreateInteractionResponse::default();
            resp.kind(InteractionResponseType::ChannelMessageWithSource);

            let mut components = CreateComponents::default();
            let mut action_row = CreateActionRow::default();
            let mut select_menu = CreateSelectMenu::default();
            let mut opt_vec = Vec::new();

            for role in guild_roles {
                let mut option = CreateSelectMenuOption::default();
                option.label(&role.1.name);
                option.value(&role.1.id);
                option.default_selection(user_roles.contains(&role.0));
                opt_vec.push(option);
            }

            let option_count = (opt_vec.len() as u64).min(25);
            select_menu.min_values(0);
            select_menu.max_values(option_count);

            select_menu.custom_id("roles_selectmenu");
            select_menu.options(|options| options.set_options(opt_vec));

            action_row.add_select_menu(select_menu);
            components.add_action_row(action_row);

            resp.interaction_response_data(|msg| {
                msg.set_components(components);
                msg.ephemeral(true);
                msg
            });

            Ok(resp)
        }
        None => Err(InteractionHandleError::MissingGuildContext),
    }
}
