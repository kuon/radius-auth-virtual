use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub radius: radius::User,
    pub mapping: crate::config::UserMapping,
}

