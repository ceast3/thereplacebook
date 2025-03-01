use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, Router,
};
use sqlx::{PgPool, FromRow}; // Removed `Pool`
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use tower_http::services::ServeDir;
use aws_smithy_http::result::SdkError; // ✅ Fix for missing `aws_smithy_http`
use aws_config::{BehaviorVersion, Region}; // ✅ Added correct AWS SDK imports
use aws_sdk_secretsmanager::Client;

#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

#[derive(FromRow, Debug, serde::Serialize)]
struct User {
    id: i32, // ✅ Fixed type mismatch (was `i64`, expected `i32`)
    name: String,
    image_url: String,
    rating: f64,
}

#[derive(serde::Deserialize)]
struct MatchResult {
    winner_id: i32, // ✅ Fixed type mismatch
    loser_id: i32,  // ✅ Fixed type mismatch
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, image_url, rating FROM users ORDER BY rating DESC"
    )
        .fetch_all(&*state.db)
        .await
        .unwrap();
    Json(users)
}

async fn submit_match(
    State(state): State<AppState>,
    Json(match_result): Json<MatchResult>,
) -> StatusCode {
    let row1 = sqlx::query!("SELECT rating FROM users WHERE id = $1", match_result.winner_id)
        .fetch_one(&*state.db)
        .await
        .unwrap();

    let row2 = sqlx::query!("SELECT rating FROM users WHERE id = $1", match_result.loser_id)
        .fetch_one(&*state.db)
        .await
        .unwrap();

    let rating_winner: f64 = row1.rating.unwrap_or(1000.0);
    let rating_loser: f64 = row2.rating.unwrap_or(1000.0);

    let k = 32.0;
    let e_winner = 1.0 / (1.0 + 10f64.powf((rating_loser - rating_winner) / 400.0));
    let e_loser = 1.0 / (1.0 + 10f64.powf((rating_winner - rating_loser) / 400.0));

    let new_winner_rating = rating_winner + k * (1.0 - e_winner);
    let new_loser_rating = rating_loser + k * (0.0 - e_loser);

    sqlx::query!(
        "UPDATE users SET rating = $1 WHERE id = $2",
        new_winner_rating,
        match_result.winner_id
    )
        .execute(&*state.db)
        .await
        .unwrap();

    sqlx::query!(
        "UPDATE users SET rating = $1 WHERE id = $2",
        new_loser_rating,
        match_result.loser_id
    )
        .execute(&*state.db)
        .await
        .unwrap();

    sqlx::query!(
        "INSERT INTO matches (winner_id, loser_id) VALUES ($1, $2)",
        match_result.winner_id,
        match_result.loser_id
    )
        .execute(&*state.db)
        .await
        .unwrap();

    StatusCode::OK
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // ✅ Fixed database connection type
    let db = PgPool::connect(&database_url).await.unwrap();
    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/match", post(submit_match))
        .nest_service("/", ServeDir::new("static")) // ✅ Serves static files
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::Server::bind(&listener.local_addr().unwrap()) // ✅ Fixed `serve` error
        .serve(app.into_make_service())
        .await
        .unwrap();
}