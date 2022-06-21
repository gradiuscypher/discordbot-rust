use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};
use hex::FromHexError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignatureValidationError {
    #[error("Invalid signature encoding ({0})")]
    InvalidEncoding(#[from] FromHexError),
    #[error("Invalid signature length {0} bytes != 64")]
    InvalidLength(usize),
    #[error("Invalid signature")]
    InvalidError(#[from] SignatureError),
}

// stolen from rusty-interaction
pub fn verify_discord_message(
    public_key: &PublicKey,
    signature: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), SignatureValidationError> {
    let signature_bytes = hex::decode(signature)?;
    let signature = Signature::from_bytes(&signature_bytes)?;
    let msg = format!("{}{}", timestamp, body);

    public_key.verify(msg.as_bytes(), &signature)?;

    Ok(())
}
