use std::{any::Any, error::Error, net::SocketAddr, collections::HashMap};
use serde::{Deserialize};
use tokio::fs;
use tracing::{info, Level, error};
use tracing_subscriber::util::SubscriberInitExt;

use crate::{routers, config::ServerConfig};

use super::db::{DBPool, self};

/// 服务器应用
pub struct App {
    pub server: Box<dyn Any>,
    pub conf: HashMap<String, ServerConfig>,
    pub data: Option<DBPool>, 
}

/// 配置文件类型
#[allow(dead_code)]
#[derive(Deserialize, Debug,  Clone)]
pub enum ConfigType {
    JSON(String),
    TOML(String),
}

/// 配置
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    source: ConfigType,
    data: ServerConfig,
}

/// 配置信息
impl Config{
    pub async fn load_default_config(mut self,app: &mut App) -> Result<(), Box<dyn Error>> {
        let conifg_path = match self.source {
            ConfigType::JSON(path) => path,
            ConfigType::TOML(path) => path,
        };
        let config_file = fs::read_to_string(conifg_path.as_str()).await;
        let config_file_data = match config_file {
            Ok(data) => data,
            Err(err) => {
                error!("Couldn't read config file, {}", err);
                std::process::exit(2);
            }
        };
        let config= serde_json::from_str(config_file_data.as_str());
        self.data = match config {
            Ok(config) => config,
            Err(err) => {
                error!("configer load error:{}", err);
                ServerConfig::default()
            },
        };
        app.conf.insert("default".to_string(), self.data);
        Ok(())
    }
}

/// 应用信息
impl App {
    /// 初始化配置
    pub async fn init_config(&mut self, conf_type: ConfigType) -> Result<(), Box<dyn Error>> {
        let config = Config{
            source: conf_type,
            data: ServerConfig::default()
        };
        config.load_default_config(self,).await?;
        Ok(())
    }

    /// 创建axum http服务器
    pub async fn new_axum_http_server(mut self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        let router = routers::new_http();
        info!("hettp server is running on http://{}", addr);
        let server = axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await?;
        self.server = Box::new(server);
        Result::Ok(())
    }

    /// 日志初始化
    pub fn init_log(&self, level: Level){
        tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // builds the subscriber.
        .finish().init();
    }

}

/// 初始化axum http 服务器 并初始化配置、日志以及数据库信息
pub async fn new_axum_server(default_config_path: &str) -> Result<(), Box<dyn Error>>{
    // 初始化APP
    let mut app = App{
        server: Box::new(()),
        conf: HashMap::new(),
        data: None,
    };

    // 日志初始化
    app.init_log(Level::DEBUG);

    // 默认初始配置
    let default_conf = &ServerConfig::default();
    // 配置文件初始化
    let config_type = ConfigType::JSON(default_config_path.to_string());
    app.init_config(config_type).await?;

    let conf = app.conf.get(&"default".to_string()).unwrap_or(default_conf);
    let port = conf.port;
    let host = conf.host.as_str();
    //配置信息读取
    let addr:SocketAddr = (host.to_string()+ ":" + &port.to_string()).parse()?;

    // 数据库设置
    app.data = Some(db::new_dbpool(conf).await?);

    // 初始化 axum http服务器
    app.new_axum_http_server(addr).await?;

    Ok(())
}
