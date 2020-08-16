use serde::{Deserialize, Serialize};

use common::serde::{decode_base16, encode_base16};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub vendor: u32,
    pub subtype: u8,
    #[serde(
        serialize_with = "encode_base16",
        deserialize_with = "decode_base16"
    )]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub attributes: Vec<Attribute>,
}

impl User {
    pub fn new<S: Into<String>>(username: S) -> Self {
        User {
            attributes: vec![],
            username: username.into(),
        }
    }
}
