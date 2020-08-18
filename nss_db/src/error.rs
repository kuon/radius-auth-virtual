use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not init OS")]
    OSInitFailed,
    #[error("TOML syntax error")]
    TomlError(#[from] toml::de::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Incompatible database schema")]
    IncompatibleDbVersion,
    #[error("Database error")]
    DatabaseError(#[from] sqlite::Error),
    #[error("CBOR serialize error")]
    CborSerializeError(#[from] serde_cbor::error::Error),
    #[error("Underlying IO error")]
    IOError(#[from] std::io::Error),
}
