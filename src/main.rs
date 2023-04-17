mod api;
mod config;

use crate::api::app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config();
    app::run(&config).await
}
