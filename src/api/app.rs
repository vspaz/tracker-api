use crate::api::handlers::register_handlers;
use actix_web::HttpServer;
use crate::config::Config;

pub async fn start_server(config: &Config) -> std::io::Result<()> {
    HttpServer::new(register_handlers)
        .bind(format!("{}:{}", config.server, config.port))?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
