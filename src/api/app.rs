use crate::api::handlers::register_handlers;
use crate::config::Config;
use actix_web::HttpServer;

pub async fn start_server(config: &Config) -> std::io::Result<()> {
    HttpServer::new(register_handlers).keep_alive(config.http.server.keep_alive)
        .bind(format!("{}:{}", config.http.server.host, config.http.server.port))?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
