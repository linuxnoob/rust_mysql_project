use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct SearchPayload {
    pub query: String,
}

#[get("/search")]
async fn search_films(
    pool: web::Data<MySqlPool>,
    payload: web::Query<SearchPayload>,
) -> impl Responder {
    let query = &payload.query;

    // Define SQL query with full-text search on description
    let sql = r#"
        SELECT ft.film_id, ft.title, a.actor_id, a.first_name, a.last_name
        FROM film AS ft
        JOIN film_actor AS fa ON ft.film_id = fa.film_id
        JOIN actor AS a ON fa.actor_id = a.actor_id
        WHERE MATCH(ft.description) AGAINST(? IN NATURAL LANGUAGE MODE)
        ORDER BY ft.film_id;
    "#;

    // Execute query
    let result = sqlx::query(sql)
        .bind(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            // Format the results to group actors by film title
            let mut films: Vec<(String, Vec<String>)> = Vec::new();
            let mut current_title = String::new();
            let mut current_actors = Vec::new();

            for row in rows {
                let title: String = row.get("title");
                let actor_name = format!("{} {}", row.get::<String, _>("first_name"), row.get::<String, _>("last_name"));

                if title != current_title && !current_title.is_empty() {
                    films.push((current_title.clone(), current_actors.clone()));
                    current_actors.clear();
                }

                current_title = title.clone();
                current_actors.push(actor_name);
            }
            // Push the last film and its actors
            if !current_title.is_empty() {
                films.push((current_title, current_actors));
            }

            HttpResponse::Ok().json(films)
        }
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            HttpResponse::InternalServerError().body("Failed to execute search query")
        }
    }
}
