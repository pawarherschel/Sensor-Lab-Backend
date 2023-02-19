mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Data::new(config.clone()))
            .service(api::get)
            .service(api::post)
            .service(api::put)
            .service(api::delete)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
