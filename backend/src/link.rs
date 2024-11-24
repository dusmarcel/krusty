use serde::Serialize;

#[derive(Serialize)]
pub struct Link {
    rel: String,
    #[serde(rename="type")]
    link_type: String,
    href: String,
}

impl Link {
    pub fn new(host: &String, user: &String) -> Self {
        Self {
            rel: "self".to_string(),
            link_type: "application/activity+json.to".to_string(),
            href: format!("https://{}/user/{}", host, user),
        }
    }
}