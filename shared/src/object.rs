use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: String,
    #[serde(rename="type")]
    pub object_type: String,
    pub published: String,
    pub attributed_to: String,
    #[serde(rename="inReplyTo")]
    pub in_reply_to: Option<String>,
    pub content: String,
    pub to: Vec<String>
}