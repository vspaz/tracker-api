mod api;
use crate::api::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}
