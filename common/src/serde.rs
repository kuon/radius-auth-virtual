use serde::{Deserializer, Serializer};

pub fn encode_base16<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base16::encode_upper(key.as_ref()))
}

pub fn decode_base16<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    base16::decode(s).map_err(|err| Error::custom(err.to_string()))
}

pub fn decode_attrs<'de, D: Deserializer<'de>>(
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

pub fn decode_attr<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<(u32, u8), D::Error> {
    let str_attr: String = serde::de::Deserialize::deserialize(deserializer)?;

    use serde::de::Error;
    parse_attr(&str_attr).map_err(|err| Error::custom(err.to_string()))
}

fn parse_attr(attr: &String) -> Result<(u32, u8), String> {
    let parts: Vec<&str> = attr.split('.').collect();
    let err = Err("invalid attribute format".to_string());
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

pub fn encode_attr<S>(key: &(u32, u8), serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}.{}", key.0, key.1);
    serializer.serialize_str(&s)
}
