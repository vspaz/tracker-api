use crate::api::handlers::register_handlers;
use crate::config::Config;
use actix_web::HttpServer;


pub async fn start_server(config: &Config) -> std::io::Result<()> {
    let host_and_port = &format!(
            "{}:{}",
            config.http.server.host, config.http.server.port);
    info!("starting server at '{}'", host_and_port);
    HttpServer::new(register_handlers)
        .keep_alive(config.http.server.keep_alive)
        .bind(host_and_port)?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
