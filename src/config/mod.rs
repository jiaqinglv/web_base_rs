use serde::{Serialize, Deserialize};

use self::db::DBPoolConfig;

pub mod db;

#[derive(Debug,Serialize,Deserialize, Clone)]
/// http 服务器配置文件
pub struct ServerConfig {
    pub host: String,
    /// 端口
    pub port: i64,

    pub pg: DBPoolConfig, 

    pub redis: DBPoolConfig,
}

impl ServerConfig {
    pub fn default() -> ServerConfig {
        return ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            pg: DBPoolConfig{
                max_size: 30,
                min_size: 10,
                idle_timeout: 100,
                connect_str: "postgresql://postgres:admin@localhost:5432/book".to_string(),
            },
            redis: DBPoolConfig{
                max_size: 30,
                min_size: 10,
                idle_timeout: 100,
                connect_str: "redis://default:admin@localhost:6379/db0".to_string(),
            }
        }
    }
}