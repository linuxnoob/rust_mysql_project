use actix_web::{post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::models::NewFilm;

#[post("/add_film")]
async fn add_film(
    pool: web::Data<MySqlPool>,
    film: web::Json<NewFilm>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO film (title, description, last_update) VALUES (?, ?, NOW())",
        film.title,
        film.description
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Film added successfully"),
        Err(e) => {
            eprintln!("Error adding film: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add film")
        }
    }
}
