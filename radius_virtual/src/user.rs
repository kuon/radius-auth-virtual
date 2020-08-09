use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub vendor: u32,
    pub subtype: u8,
    #[serde(serialize_with = "as_base16", deserialize_with = "from_base16")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
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
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base16::encode_upper(key.as_ref()))
}

fn from_base16<'de, D>(deserializer:  D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    base16::decode(s).map_err(|err| Error::custom(err.to_string()))
}


#[cfg(test)]
mod tests {
    #[test]
    fn serialize() {
        assert_eq!(2 + 2, 4);
    }
}
