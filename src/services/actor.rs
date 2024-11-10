// src/services/actor.rs

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;
use regex::Regex;

#[derive(Deserialize)]
pub struct AddActorPayload {
    pub first_name: String,
    pub last_name: String,
}

fn validate_name(name: &str) -> bool {
    // Only allows alphabetic characters and spaces
    let re = Regex::new(r"^[a-zA-Z ]+$").unwrap();
    re.is_match(name)
}

#[post("/actors/add")]
pub async fn add_actor(
    pool: web::Data<MySqlPool>,
    payload: web::Json<AddActorPayload>,
) -> impl Responder {
    let first_name = &payload.first_name;
    let last_name = &payload.last_name;

    // Validate names for alphanumeric characters and spaces only
    if !validate_name(first_name) || !validate_name(last_name) {
        return HttpResponse::BadRequest().body("Invalid characters in actor's name.");
    }

    let sql = "INSERT INTO actor (first_name, last_name) VALUES (?, ?)";
    match sqlx::query(sql)
        .bind(first_name)
        .bind(last_name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Actor added successfully."),
        Err(e) => {
            eprintln!("Error adding actor: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add actor.")
        }
    }
}
