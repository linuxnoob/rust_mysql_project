use serde::{Deserialize, Serialize};


// Payload for creating a new actor
#[derive(Debug, Serialize, Deserialize)]
pub struct NewActor {
    pub first_name: String,
    pub last_name: String,
}

// Payload for creating a new film
#[derive(Debug, Serialize, Deserialize)]
pub struct NewFilm {
    pub title: String,
    pub description: Option<String>,
}

// Payload for creating a new film description
#[derive(Debug, Serialize, Deserialize)]
pub struct NewFilmDescription {
    pub film_id: i32,
    pub description: String,
}

#[derive(Serialize, Clone)]
pub struct Actor {
    pub actor_id: u16,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct FilmSearchResult {
    pub title: String,
    pub actors: Vec<Actor>, // List of actors for each film
}

