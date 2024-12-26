use std::sync::Mutex;

use serde::{Serialize, Deserialize};
use actix_web::{error, get, web, Responder, Result};

use crate::{
    Backend,
    user::User,
    link::Link
};

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

#[get("/webfinger")]
async fn webfinger(backend: web::Data<Mutex<Backend>>, query: web::Query<Resource>) -> Result<impl Responder> {
    let my_backend = backend.lock().unwrap();
    let resource = query.into_inner().resource;
    if let Some(host) =  &my_backend.host {
        let resource_parts: Vec<&str> = resource.split(':').collect();
        if resource_parts.len() == 2 {
            if let Some(first_part) = resource_parts.get(0) {
                if *first_part == "acct" {
                    if let Some(second_part) = resource_parts.get(1) {
                        let acct_parts: Vec<&str> = second_part.split('@').collect();
                        if acct_parts.len() == 2 {
                            if let Some(first_part) = acct_parts.get(0) {
                                let result = sqlx::query_as::<_, User>(
                                        "SELECT * FROM users WHERE preferred_username = $1"
                                    )
                                    .bind(*first_part)
                                    .fetch_optional(&my_backend.pool)
                                    .await;

                                match result {
                                    Ok(res) => {
                                        if let Some(user) = res { 
                                            if let Some(second_part) = acct_parts.get(1) {
                                                if *second_part == host {
                                                    Ok(web::Json(crate::webfinger::Webfinger::new(host, &user.preferred_username)))
                                                } else {
                                                    Err(error::ErrorNotFound("Not found!"))
                                                }
                                            } else {
                                                Err(error::ErrorNotFound("Not found!"))
                                            }
                                        } else {
                                            Err(error::ErrorNotFound("Not found!"))
                                        } 
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        Err(error::ErrorInternalServerError("Internal server error!"))
                                    }
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
}
