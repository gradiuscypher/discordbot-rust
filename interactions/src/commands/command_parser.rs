use crate::commands::{button_test, modal_test, role_selection, selectmenu_test, slash_test};

use anyhow::Result;
use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::{debug, error};
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::autocomplete::AutocompleteInteraction;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InteractionHandleError {
    #[error("no payload in data")]
    MissingPayload,
    #[error("unknown command {0}")]
    UnknownCommand(String),
    #[error("missing required field {0}")]
    MissingRequiredField(&'static str),
    #[error("No Guild ID in command context")]
    MissingGuildContext,
}

impl IntoResponse for InteractionHandleError {
    fn into_response(self) -> Response {
        error!("error handling interaction: {self}");

        let status = match self {
            Self::MissingPayload
            | Self::MissingRequiredField(_)
            | Self::UnknownCommand(_)
            | Self::MissingGuildContext => StatusCode::BAD_REQUEST,
        };

        Response::builder()
            .status(status)
            .body(body::boxed(body::Empty::new()))
            .unwrap()
    }
}

pub async fn execute_command(
    cmd: ApplicationCommandInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("ApplicationCommandInteraction: {:?}", cmd.data.name);
    match cmd.data.name.as_str() {
        "demo_modal" => modal_test::command_interaction::demo_modal(cmd),
        "basic_slash" => slash_test::command_interaction::basic_slash(cmd),
        "static_slash" => slash_test::command_interaction::static_slash(cmd),
        "buttons" => button_test::command_interaction::make_buttons(cmd),
        "select_menu" => selectmenu_test::command_interaction::select_menu(cmd),
        "role_select" => role_selection::command_interaction::role_select(cmd).await,
        _ => Err(InteractionHandleError::UnknownCommand(format!(
            "ApplicationCommand: {}",
            cmd.data.name
        ))),
    }
}

pub async fn execute_component(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("MessageComponentInteraction: {:?}", cmd.data.custom_id);
    let command: &str = cmd.data.custom_id.as_str().split("_").next().unwrap();
    match command {
        "button" => button_test::component_interaction::run_buttons(cmd),
        "roles" => role_selection::component_interaction::process_role_select(cmd).await,
        _ => Err(InteractionHandleError::UnknownCommand(format!(
            "MessageComponent: {}",
            cmd.data.custom_id
        ))),
    }
}

pub async fn execute_modal(
    cmd: ModalSubmitInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("{:?}", cmd.data.custom_id);
    match cmd.data.custom_id.as_str() {
        "echo_modal" => modal_test::modal_interaction::debug_one(cmd),
        _ => Err(InteractionHandleError::UnknownCommand(format!(
            "ModalSubmit: {}",
            cmd.data.custom_id
        ))),
    }
}

pub async fn execute_autocomplete(
    cmd: AutocompleteInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("data: {:?}", cmd.data);
    // get the command option that's currently focused
    let command: &str = cmd
        .data
        .options
        .iter()
        .find(|f| f.focused == true)
        .unwrap()
        .name
        .as_str()
        .split("_")
        .next()
        .unwrap();

    debug!("command: {command}");

    match command {
        _ => Err(InteractionHandleError::UnknownCommand(format!(
            "AutocompleteInteraction: {}",
            command
        ))),
    }
}
