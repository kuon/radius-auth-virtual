use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Attribute {
    pub vendor: u32,
    pub subtype: u8,
    #[serde(serialize_with = "as_base16")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize)]
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

fn as_base16<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: AsRef<[u8]>,
          S: Serializer
{
    serializer.serialize_str(&base16::encode_upper(key.as_ref()))
}
