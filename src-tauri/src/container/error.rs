use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContainerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP parsing error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON deserialization error in {path}: {source}")]
    Json {
        path: String,
        source: serde_json::Error,
    },

    #[error("First entry MUST be 'mimetype' with stored compression")]
    InvalidMimetypeEntry,

    #[error("Invalid mimetype content: expected '{expected}', found '{found}'")]
    MimetypeMismatch { expected: String, found: String },

    #[error("Prohibited path traversal or malformed path: {0}")]
    UnsafePath(String),

    #[error("Missing required file: {0}")]
    MissingRequiredFile(String),

    #[error("Integrity check failed for {path}: expected digest {expected}, computed {computed}")]
    DigestMismatch {
        path: String,
        expected: String,
        computed: String,
    },
}
