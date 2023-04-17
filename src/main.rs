mod api;
mod config;
use crate::api::app;
use crate::config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
