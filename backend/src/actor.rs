use anyhow::{Context, Result};

use crate::key::Key;

#[derive(Clone)]
pub struct Actor {
    context: Vec<String>,
    id: String,
    actor_type: String,
    preferred_username: String,
    inbox: String,
    key: Key,
}

impl Actor {
    pub fn new(host: &Option<String>, user: &Option<String>) -> Result<Self> {
        let context = vec![
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string()
        ];
        let id = format!("https://{}/user/{}",
            host.clone().context("No host given")?,
            user.clone().context("No host given")?);
        let actor_type = "Person".to_string();
        let preferred_username = user.clone().context("No user given")?;
        let inbox = format!("https://{}/inbox", host.clone().context("No host given")?);
        let key = Key::new(&host)?;

        Ok(Self {
            context,
            id,
            actor_type,
            preferred_username,
            inbox,
            key,
        })
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