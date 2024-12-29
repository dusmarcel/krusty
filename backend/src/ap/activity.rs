use uuid::Uuid;

use crate::{
    user::User,
    ap::object::Object
};

pub struct Activity {
    pub context: String,
    pub id: String,
    pub activity_type: String,
    pub actor: String,
    pub object: Object
}

impl Activity {
    pub fn new(host: &String, user: &User, in_reply_to: &Option<String>, post: &String) -> Self {
        let context = "https://www.w3.org/ns/activitystreams".to_string();
        let id = format!("https://{}/activity/{}", host, Uuid::now_v7());
        let activity_type = "Create".to_string();
        let actor = format!("https://{}/user/{}", host, user.preferred_username);
        let object = Object::new(host, user, in_reply_to, post);

        Self {
            context,
            id,
            activity_type,
            actor,
            object
        }
    }

    pub fn to_shared(&self) -> shared::activity::Activity {
        shared::activity::Activity {
            context: self.context.clone(),
            id: self.id.clone(),
            activity_type: self.activity_type.clone(),
            actor: self.actor.clone(),
            object: self.object.to_shared()
        }
    }
}