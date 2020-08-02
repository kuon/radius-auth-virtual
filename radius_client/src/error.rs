use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not init OS")]
    OSInitFailed,
    #[error("could not allocate memory")]
    Memory,
    #[error("no server provided")]
    NoServer,
    #[error("invalid server `{0}`")]
    InvalidServer(String),
    #[error("no shared secret provided")]
    NoSharedSecret,
    #[error("shared secret too long (max 256 chars)")]
    SharedSecretTooLong,
    #[error("underlying IO error")]
    IOError(#[from] std::io::Error),
    #[error("failed to initialize RADIUS client")]
    RadiusClient,
}
