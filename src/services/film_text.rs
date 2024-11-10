use actix_web::{post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::models::NewFilmDescription;

#[post("/add_film_description")]
async fn add_film_description(
    pool: web::Data<MySqlPool>,
    film_description: web::Json<NewFilmDescription>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO film_text (film_id, description) VALUES (?, ?)",
        film_description.film_id,
        film_description.description
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Film description added successfully"),
        Err(e) => {
            eprintln!("Error adding film description: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add film description")
        }
    }
}
