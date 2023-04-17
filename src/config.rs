struct Server {
    pub host: String,
    pub port: int,
}

struct Http {
    pub server: Server
}


pub struct Config {
    pub http: Http
}

pub fn get_config() -> Config {
    Config {
        http: Http {
            server: Server {
                host: "127.0.0.1".to_string(),
                port: 9000 },
        },
    }
}
