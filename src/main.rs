#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod app;
// Only the harness changes this module; nothing reads the constant.
#[allow(dead_code)]
mod bench_touch;
mod constants;
mod error;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // The benchmark parsers read this marker as the start of app boot. The binary is the whole start command.
    println!("::bench::start {}", epoch_ms());
    println!("start conduit server...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        utils::db::run_migrations(&pool);
        use app::drivers::middlewares::state::AppState;
        AppState::new(pool)
    };

    let port = listen_port();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(app::drivers::middlewares::cors::cors())
            .wrap(app::drivers::middlewares::auth::Authentication)
            .configure(app::drivers::routes::api)
    })
    .bind((constants::BIND_HOST, port))?;
    println!("Server running on port {port}");
    // The socket is bound here, so the port accepts connections from this point.
    println!("::bench::listening {}", epoch_ms());
    server.run().await
}

fn listen_port() -> u16 {
    match std::env::var(constants::env_key::PORT) {
        Ok(port) => port.parse().expect("PORT must be a port number"),
        Err(_) => constants::DEFAULT_PORT,
    }
}

fn epoch_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is before 1970")
        .as_millis()
}
