use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager::Client as SecretsClient;
use serde_json::Value;
use sqlx::{Pool, PgPool, FromRow};
use axum::{extract::State, routing::{get, post}, http::StatusCode, Json, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use Hyper::Server;
use aws_smithy_http::result::SdkError;


#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

#[derive(FromRow, Debug, serde::Serialize)]
struct User {
    id: i64,
    name: String,
    image_url: String,
    rating: f64,
}

#[derive(serde::Deserialize)]
struct MatchResult {
    winner_id: i64,
    loser_id: i64,
}

// 🔹 Function to get `DATABASE_URL` from AWS Secrets Manager
async fn get_database_url() -> Result<String, aws_sdk_secretsmanager::Error> {
    let secret_name = "replacebook-db-secret";  // 🔹 Your AWS Secrets Manager secret name
    let region = Region::new("us-east-1");      // 🔹 Update if using a different region

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let client = SecretsClient::new(&config);

    let response = client.get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;

    if let Some(secret_string) = response.secret_string() {
        let parsed: Value = serde_json::from_str(&secret_string).expect("Invalid JSON in Secrets Manager");
        if let Some(db_url) = parsed.get("DATABASE_URL").and_then(|s| s.as_str()) {
            return Ok(db_url.to_string());
        }
    }


    Err(SdkError::construction_failure("Secret not found".to_string()))}

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

async fn submit_match(State(state): State<AppState>, Json(match_result): Json<MatchResult>) -> StatusCode {
    let row1 = sqlx::query!("SELECT rating FROM users WHERE id = $1", match_result.winner_id)
        .fetch_one(&*state.db)
        .await
        .unwrap();

    let row2 = sqlx::query!("SELECT rating FROM users WHERE id = $1", match_result.loser_id)
        .fetch_one(&*state.db)
        .await
        .unwrap();

    let rating_winner = row1.rating.unwrap();
    let rating_loser = row2.rating.unwrap();

    let k = 32.0;
    let e_winner = 1.0 / (1.0 + 10f64.powf((rating_loser - rating_winner) / 400.0));
    let e_loser = 1.0 / (1.0 + 10f64.powf((rating_winner - rating_loser) / 400.0));

    let new_winner_rating = rating_winner + k * (1.0 - e_winner);
    let new_loser_rating = rating_loser + k * (0.0 - e_loser);

    sqlx::query!("UPDATE users SET rating = $1 WHERE id = $2", new_winner_rating, match_result.winner_id)
        .execute(&*state.db)
        .await
        .unwrap();

    sqlx::query!("UPDATE users SET rating = $1 WHERE id = $2", new_loser_rating, match_result.loser_id)
        .execute(&*state.db)
        .await
        .unwrap();

    sqlx::query!("INSERT INTO matches (winner_id, loser_id) VALUES ($1, $2)", match_result.winner_id, match_result.loser_id)
        .execute(&*state.db)
        .await
        .unwrap();

    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = get_database_url().await.expect("Failed to retrieve database URL");

    let db = PgPool::connect(&database_url).await?;
    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/match", post(submit_match))
        .nest_service("/", ServeDir::new("static")) // Serve static files
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}