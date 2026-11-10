use crate::constants::env_key;
use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn cors() -> Cors {
    // actix-cors rejects "*" as an allowed origin and the server then fails to start,
    // so an unset FRONTEND_ORIGIN allows any origin explicitly.
    let cors = match env::var(env_key::FRONTEND_ORIGIN) {
        Ok(frontend_origin) => Cors::default().allowed_origin(&frontend_origin),
        Err(_) => Cors::default().allow_any_origin(),
    };

    cors.allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
