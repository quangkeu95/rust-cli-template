use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct HttpServer {
    pub port: u16,
    pub log_level: String,
}
