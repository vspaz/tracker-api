pub struct Config {
    pub server: String,
    pub port: String,
}

pub fn get_config() -> Config {
    Config {
        server: "127.0.0.1".to_string(),
        port: "9000".to_string(),
    }
}
