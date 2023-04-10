mod api;
use crate::api::app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
