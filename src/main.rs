#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;

mod api;
mod config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config();
    println!("config loaded: 'Ok'");
    let log_config = LogConfigBuilder::builder()
        .time_format(&config.logging.time_format)
        .output_console()
        .level(&config.logging.level)
        .unwrap()
        .build();
    simple_log::new(log_config).expect("failed to configure logger");
    info!("logger configured: 'Ok'");
    api::app::run(&config).await
}
