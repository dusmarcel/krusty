use std::sync::Mutex;

use actix_web::{get, http::header::{ContentDisposition, ContentType}, web, HttpResponse, Responder};

use crate::Backend;

#[get("/host-meta")]
async fn host_meta(backend: web::Data<Mutex<Backend>>) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    if let Some(host) = &my_backend.host {
        HttpResponse::Ok()
            .content_type(ContentType::xml())
            .insert_header(ContentDisposition::attachment("host-meta"))
            .body(
                format!(
                    r#"<?xml version="1.0"?>
                    <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
                        <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource=acct:{{uri}}" type="application/json"/>
                    </XRD>"#,
                    host
                )
            )
    } else {
        HttpResponse::InternalServerError().finish()
    }
}