use serde::{Serialize, Deserialize};
use actix_web::{error, get, web, Responder, Result};

use crate::Backend;
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

#[derive(Deserialize)]
pub struct Resource {
    pub resource: String,
}

#[get("/.well-known/webfinger")]
async fn webfinger(backend: web::Data<Backend>, query: web::Query<Resource>) -> Result<impl Responder> {
    let resource = query.into_inner().resource;
    if let Some(host) =  &backend.host {
        if let Some(b_user) = &backend.user {
            let resource_parts: Vec<&str> = resource.split(':').collect();
            if resource_parts.len() == 2 {
                if let Some(first_part) = resource_parts.get(0) {
                    if *first_part == "acct" {
                        if let Some(second_part) = resource_parts.get(1) {
                            let acct_parts: Vec<&str> = second_part.split('@').collect();
                            if acct_parts.len() == 2 {
                                if let Some(first_part) = acct_parts.get(0) {
                                    if *first_part == b_user {
                                        if let Some(second_part) = acct_parts.get(1) {
                                            if *second_part == host {
                                                Ok(web::Json(crate::webfinger::Webfinger::new(host, b_user)))
                                            } else {
                                                Err(error::ErrorNotFound("Not found!"))
                                            }
                                        } else {
                                            Err(error::ErrorNotFound("Not found!"))
                                        }
                                    } else {
                                        Err(error::ErrorNotFound("Not found!"))
                                    }
                                } else {
                                    Err(error::ErrorNotFound("Not found!"))
                                }
                            } else {
                                Err(error::ErrorNotFound("Not found!"))
                            }
                        } else {
                            Err(error::ErrorNotFound("Not found!"))
                        }
                    } else {
                        Err(error::ErrorInternalServerError("Internal server error!"))
                    }
                } else {
                    Err(error::ErrorInternalServerError("Internal server error!"))
                }
            } else {
                Err(error::ErrorInternalServerError("Internal server error!"))
            }
        } else {
            Err(error::ErrorInternalServerError("Internal server error!"))
        }
    } else {
        Err(error::ErrorInternalServerError("Internal server error!"))
    }
}
