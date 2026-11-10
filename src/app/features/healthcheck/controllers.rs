use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// The commit that build.rs embedded at build time. It does not touch the database.
pub async fn version() -> impl Responder {
    HttpResponse::Ok().json(json!({ "commit": env!("CONDUIT_COMMIT") }))
}
