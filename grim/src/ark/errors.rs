use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ArkReadError {
    #[error("Can't open ark file")]
    CantOpenArk,
    #[error("Parsing ark file not supported")]
    ArkNotSupported,
    #[error("HDR file is larger than 20mb")] // Honestly should never happen
    HdrTooBig,
}