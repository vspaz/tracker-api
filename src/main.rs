mod api;
mod config;

use actix_web::body::MessageBody;
use crate::api::app;
use crate::config::config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let config = Config{
        server: "127.0.0.1".to_string(),
        port: "9000".to_string(),
    };

    app::run(&config).await
}
