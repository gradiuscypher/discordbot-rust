use super::super::command_parser::InteractionHandleError;
use config::Config;
use serenity::http;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::{builder::CreateInteractionResponse, model::prelude::RoleId};

pub async fn process_role_select(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let mut response = CreateInteractionResponse::default();
    response.interaction_response_data(|msg| {
        msg.content("Your roles have been set!");
        msg.ephemeral(true)
    });

    // get the client object to fetch roles from the guild
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();
    let token = settings.get_string("bot.token").unwrap();
    let application_id: u64 = settings.get("bot.application_id").unwrap();
    let http = http::Http::new_with_application_id(&token, application_id);

    match cmd.guild_id {
        Some(user_guild) => {
            // identify which roles the user already has enabled and mark it as selected
            let mut guild_roles = user_guild.roles(&http).await.unwrap();

            // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
            guild_roles.retain(|_, role| role.name.starts_with("."));

            // list of roles that are self assignable
            let mut target_member = user_guild.member(&http, cmd.user.id).await.unwrap();
            let mut user_roles = target_member.roles.clone();
            user_roles.retain(|role_id| guild_roles.contains_key(role_id));

            // iterate over all the user roles, if it's not in the new list, remove it, then remove that item from the array
            let mut remaining_roles = cmd.data.values.clone();
            for role_id in user_roles {
                if !cmd.data.values.contains(&role_id.to_string()) {
                    target_member.remove_role(&http, role_id).await.unwrap();
                    remaining_roles.retain(|val| val != &role_id.to_string());
                }
            }

            // assign all remaining roles in the list
            for role_id in remaining_roles {
                let target_role: RoleId = role_id.parse().unwrap();
                if guild_roles.contains_key(&target_role) {
                    target_member.add_role(&http, target_role).await.unwrap();
                }
            }

            Ok(response)
        }
        None => Err(InteractionHandleError::MissingGuildContext),
    }
}
