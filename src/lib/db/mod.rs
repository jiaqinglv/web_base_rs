use std::time::Duration;
use std::error::Error;

use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres::Postgres};
use deadpool_redis::{self, Timeouts};

use crate::config::ServerConfig;

/// 数据库连接池
#[allow(dead_code)]
pub struct DBPool {
    pg: Pool<Postgres>,
    redis: deadpool_redis::Pool
}

/// 初始化数据库池
pub async fn new_dbpool(conf: &ServerConfig) -> Result<DBPool, Box<dyn Error>> {
    let pg_res= PgPoolOptions::new()
        .max_connections(conf.pg.max_size)
        .min_connections(conf.pg.min_size)
        .idle_timeout(Duration::new(conf.pg.idle_timeout, 0))
        .connect(&conf.pg.connect_str).await;
    
    let pg = match pg_res {
        Ok(pool) => pool,
        Err(err) => panic!("Postgresql Connect Error: {}", err),
    };

    let mut redis_conf = deadpool_redis::Config::from_url(&conf.redis.connect_str);
    redis_conf.pool = Some(deadpool_redis::PoolConfig{
        max_size: conf.redis.max_size.to_string().parse::<usize>().unwrap_or(30),
        timeouts: Timeouts{
            // 等待插槽可用时超时
            wait: Some(Duration::from_secs(conf.redis.idle_timeout)),
            // 创建新对象时超时
            create: Some(Duration::from_secs(conf.redis.idle_timeout)),
            // 回收对象时超时
            recycle: Some(Duration::from_secs(conf.redis.idle_timeout)),
        },
    });
    let redis = match redis_conf.create_pool(Some(deadpool_redis::Runtime::Tokio1)) {
        Ok(pool) => pool,
        Err(error) => panic!("Redis Connect Error: {}", error),
    };
    
    Ok(DBPool{
        pg,
        redis,
    })
}

