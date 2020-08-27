use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not init OS")]
    OSInitFailed,
    #[error("Could not allocate memory")]
    Memory,
    #[error("No server provided")]
    NoServer,
    #[error("All servers timed out")]
    ServerTimeout,
    #[error("Invalid server `{0}`")]
    InvalidServer(String),
    #[error("No shared secret provided")]
    NoSharedSecret,
    #[error("Shared secret too long (max 256 chars)")]
    SharedSecretTooLong,
    #[error("Underlying IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("RADIUS client failure")]
    RadiusClient,
    #[error("Authentication rejected, wrong credentials")]
    AuthReject,
    #[error("TOML syntax error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Config format error")]
    ConfigFormat,
}
