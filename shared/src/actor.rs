use serde::{Deserialize, Serialize};

use crate::key::Key;

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    #[serde(rename="@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename="type")]
    pub actor_type: String,
    #[serde(rename="preferredUsername")]
    pub preferred_username: String,
    pub inbox: String,
    #[serde(rename="publicKey")]
    pub key: Key
}
