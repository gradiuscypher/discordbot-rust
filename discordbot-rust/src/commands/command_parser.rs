use super::debug_response::debug_one;
use super::modal_tests::echo_modal;
use anyhow::Result;
use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serenity::builder::CreateInteractionResponse;
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, modal::ModalSubmitInteraction,
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
        eprintln!("error handling interaction: {self}");

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
    match cmd.data.name.as_str() {
        "echo_modal" => echo_modal(cmd.clone()),
        _ => Err(InteractionHandleError::UnknownCommand(cmd.data.name)),
    }
}

pub async fn execute_component(
    cmd: ModalSubmitInteraction,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    println!("{:?}", cmd.data.custom_id);
    match cmd.data.custom_id.as_str() {
        "echo_modal" => debug_one(cmd.clone()),
        _ => Err(InteractionHandleError::UnknownCommand(cmd.data.custom_id)),
    }
}
