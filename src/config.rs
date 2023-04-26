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
    pub request_timeout: i32,
    pub shutdown_timeout: i32,
}

pub struct Http {
    pub server: Server,
    pub worker: Worker,
    pub client: Client,
}

pub struct Config {
    pub http: Http,
    pub logging: Logging,
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
            worker: Worker {
                max_connections: 20_000,
                count: 6,
            },
            client: Client {
                request_timeout: 10000,
                shutdown_timeout: 5000,
            }
        },
        logging: Logging {
            level: "info".to_string(),
            time_format: "%Y-%m-%d %H:%M:%S.%f".to_string(),
        },
    }
}
