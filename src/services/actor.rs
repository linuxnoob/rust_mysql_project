use actix_web::{post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::models::NewActor;

#[post("/add_actor")]
async fn add_actor(
    pool: web::Data<MySqlPool>,
    actor: web::Json<NewActor>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO actor (first_name, last_name, last_update) VALUES (?, ?, NOW())",
        actor.first_name,
        actor.last_name
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Actor added successfully"),
        Err(e) => {
            eprintln!("Error adding actor: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add actor")
        }
    }
}
