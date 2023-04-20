#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;

mod api;
mod config;
use crate::api::app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let log_config = LogConfigBuilder::builder()
        .time_format("%Y-%m-%d %H:%M:%S.%f")
        .level("info")
        .output_console()
        .build();
    simple_log::new(log_config).expect("failed to configure logger");
    let config = config::get_config();
    app::run(&config).await
}
