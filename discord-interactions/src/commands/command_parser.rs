use crate::commands::ro_application;

use anyhow::Result;
use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use config::Config;
use log::{debug, error};
use serenity::builder::CreateInteractionResponse;
use serenity::http;
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
}

impl IntoResponse for InteractionHandleError {
    fn into_response(self) -> Response {
        error!("error handling interaction: {self}");

        let status = match self {
            Self::MissingPayload | Self::MissingRequiredField(_) | Self::UnknownCommand(_) => {
                StatusCode::BAD_REQUEST
            }
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
        "apply" => ro_application::command_interaction::apply(cmd),
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
        _ => Err(InteractionHandleError::UnknownCommand(format!(
            "MessageComponent: {}",
            cmd.data.custom_id
        ))),
    }
}

pub async fn execute_modal(
    cmd: ModalSubmitInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    let token = settings.get_string("bot.token").unwrap();
    let application_id: u64 = settings.get("bot.application_id").unwrap();
    let client = http::Http::new_with_application_id(&token, application_id);

    debug!("{:?}", cmd.data.custom_id);

    match cmd.data.custom_id.as_str() {
        "apply" => ro_application::modal_interaction::apply(cmd, client).await,
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
