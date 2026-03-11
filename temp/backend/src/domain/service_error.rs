use std::fmt;

use super::data_error::DataError;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Conflict(String),
    Validation(String),
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "not found"),
            ServiceError::Conflict(msg) => write!(f, "conflict: {msg}"),
            ServiceError::Validation(msg) => write!(f, "validation: {msg}"),
            ServiceError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::NotFound => ServiceError::NotFound,
            DataError::Conflict(msg) => ServiceError::Conflict(msg),
            DataError::Internal(msg) => ServiceError::Internal(msg),
        }
    }
}

impl From<String> for ServiceError {
    fn from(msg: String) -> Self {
        ServiceError::Internal(msg)
    }
}
