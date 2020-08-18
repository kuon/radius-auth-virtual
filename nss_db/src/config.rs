use crate::error::Error;
use crate::user::User;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use common::serde::{decode_attr, encode_attr};

const CONFIG_PATH: &str = "/etc/radius_auth_virtual.toml";

#[derive(Deserialize, Debug)]
pub struct Db {
    pub path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserMapping {
    pub username: String,
    pub uid: u32,
    pub group: String,
    pub gid: u32,
    pub home: String,
    pub shell: String,
    #[serde(serialize_with = "encode_attr", deserialize_with = "decode_attr")]
    pub attribute: (u32, u8),
    pub attribute_value: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: Db,
    pub radius: radius::Config,
    pub users: Vec<UserMapping>,
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

    pub fn map_user(&self, radius: &radius::User) -> Option<User> {
        for user in self.users.iter() {
            for attr in radius.attributes.iter() {
                if attr.vendor == user.attribute.0
                    && attr.subtype == user.attribute.1
                    && attr.data == user.attribute_value
                {
                    return Some(User {
                        radius: radius.clone(),
                        mapping: user.clone(),
                    });
                }
            }
        }

        None
    }
}
