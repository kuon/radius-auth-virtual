use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not init OS")]
    OSInitFailed,
    #[error("TOML syntax error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Incompatible database schema")]
    IncompatibleDbVersion,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),
    #[error("CBOR serialize error: {0}")]
    CborSerializeError(#[from] serde_cbor::error::Error),
    #[error("Underlying IO error: {0}")]
    IOError(#[from] std::io::Error),
}
