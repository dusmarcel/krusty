use anyhow::{Context, Result};
use openssl::rsa::Rsa;

#[derive(Clone)]
pub struct Key {
    pub id: String,
    pub owner: String,
    pub private_key_pem: String,
    pub public_key_pem: String,
}

impl Key {
    pub fn new(host: &Option<String>) -> Result<Self> {
        let id = format!("https://{}/actor#main-key", host.clone().context("No valid host given")?);
        let owner = format!("https://{}/actor", host.clone().context("No valid host given")?);
        let rsa = Rsa::generate(2048)?;
        let private_key_pem = match rsa.private_key_to_pem() {
            Ok(v) => String::from_utf8(v)?,
            Err(e) => format!("Couldn't read private key. The error message was: {}", e),
        };
        let public_key_pem = match rsa.public_key_to_pem() {
            Ok(v) => String::from_utf8(v)?,
            Err(e) => format!("Couldn't read public key. The error message was: {}", e),
        };

        Ok(Self {
            id,
            owner,
            private_key_pem,
            public_key_pem,
        })
    }

    pub fn to_shared(&self) -> shared::key::Key {
        shared::key::Key {
            id: self.id.clone(),
            owner: self.owner.clone(),
            public_key_pem: self.public_key_pem.clone(),
        }
    }
}