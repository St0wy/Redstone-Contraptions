mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;

use crate::db::get_db_pool;
use crate::handlers::register;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();

    let pool = get_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
