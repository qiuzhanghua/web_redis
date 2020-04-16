#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use r2d2::{Pool, PooledConnection};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(move || App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

pub async fn index(_request: HttpRequest) -> impl Responder {
    use r2d2_redis::redis::Commands;
    use std::collections::HashMap;
    let mut conn = get_redis_connection();
    let key = format!("{}:{}", "people", "834212ef-7022-459e-a281-16342addc1d0");
    let map = conn.hgetall::<&str, HashMap<String, String>>(&key);
    match map {
        Ok(m) => m.get("name").unwrap().clone(),
        _ => "Error".to_string(),
    }
}

// pub fn establish_redis_connection() -> PooledConnection<r2d2_redis::RedisConnectionManager> {
//     dotenv::dotenv().ok();
//     let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
//     let manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
//     let pool = r2d2::Pool::builder()
//         .max_size(REDIS_POOL_SIZE)
//         .build(manager)
//         .unwrap();
//     pool.get().unwrap()
// }

pub const REDIS_POOL_SIZE: u32 = 64;

lazy_static! {
    pub static ref REDIS_POOL: Pool<r2d2_redis::RedisConnectionManager> = {
        dotenv::dotenv().ok();
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
        r2d2::Pool::builder()
            .max_size(REDIS_POOL_SIZE)
            .build(manager)
            .expect("Failed to create redis pool.")
    };

    // Used to update core data into redis master, such as person, role and dept etc.
    // pub static ref MASTER_REDIS_POOL: Pool<r2d2_redis::RedisConnectionManager> = {
    //     dotenv().ok();
    //     let redis_url = env::var("MASTER_REDIS_URL").expect("MASTER_REDIS_URL must be set");
    //     let manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    //     r2d2::Pool::builder()
    //         .max_size(REDIS_POOL_SIZE)
    //         .build(manager)
    //         .expect("Failed to create master redis pool.")
    // };

}

pub fn get_redis_connection() -> PooledConnection<r2d2_redis::RedisConnectionManager> {
    REDIS_POOL.clone().get().unwrap()
}
