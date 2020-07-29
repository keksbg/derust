use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DerustError {
    UnknownError,
    HttpError,
    NoPermissions,
    InvalidToken,
}
