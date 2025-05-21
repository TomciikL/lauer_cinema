use axum::{extract::Query, Json};
use loco_rs::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct DetailParams {
    pub id: u32,
    #[serde(default = "default_media_type")]
    pub r#type: String, // "movie" nebo "tv"
}

fn default_media_type() -> String {
    "movie".to_string()
}

#[derive(Serialize)]
pub struct MediaDetail {
    pub id: u32,
    pub title: String,
    pub overview: String,
    pub release_date: String,
    pub genres: Vec<String>,
    pub poster_url: String,
    pub rating: f32,
}

#[derive(Debug, Deserialize)]
struct Genre {
    name: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    id: u32,
    title: Option<String>,
    name: Option<String>,
    overview: String,
    release_date: Option<String>,
    first_air_date: Option<String>,
    genres: Vec<Genre>,
    poster_path: Option<String>,
    vote_average: f32,
}

pub async fn detail(Query(params): Query<DetailParams>) -> Result<Json<MediaDetail>> {
    let api_key = env::var("TMDB_API_KEY").expect("Missing TMDB_API_KEY");
    let client = Client::new();
    tracing::info!("Získávám detail: {} {}", params.r#type, params.id);

    let url = format!(
        "https://api.themoviedb.org/3/{}/{}?api_key={}&language=cs-CZ",
        params.r#type, params.id, api_key
    );

    let res = client.get(&url).send().await?.json::<ApiResponse>().await?;

    let detail = MediaDetail {
        id: res.id,
        title: res.title.or(res.name).unwrap_or_default(),
        overview: res.overview,
        release_date: res.release_date.or(res.first_air_date).unwrap_or_default(),
        genres: res.genres.into_iter().map(|g| g.name).collect(),
        poster_url: res
            .poster_path
            .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p))
            .unwrap_or_default(),
        rating: res.vote_average,
    };

    Ok(Json(detail))
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/detail/").add("/", get(detail))
}
