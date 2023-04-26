use crate::api::handlers::register_handlers;
use crate::config::Config;
use actix_web::HttpServer;

pub async fn start_server(config: &Config) -> std::io::Result<()> {
    let host_and_port = format!("{}:{}", config.http.server.host, config.http.server.port);
    let app = HttpServer::new(register_handlers);
    info!("handlers configured: 'ok'");
    info!("starting server at '{}'", &host_and_port);
    app.keep_alive(config.http.server.keep_alive)
        .workers(config.http.worker.count)
        .max_connections(config.http.worker.max_connections)
        .bind(host_and_port)?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
