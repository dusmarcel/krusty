use crate::{ap::key::Key, user::User};

#[derive(Clone)]
pub struct Actor {
    context: Vec<String>,
    id: String,
    actor_type: String,
    preferred_username: String,
    inbox: String,
    key: Key
}

impl Actor {
    pub fn new(host: &String, user: &User) -> Self {
        let context = vec![
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string()
        ];
        let id = format!("https://{}/user/{}",
            host,
            user.preferred_username);
        let actor_type = "Person".to_string();
        let preferred_username = user.preferred_username.clone();
        let inbox = format!("https://{}/inbox", host);
        let key = Key::new(&host, &user);

        Self {
            context,
            id,
            actor_type,
            preferred_username,
            inbox,
            key
        }
    }

    pub fn to_shared(&self) -> shared::actor::Actor {
        shared::actor::Actor {
            context: self.context.clone(),
            id: self.id.clone(),
            actor_type: self.actor_type.clone(),
            preferred_username: self.preferred_username.clone(),
            inbox: self.inbox.clone(),
            key: self.key.to_shared(),
        }
    }
}