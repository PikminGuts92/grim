use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ArkReadError {
    #[error("Can't open ark file")]
    CantOpenArk
}