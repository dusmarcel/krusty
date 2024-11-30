include!("../backend_config.rs");

use serde::Deserialize;
use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder, Result};

use backend::Backend;

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello from Krusty!")
}

#[get("/user/{user}")]
async fn user(backend: web::Data<Backend>, path: web::Path<String>) -> Result<impl Responder> {
    let user = path.into_inner();
    if let Some(b_user) = &backend.user {
        if *b_user == user {
            Ok(web::Json(backend.actor.to_shared()))
        } else {
            Err(error::ErrorNotFound("Not found!"))
        }
    } else {
        Err(error::ErrorInternalServerError("Internal server error!"))
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
                                                Ok(web::Json(backend::webfinger::Webfinger::new(host, b_user)))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = Backend::new().map_err(|e| {
        eprintln!("I cannot work under these conditions! Error while building backend: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("backend error: {}", e))        
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(backend.clone()))
            .service(back)
            .service(user)
            .service(webfinger)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
