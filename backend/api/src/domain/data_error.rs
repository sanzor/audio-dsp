use std::fmt;

#[derive(Debug)]
pub enum DataError {
    NotFound,
    Conflict(String),
    Validation(String),
    Internal(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::NotFound => write!(f, "record not found"),
            DataError::Conflict(msg) => write!(f, "conflict: {msg}"),
            DataError::Validation(msg) => write!(f, "validation: {msg}"),
            DataError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl From<sqlx::Error> for DataError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => DataError::NotFound,
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                DataError::Conflict(db_err.message().to_string())
            }
            _ => DataError::Internal(err.to_string()),
        }
    }
}
