use crate::api::handlers::register_handlers;
use crate::config::Config;
use actix_web::HttpServer;

pub async fn start_server(config: &Config) -> std::io::Result<()> {
    let host_and_port = format!("{}:{}", config.http.server.host, config.http.server.port);
    let app = HttpServer::new(register_handlers);
    info!("handlers configured: 'Ok'");
    info!("starting server at '{}'", &host_and_port);
    app.workers(config.worker.count)
        .worker_max_blocking_threads(config.worker.threads)
        .max_connections(config.worker.max_connections)
        .client_request_timeout(config.http.client.request_timeout)
        .client_disconnect_timeout(config.http.client.shutdown_timeout)
        .keep_alive(config.http.server.keep_alive)
        .bind(host_and_port)?
        .run()
        .await
}

pub async fn run(config: &Config) -> std::io::Result<()> {
    start_server(config).await
}
