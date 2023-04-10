use crate::api::handlers::register_handlers;
use actix_web::HttpServer;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(register_handlers)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

pub async fn run() -> std::io::Result<()> {
    start_server().await
}
