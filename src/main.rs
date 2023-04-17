mod api;
mod config;


use crate::api::app;
use actix_web::body::MessageBody;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config();
    app::run(&config).await
}
