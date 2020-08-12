use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not init OS")]
    OSInitFailed,
    #[error("could not allocate memory")]
    Memory,
    #[error("no server provided")]
    NoServer,
    #[error("all servers timed out")]
    ServerTimeout,
    #[error("invalid server `{0}`")]
    InvalidServer(String),
    #[error("no shared secret provided")]
    NoSharedSecret,
    #[error("shared secret too long (max 256 chars)")]
    SharedSecretTooLong,
    #[error("underlying IO error")]
    IOError(#[from] std::io::Error),
    #[error("TOML syntax error")]
    TomlError(#[from] toml::de::Error),
    #[error("RADIUS client failure")]
    RadiusClient,
    #[error("authentication rejected, wrong credentials")]
    AuthReject,
    #[error("database error")]
    DatabaseError(#[from] sqlite::Error),
    #[error("CBOR serialize error")]
    CborSerializeError(#[from] serde_cbor::error::Error),
    #[error("incompatible database schema")]
    IncompatibleDbVersion,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid attribute format")]
    AttrFormat
}
