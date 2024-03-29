extern crate lazy_static;

use axum::async_trait;
use axum::body;
use axum::body::{Bytes, HttpBody};
use axum::extract::rejection::BytesRejection;
use axum::extract::{FromRequest, Json, RequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use commands::command_parser::execute_autocomplete;
use commands::command_parser::{execute_command, execute_component};
use commands::command_parser::{execute_modal, InteractionHandleError};
use ed25519_dalek::PublicKey;
use serde_json::value::Value;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::application::interaction::{
    Interaction, InteractionResponseType, InteractionType,
};
use std::collections::HashMap;
use std::env;
use thiserror::Error;

use security::{verify_discord_message, SignatureValidationError};

pub mod commands;
mod security;

static SIGNATURE_HEADER: &str = "X-Signature-Ed25519";
static TIMESTAMP_HEADER: &str = "X-Signature-Timestamp";

lazy_static::lazy_static! {
    static ref KEY: PublicKey = get_key_from_env();
}

fn get_key_from_env() -> PublicKey {
    let key_bytes =
        hex::decode(env::var("DISCORD_PUBLIC_KEY").expect("DISCORD_PUBLIC_KEY unset")).unwrap();
    PublicKey::from_bytes(&key_bytes).unwrap()
}

#[derive(Debug, Error)]
pub enum InteractionPrepareError {
    #[error("error validating signature: {0}")]
    ValidationFailure(#[from] SignatureValidationError),
    #[error("missing signature header")]
    NoSignature,
    #[error("missing timestamp header")]
    NoTimestamp,
    #[error("missing payload")]
    NoPayload,
    #[error("error deserialising JSON payload {0}")]
    JSONError(#[from] serde_json::Error),
    #[error("error reading body {0}")]
    PayloadError(#[from] BytesRejection),
}

impl IntoResponse for InteractionPrepareError {
    fn into_response(self) -> Response {
        eprintln!("error handling request: {self}");

        let code = match self {
            Self::JSONError(_) | Self::PayloadError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::UNAUTHORIZED,
        };

        Response::builder()
            .status(code)
            .body(body::boxed(body::Empty::new()))
            .unwrap()
    }
}

pub type InteractionResponse = HashMap<&'static str, Value>;

pub struct InteractionRequest(Interaction);

#[async_trait]
impl<B> FromRequest<B> for InteractionRequest
where
    B: HttpBody + Send,
    <B as HttpBody>::Data: Send + Sync + 'static,
    <B as HttpBody>::Error: std::error::Error + Send + Sync + 'static,
{
    type Rejection = InteractionPrepareError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req).await?;
        let headers = req.headers().ok_or(InteractionPrepareError::NoSignature)?;

        let get = |k| headers.get(k).map(|v| v.to_str().unwrap());
        let sig = get(SIGNATURE_HEADER).ok_or(InteractionPrepareError::NoSignature)?;
        let ts = get(TIMESTAMP_HEADER).ok_or(InteractionPrepareError::NoTimestamp)?;

        let body = String::from_utf8(body.to_vec()).expect("invalid request body");
        verify_discord_message(&KEY, sig, ts, &body)?;

        let parsed: Interaction = serde_json::from_str(&body)?;

        Ok(Self(parsed))
    }
}

pub fn modal_interaction(inter: Interaction) -> Option<ModalSubmitInteraction> {
    match inter {
        Interaction::ModalSubmit(i) => Some(i),
        _ => None,
    }
}

pub async fn handle_interaction(
    data: InteractionRequest,
) -> Result<Json<InteractionResponse>, InteractionHandleError> {
    let data = data.0;
    let resp = match data.kind() {
        InteractionType::Ping => {
            let mut resp = CreateInteractionResponse::default();
            resp.kind(InteractionResponseType::Pong);

            resp
        }
        InteractionType::ApplicationCommand => {
            let cmd = data
                .application_command()
                .ok_or(InteractionHandleError::MissingPayload)?;

            match execute_command(cmd).await {
                Ok(result) => result,
                Err(_e) => return Err(_e),
            }
        }
        InteractionType::MessageComponent => {
            let cmd = data
                .message_component()
                .ok_or(InteractionHandleError::MissingPayload)?;

            match execute_component(cmd).await {
                Ok(result) => result,
                Err(_e) => return Err(_e),
            }
        }
        InteractionType::ModalSubmit => {
            let cmd = modal_interaction(data).ok_or(InteractionHandleError::MissingPayload)?;

            match execute_modal(cmd).await {
                Ok(result) => result,
                Err(_e) => return Err(_e),
            }
        }
        InteractionType::Autocomplete => {
            let cmd = data
                .autocomplete()
                .ok_or(InteractionHandleError::MissingPayload)?;

            match execute_autocomplete(cmd).await {
                Ok(result) => result,
                Err(_e) => return Err(_e),
            }
        }
        InteractionType::Unknown => todo!(),
        _ => todo!(),
    };

    Ok(Json(resp.0))
}
