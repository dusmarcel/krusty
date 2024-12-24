use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub preferred_username: String,
    pub summary: Option<String>,
    pub public_key: String
}