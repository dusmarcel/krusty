use serde::Serialize;

use crate::link::Link;

#[derive(Serialize)]
pub struct Webfinger {
    subject: String,
    links: Vec<Link>,
}

impl Webfinger {
    pub fn new(host: &String, user: &String) -> Self {
        Self {
            subject: format!("acct:{}@{}", user, host),
            links: vec![
                Link::new(host, user),
            ]
        }
    }
}