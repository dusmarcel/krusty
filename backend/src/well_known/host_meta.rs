use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use mime;

use crate::back::Backend;

#[get("/host-meta")]
async fn host_meta(backend: web::Data<Mutex<Backend>>) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    if let Some(host) = &my_backend.host {
        let mime = "application/xrd+xml; charset=utf-8".parse::<mime::Mime>().unwrap();
        HttpResponse::Ok()
            .content_type(mime)
            .body(
                format!(
r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}" />
</XRD>
"#,
                    host
                )
            )
    } else {
        HttpResponse::InternalServerError().finish()
    }
}