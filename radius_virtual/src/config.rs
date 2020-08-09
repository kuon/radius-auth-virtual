use crate::error::Error;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

const CONFIG_PATH: &str = "/etc/radius_auth_virtual.toml";

#[derive(Deserialize, Debug)]
pub struct Server {
    pub(crate) address: String,
    pub(crate) shared_secret: Option<String>,
    pub(crate) timeout: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct Db {
    pub(crate) path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Radius {
    pub(crate) shared_secret: Option<String>,
    pub(crate) servers: Vec<Server>,
    pub(crate) debug: Option<bool>,
    pub(crate) timeout: Option<u16>,
    #[serde(deserialize_with = "decode_attrs")]
    pub(crate) attributes: Option<Vec<(u32, u8)>>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub(crate) username: String,
    pub(crate) uid: u32,
    pub(crate) group: String,
    pub(crate) gid: u32,
    pub(crate) attribute: String,
    pub(crate) attribute_value: u32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) db: Db,
    pub(crate) radius: Radius,
    pub(crate) users: Vec<User>,
}

impl Config {
    pub fn system() -> Result<Config, Error> {
        Config::read_file(CONFIG_PATH)
    }

    pub fn system_path() -> &'static str {
        CONFIG_PATH
    }

    pub fn read_file<S: Into<PathBuf>>(path: S) -> Result<Config, Error> {
        let config = std::fs::read_to_string(path.into())?;
        let config = toml::from_str::<Config>(&config)?;
        Ok(config)
    }
}

fn decode_attrs<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<(u32, u8)>>, D::Error> {
    let str_attrs: Vec<String> =
        serde::de::Deserialize::deserialize(deserializer)?;

    let mut attrs: Vec<(u32, u8)> = vec![];

    for attr in str_attrs.iter() {
        if let Ok(attr) = parse_attr(attr) {
            attrs.push(attr);
        }
    }
    Ok(Some(attrs))
}

fn parse_attr(attr: &String) -> Result<(u32, u8), &str> {
    let parts: Vec<&str> = attr.split('.').collect();
    let err = Err("invalid attribute format");
    if parts.len() != 2 {
        return err;
    }
    let vendor = parts.get(0).unwrap();
    let subtype = parts.get(1).unwrap();

    if let (Ok(vendor), Ok(subtype)) = (vendor.parse(), subtype.parse()) {
        Ok((vendor, subtype))
    } else {
        err
    }
}
