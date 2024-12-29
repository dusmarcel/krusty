use uuid::Uuid;

use crate::user::User;

#[derive(Debug)]
pub struct Object {
    pub id: String,
    pub object_type: String,
    pub attributed_to: String,
    pub in_reply_to: Option<String>,
    pub content: String,
    pub to: Vec<String>
}

impl Object {
    pub fn new(host: &String, user: &User, in_reply_to: &Option<String>, content: &String) -> Self {
        let id = format!("https://{}/object/{}", host, Uuid::now_v7());
        let object_type = "Note".to_string();
        let attributed_to = format!("https://{}/user/{}", host, user.preferred_username);
        let in_reply_to = in_reply_to.clone();
        let content = content.clone();
        let to = vec!["https://www.w3.org/ns/activitystreams#Public".to_string()];

        Self {
            id,
            object_type,
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
            attributed_to: self.attributed_to.clone(),
            in_reply_to: self.in_reply_to.clone(),
            content: self.content.clone(),
            to: self.to.clone()
        }
    }
}