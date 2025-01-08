use std::env;
use std::time::Duration;

pub struct Worker {
    pub count: usize,
    pub threads: usize,
    pub max_connections: usize,
}

pub struct Server {
    pub host: String,
    pub port: i32,
    pub keep_alive: Duration,
}

pub struct Client {
    pub request_timeout: Duration,
    pub shutdown_timeout: Duration,
}

pub struct Http {
    pub server: Server,
    pub client: Client,
}

pub struct Config {
    pub http: Http,
    pub logging: Logging,
    pub worker: Worker,
}

pub struct Logging {
    pub level: String,
    pub time_format: String,
}

pub fn get_config() -> Config {
    Config {
        http: Http {
            server: Server {
                host: "0.0.0.0".to_string(),
                port: 9000,
                keep_alive: Duration::from_secs(60 * 2),
            },
            client: Client {
                request_timeout: Duration::from_secs(10),
                shutdown_timeout: Duration::from_secs(10),
            },
        },
        worker: Worker {
            count: 6,
            threads: 8,
            max_connections: 20_000,
        },
        logging: Logging {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            time_format: env::var("LOG_FORMAT").unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S.%f".to_string()),
        },
    }
}
