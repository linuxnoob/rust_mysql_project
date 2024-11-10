
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use crate::models::{Actor, FilmSearchResult};

#[derive(Deserialize)]
pub struct SearchPayload {
    pub query: String,
}

#[get("/search")]
pub async fn search_films(
    pool: web::Data<MySqlPool>,
    payload: web::Query<SearchPayload>,
) -> impl Responder {
    let query = &payload.query;

    let sql = r#"
        SELECT ft.title, a.actor_id, a.first_name, a.last_name
        FROM film AS ft
        JOIN film_actor AS fa ON ft.film_id = fa.film_id
        JOIN actor AS a ON fa.actor_id = a.actor_id
        WHERE MATCH(ft.description) AGAINST(? IN NATURAL LANGUAGE MODE)
        ORDER BY ft.title;
    "#;

    let result = sqlx::query(sql)
        .bind(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            // Initialize variables to collect grouped films and actors
            let mut films: Vec<FilmSearchResult> = Vec::new();
           // let mut current_film_id: Option<u16> = None;
            let mut current_title = String::new();
            let mut current_actors: Vec<Actor> = Vec::new();

            for row in rows {
                //let film_id: u16 = row.get("film_id");
                let title: String = row.get("title");
                let actor = Actor {
                    actor_id: row.get("actor_id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                };

                if title != current_title && !current_title.is_empty() {
                    // Push the completed film with its actors into the films list
                    films.push(FilmSearchResult {
                        //film_id: current_film_id.unwrap(),
                        title: current_title.clone(),
                        actors: current_actors.clone(),
                    });
                    current_actors.clear();
                }

                //current_film_id = Some(film_id);
                current_title = title.clone();
                current_actors.push(actor);
            }

            // Add the last film
            if !current_title.is_empty() {
                films.push(FilmSearchResult {
                    title: current_title,
                    actors: current_actors,
                });
            }

            HttpResponse::Ok().json(films)
        }
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            HttpResponse::InternalServerError().body("Failed to execute search query")
        }
    }
}
