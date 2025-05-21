#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::{extract::Query, Json};
use loco_rs::{controller::middleware::secure_headers, prelude::*};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Deserialize)]
pub struct SearchParams {
    pub query: String,
    #[serde(default = "default_media_type")]
    pub media_type: String, // "movie" nebo "tv"
}

fn default_media_type() -> String {
    "movie".to_string()
}

#[derive(Serialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub overview: String,
    pub release_date: String,
    pub genres: Vec<String>,
    pub poster_url: String,
}

#[derive(Debug, Deserialize)]
struct Genre {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct GenreResponse {
    genres: Vec<Genre>,
}

pub async fn search(Query(params): Query<SearchParams>) -> Result<Json<Vec<Movie>>> {
    let api_key = env::var("TMDB_API_KEY").expect("Missing TMDB_API_KEY");
    let client = Client::new();

    let is_tv = params.media_type == "tv";
    tracing::info!(
        "Hledám {}: {}",
        if is_tv { "seriál" } else { "film" },
        params.query
    );

    // Získání žánrů (film vs tv)
    let genre_url = format!(
        "https://api.themoviedb.org/3/genre/{}/list?api_key={}&language=cs-CZ",
        params.media_type, api_key
    );
    let genre_res = client
        .get(&genre_url)
        .send()
        .await?
        .json::<GenreResponse>()
        .await?;
    let genre_map: HashMap<u32, String> = genre_res
        .genres
        .into_iter()
        .map(|g| (g.id, g.name))
        .collect();

    // Hledání podle typu
    let search_url = format!(
        "https://api.themoviedb.org/3/search/{}?api_key={}&query={}&include_adult=false&language=cs-CZ",
        params.media_type,
        api_key,
        params.query
    );
    let search_res = client
        .get(&search_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let results = search_res["results"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| {
            let genre_ids = item["genre_ids"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|id| id.as_u64().map(|i| i as u32))
                .filter_map(|id| genre_map.get(&id).cloned())
                .collect::<Vec<_>>();

            Movie {
                id: item["id"].as_u64().unwrap_or(0) as u32,
                title: item["title"]
                    .as_str()
                    .or_else(|| item["name"].as_str())
                    .unwrap_or("")
                    .to_string(),
                overview: item["overview"].as_str().unwrap_or("").to_string(),
                release_date: item["release_date"]
                    .as_str()
                    .or_else(|| item["first_air_date"].as_str())
                    .unwrap_or("")
                    .to_string(),
                genres: genre_ids,
                poster_url: item["poster_path"]
                    .as_str()
                    .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p))
                    .unwrap_or_default(),
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(results))
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/search/").add("/", get(search))
}
