use std::time::Duration;

pub struct Server {
    pub host: String,
    pub port: i32,
    pub keep_alive: Duration,
    pub max_connections: i32,
}

pub struct Http {
    pub server: Server,
}

pub struct Config {
    pub http: Http,
}

pub fn get_config() -> Config {
    Config {
        http: Http {
            server: Server {
                host: "127.0.0.1".to_string(),
                port: 9000,
                keep_alive: Duration::from_secs(60 * 2),
                max_connections: 1000,
            },
        },
    }
}
