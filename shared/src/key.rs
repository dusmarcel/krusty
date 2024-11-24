use serde::Serialize;

#[derive(Serialize)]
pub struct Key {
    pub id: String,
    pub owner: String,
    #[serde(rename="publicKeyPem")]
    pub public_key_pem: String,
}