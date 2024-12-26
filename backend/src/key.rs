use crate::user::User;

#[derive(Clone)]
pub struct Key {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

impl Key {
    pub fn new(host: &String, user: &User) -> Self {
        let id = format!("https://{}/{}#main-key", host, user.preferred_username);
        let owner = format!("https://{}/{}", host, user.preferred_username);
        let public_key_pem = user.public_key.clone();

        Self {
            id,
            owner,
            public_key_pem
        }
    }

    pub fn to_shared(&self) -> shared::key::Key {
        shared::key::Key {
            id: self.id.clone(),
            owner: self.owner.clone(),
            public_key_pem: self.public_key_pem.clone(),
        }
    }
}