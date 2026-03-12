use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Conflict(String),
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "not found"),
            ServiceError::Conflict(msg) => write!(f, "conflict: {msg}"),
            ServiceError::Internal(msg) => write!(f, "internal: {msg}"),
        }
    }
}

impl From<String> for ServiceError {
    fn from(s: String) -> Self { ServiceError::Internal(s) }
}
