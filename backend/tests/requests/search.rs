use backend::app::App;
use dotenvy::dotenv;
use loco_rs::testing::prelude::*;
use serde_json::Value;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_search_movies() {
    dotenv().ok(); // Načti TMDB_API_KEY z .env souboru

    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/search?query=Inception").await;
        assert_eq!(res.status_code(), 200);

        let json: Value = res.json::<Value>();
        assert!(json.is_array());

        let arr = json.as_array().unwrap();
        assert!(!arr.is_empty(), "Expected at least one result");

        let found = arr.iter().any(|item| {
            item["title"]
                .as_str()
                .map(|t| t.to_lowercase().contains("počátek"))
                .unwrap_or(false)
        });

        assert!(found, "Expected to find movie with title 'Počátek'");
    })
    .await;
}
