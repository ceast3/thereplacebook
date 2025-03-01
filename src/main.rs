use std::net::SocketAddr;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::Client;
use std::env;
use tokio;

#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

async fn fetch_database_url() -> Result<String, Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let secret_name = "DATABASE_URL"; // Secret name in AWS Secrets Manager
    let secret_value = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?
        .secret_string()
        .ok_or("Secret string is empty")?;

    Ok(secret_value)
}

#[tokio::main]
async fn main() {
    // Fetch the database URL from AWS Secrets Manager
    let database_url = match fetch_database_url().await {
        Ok(url) => url,
        Err(err) => {
            eprintln!("❌ Failed to fetch DATABASE_URL: {:?}", err);
            std::process::exit(1);
        }
    };

    // Connect to PostgreSQL
    let db = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("❌ Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(
        TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service()
    )
        .await
        .unwrap();
}