use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use tower_http::services::ServeDir;
use axum::serve;

#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).await.unwrap();
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
