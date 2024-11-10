use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;

mod db;
mod models;
mod services;

use services::actor::add_actor;
use services::film::add_film;
use services::film_text::add_film_description;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: MySqlPool = db::get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_actor)               // Register actor route
            .service(add_film)                // Register film route
            .service(add_film_description)    // Register film description route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
