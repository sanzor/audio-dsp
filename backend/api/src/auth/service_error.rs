use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Conflict(String),
    Forbidden,
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "not found"),
            ServiceError::Conflict(msg) => write!(f, "conflict: {msg}"),
            ServiceError::Forbidden => write!(f, "forbidden"),
            ServiceError::Internal(msg) => write!(f, "internal: {msg}"),
        }
    }
}

impl From<String> for ServiceError {
    fn from(s: String) -> Self { ServiceError::Internal(s) }
}

impl From<crate::domain::service_error::ServiceError> for ServiceError {
    fn from(err: crate::domain::service_error::ServiceError) -> Self {
        match err {
            crate::domain::service_error::ServiceError::NotFound => ServiceError::NotFound,
            crate::domain::service_error::ServiceError::Conflict(msg) => ServiceError::Conflict(msg),
            crate::domain::service_error::ServiceError::Validation(msg) => ServiceError::Internal(msg),
            crate::domain::service_error::ServiceError::Internal(msg) => ServiceError::Internal(msg),
        }
    }
}
