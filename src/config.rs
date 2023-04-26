use std::time::Duration;

pub struct Worker {
    pub max_connections: usize,
    pub count: usize,
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
                host: "127.0.0.1".to_string(),
                port: 9000,
                keep_alive: Duration::from_secs(60 * 2),
            },
            client: Client {
                request_timeout: Duration::from_secs(10),
                shutdown_timeout: Duration::from_secs(10),
            },
        },
        worker: Worker {
            max_connections: 20_000,
            count: 6,
        },
        logging: Logging {
            level: "info".to_string(),
            time_format: "%Y-%m-%d %H:%M:%S.%f".to_string(),
        },
    }
}
