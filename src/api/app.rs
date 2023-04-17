use std::time::Duration;
use crate::api::handlers::register_handlers;
use crate::config::Config;
use actix_web::HttpServer;

pub async fn start_server(config: &Config) -> std::io::Result<()> {
    HttpServer::new(register_handlers).keep_alive(Duration::from_secs(60 * 2))
        .bind(format!("{}:{}", config.http.server.host, config.http.server.port))?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
