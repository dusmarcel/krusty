use serde::{Deserialize, Serialize};

use crate::{actor::Actor, object::Object};

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    #[serde(rename="@context")]
    pub context: String,
    pub id: String,
    #[serde(rename="type")]
    pub activity_type: String,
    pub actor: Actor,
    pub object: Object
}