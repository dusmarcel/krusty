use serde::Serialize;

use crate::{actor::Actor, object::Object};

#[derive(Serialize)]
pub struct Activity {
    #[serde(rename="@context")]
    pub context: String,
    pub id: String,
    #[serde(rename="type")]
    pub activity_type: String,
    pub actor: Actor,
    #[serde(rename="preferredUsername")]
    pub object: Object
}