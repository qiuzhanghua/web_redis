extern crate actix_web;
extern crate log;
extern crate serde_json;

use actix_web::{web, App, HttpRequest, HttpServer};
use bb8_redis::{bb8, RedisConnectionManager, RedisPool};

pub const REDIS_POOL_SIZE: u32 = 100;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = RedisPool::new(
        bb8::Pool::builder()
            .max_size(REDIS_POOL_SIZE)
            .min_idle(Some(REDIS_POOL_SIZE - 2))
            .build(manager)
            .await
            .unwrap(),
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .keep_alive(actix_http::KeepAlive::Os)
    // .shutdown_timeout(30)
    // .maxconn(50000)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub async fn index(_request: HttpRequest, redis: web::Data<RedisPool>) -> std::io::Result<String> {
    let mut conn = redis.get().await.unwrap();
    let conn = conn.as_mut().unwrap();
    let result = redis::cmd("GET")
        .arg(&["key"])
        .query_async(conn)
        .await
        .unwrap();
    Ok(result)
}
