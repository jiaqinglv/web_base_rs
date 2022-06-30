use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize, Clone)]
/// http 服务器配置文件
pub struct ServerConfig {
    pub host: String,
    /// 端口
    pub port: i64,
}

impl ServerConfig {
    pub fn default() -> ServerConfig {
        return ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000
        }
    }
}