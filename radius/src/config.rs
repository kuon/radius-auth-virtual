use serde::{Deserialize};
use std::path::PathBuf;
use crate::error::Error;


use common::serde::decode_attrs;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub address: String,
    pub shared_secret: Option<String>,
    pub timeout: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub shared_secret: Option<String>,
    pub servers: Vec<Server>,
    pub debug: Option<bool>,
    pub timeout: Option<u16>,
    #[serde(deserialize_with = "decode_attrs")]
    pub attributes: Option<Vec<(u32, u8)>>,
}

impl Config {

    pub fn read_file<S: Into<PathBuf>>(path: S) -> Result<Config, Error> {
        let config = std::fs::read_to_string(path.into())?;
        let config = toml::from_str::<toml::Value>(&config)?;
        let config = config.get("radius").ok_or(Error::ConfigFormat)?;
        let config = config.clone().try_into::<Config>()?;
        Ok(config)
    }
}

