use std::fmt;

pub enum ProcessorError {
    CompileError(String),
    MetadataError(String),
    WriteError(String),
    DataError(String),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorError::CompileError(msg) => write!(f, "compile error: {msg}"),
            ProcessorError::MetadataError(msg) => write!(f, "metadata error: {msg}"),
            ProcessorError::WriteError(msg) => write!(f, "storage write error: {msg}"),
            ProcessorError::DataError(msg) => write!(f, "data error: {msg}"),
        }
    }
}
