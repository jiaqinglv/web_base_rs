use serde::{Deserialize, Serialize};

/// DataBase Pool Config 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DBPoolConfig {
    // 池子最大大小
    pub max_size: u32,
    // 池子最小大小
    pub min_size: u32,
    pub idle_timeout: u64,
    // 连接字符串
    pub connect_str: String,
}