use backend::app::App;
use loco_rs::cli;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenvy::dotenv().ok();
    cli::main::<App>().await
}
