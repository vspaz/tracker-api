mod api;

use actix_web::HttpServer;
use crate::api::handlers::{register_handlers, start_server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
