// src/services/search.rs

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use crate::models::{Actor, FilmSearchResult};
use regex::Regex;

#[derive(Deserialize)]
pub struct SearchPayload {
    pub query: String,
}

fn validate_search_query(query: &str) -> bool {
    // Only allows alphanumeric characters and spaces
    let re = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
    re.is_match(query)
}

#[get("/search")]
pub async fn search_films(
    pool: web::Data<MySqlPool>,
    payload: web::Query<SearchPayload>,
) -> impl Responder {
    let query = &payload.query;

    // Validate search query input
    if !validate_search_query(query) {
        return HttpResponse::BadRequest().body("Invalid search query input.");
    }

    let sql = r#"
        SELECT ft.title, a.actor_id, a.first_name, a.last_name
        FROM film AS ft
        JOIN film_actor AS fa ON ft.film_id = fa.film_id
        JOIN actor AS a ON fa.actor_id = a.actor_id
        WHERE MATCH(ft.description) AGAINST(? IN NATURAL LANGUAGE MODE)
        ORDER BY ft.title;
    "#;

    match sqlx::query(sql)
        .bind(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let mut films: Vec<FilmSearchResult> = Vec::new();
            let mut current_title = String::new();
            let mut current_actors: Vec<Actor> = Vec::new();

            for row in rows {
                let title: String = row.get("title");
                let actor = Actor {
                    actor_id: row.get("actor_id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                };

                if title != current_title && !current_title.is_empty() {
                    films.push(FilmSearchResult {
                        title: current_title.clone(),
                        actors: current_actors.clone(),
                    });
                    current_actors.clear();
                }

                current_title = title.clone();
                current_actors.push(actor);
            }

            if !current_title.is_empty() {
                films.push(FilmSearchResult {
                    title: current_title,
                    actors: current_actors,
                });
            }

            HttpResponse::Ok().json(films)
        }
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to execute search query")
        }
    }
}
