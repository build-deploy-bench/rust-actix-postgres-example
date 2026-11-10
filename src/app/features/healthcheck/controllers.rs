use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// The commit that build.rs embedded at build time, and the run ID that the harness sets before each
// deploy (each deployment keeps the env it started with). It does not touch the database.
pub async fn version() -> impl Responder {
    let run = std::env::var("BENCH_RUN_ID")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string());
    // Railway gets no BENCH_RUN_ID: a changed variable invalidates part of its build cache. Its own
    // deployment ID identifies the deployment instead.
    let deployment = std::env::var("RAILWAY_DEPLOYMENT_ID")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string());
    HttpResponse::Ok().json(json!({ "commit": env!("CONDUIT_COMMIT"), "run": run, "deployment": deployment }))
}
