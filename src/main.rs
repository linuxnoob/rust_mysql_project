use std::env;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::MySql;
use sqlx::MySqlPool;

use tokio;

mod models; // Import models

// Import the SearchPayload structure
use models::SearchPayload;

async fn init_database() -> Result<sqlx::Pool<MySql>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect(&database_url)
        .await?;
    
    println!("Database connection established!");
    Ok(pool)
}

async fn search_films(
    pool: web::Data<MySqlPool>, 
    payload: web::Json<SearchPayload>
) -> impl Responder {
    let query = payload.query.clone();

    // Execute full-text search query
    let results = sqlx::query!(
        r#"
        SELECT ft.title, GROUP_CONCAT(CONCAT(a.first_name, ' ', a.last_name) SEPARATOR ', ') AS actors
        FROM film AS ft
        JOIN film_actor AS fa ON ft.film_id = fa.film_id
        JOIN actor AS a ON fa.actor_id = a.actor_id
        WHERE MATCH(ft.description) AGAINST (? IN NATURAL LANGUAGE MODE)
        GROUP BY ft.title
        "#,
        query
    )
    .fetch_all(pool.get_ref())
    .await;

    // Process results
    match results {
        Ok(records) => {
            let films: Vec<_> = records
                .iter()
                .map(|record| {
                    serde_json::json!({
                        "title": record.title.clone(),
                        "actors": record.actors.clone().unwrap_or_default()
                    })
                })
                .collect();

            HttpResponse::Ok().json(films) // Return JSON response
        }
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to execute query")
        }
    }
}

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     match init_database().await {
//         Ok(_) => println!("Database connected successfully."),
//         Err(e) => eprintln!("Failed to connect to database: {}", e),
//     }

//     // You can skip starting the server to test just the connection.
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initialize the MySQL connection pool
    let database_pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    // Start the Actix server and define routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .route("/search", web::post().to(search_films)) // Route to search handler
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

