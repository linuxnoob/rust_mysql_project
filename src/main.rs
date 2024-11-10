use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;

mod db;
mod models;
mod services;
mod search;

use services::actor::add_actor;
use services::film::add_film;
use services::film_text::add_film_description;
 // Import search handler
use crate::search::search_films;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    let pool: MySqlPool = db::get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_actor)
            .service(add_film)
            .service(add_film_description)
            .service(search_films)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}