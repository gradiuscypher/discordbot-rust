use crate::commands::hackerone;

use anyhow::Result;
use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::{debug, error};
use serenity::builder::CreateInteractionResponse;
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction,
    message_component::MessageComponentInteraction, modal::ModalSubmitInteraction,
};
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
        "bounty" => hackerone::command_interaction::bounty(cmd.clone()),
        _ => Err(InteractionHandleError::UnknownCommand(cmd.data.name)),
    }
}

pub async fn execute_component(
    cmd: MessageComponentInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("MessageComponentInteraction: {:?}", cmd.data.custom_id);
    let command: &str = cmd.data.custom_id.as_str().split("_").next().unwrap();
    match command {
        "hackerone" => hackerone::component_interaction::button_handler(cmd),
        _ => Err(InteractionHandleError::UnknownCommand(cmd.data.custom_id)),
    }
}

pub async fn execute_modal(
    cmd: ModalSubmitInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    debug!("{:?}", cmd.data.custom_id);
    match cmd.data.custom_id.as_str() {
        _ => Err(InteractionHandleError::UnknownCommand(cmd.data.custom_id)),
    }
}
