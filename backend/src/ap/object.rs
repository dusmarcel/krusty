use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::user::User;

#[derive(Debug)]
pub struct Object {
    pub id: String,
    pub object_type: String,
    pub published: DateTime<Utc>,
    pub attributed_to: String,
    pub in_reply_to: String,
    pub content: String,
    pub to: String
}

impl Object {
    pub fn new(host: &String, user: &User, in_reply_to: &String, content: &String) -> Self {
        let id = format!("https://{}/object/{}", host, Uuid::now_v7());
        let object_type = "Note".to_string();
        let published = chrono::Utc::now();
        let attributed_to = format!("https://{}/user/{}", host, user.preferred_username);
        let in_reply_to = in_reply_to.clone();
        let content = content.clone();
        let to = "https://www.w3.org/ns/activitystreams#Public".to_string();

        Self {
            id,
            object_type,
            published,
            attributed_to,
            in_reply_to,
            content,
            to
        }
    }

    pub fn to_shared(&self) -> shared::object::Object {
        shared::object::Object {
            id: self.id.clone(),
            object_type: self.object_type.clone(),
            published: self.published.clone(),
            attributed_to: self.attributed_to.clone(),
            in_reply_to: self.in_reply_to.clone(),
            content: self.content.clone(),
            to: self.to.clone()
        }
    }
}