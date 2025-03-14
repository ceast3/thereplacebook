use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    response::Json,
    Router,
};
use sqlx::{PgPool, FromRow, Row};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager::Client as SecretsClient;
use serde_json::Value;
use std::env;
use aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError;
use aws_sdk_secretsmanager::error::SdkError;

#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

#[derive(FromRow, Debug, serde::Serialize)]
struct User {
    id: i32,
    name: String,
    image_url: String,
    net_worth: Option<String>,  // ✅ Added net_worth (as TEXT)
    biography: Option<String>,  // ✅ Added biography
    company: Option<String>,    // ✅ Added company
    rating: Option<f64>,
}

#[derive(serde::Deserialize)]
struct NewUser {
    name: String,
    image_url: String,
}

#[derive(serde::Deserialize)]
struct MatchResult {
    winner_id: i32,
    loser_id: i32,
}
async fn add_user(
    State(state): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let inserted_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, image_url, rating) VALUES ($1, $2, $3) RETURNING id, name, image_url, rating"
    )
        .bind(new_user.name)
        .bind(new_user.image_url)
        .bind(1000.0) // Default rating
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inserted_user))
}

async fn get_database_url() -> Result<String, SdkError<GetSecretValueError>> {
    // ✅ First, check if DATABASE_URL is already set (for local dev)
    if let Ok(db_url) = env::var("DATABASE_URL") {
        println!("✅ Using DATABASE_URL from environment");
        return Ok(db_url);
    }

    let secret_name = "replacebook-db-secret";
    let region = Region::new("us-east-1");
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
        let parsed: Value = serde_json::from_str(&secret_string)
            .expect("Invalid JSON format");
        if let Some(db_url) = parsed.get("DATABASE_URL").and_then(|s| s.as_str()) {
            println!("✅ Retrieved DATABASE_URL from AWS Secrets Manager");
            return Ok(db_url.to_string());
        }
    }

    Err(SdkError::construction_failure("DATABASE_URL not found in secret".to_string()))
}
// 🔹 Fetch users from the database (✅ Fixed SQLx query)
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query("SELECT id, name, image_url, net_worth, company, biography, rating FROM users ORDER BY rating DESC")
        .fetch_all(&*state.db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| User {
            id: row.try_get("id").unwrap(),
            name: row.try_get("name").unwrap(),
            image_url: row.try_get("image_url").unwrap(),
            net_worth: row.try_get("net_worth").unwrap(),
            company: row.try_get("company").unwrap(),
            biography: row.try_get("biography").unwrap(),
            rating: Some(row.try_get("rating").unwrap_or(1000.0)), // ✅ Default rating if NULL
        })
        .collect();

    Json(users)
}

// 🔹 Handle match submission and update rankings (✅ Fixed SQLx queries)
async fn submit_match(
    State(state): State<AppState>,
    Json(match_result): Json<MatchResult>,
) -> Result<StatusCode, StatusCode> {
    // Fetch winner's rating
    let row1 = sqlx::query("SELECT rating FROM users WHERE id = $1")
        .bind(match_result.winner_id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Fetch loser's rating
    let row2 = sqlx::query("SELECT rating FROM users WHERE id = $1")
        .bind(match_result.loser_id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let rating_winner: f64 = row1.try_get("rating").unwrap_or(1000.0);
    let rating_loser: f64 = row2.try_get("rating").unwrap_or(1000.0);

    // Elo rating calculation
    let k = 32.0;
    let e_winner = 1.0 / (1.0 + 10f64.powf((rating_loser - rating_winner) / 400.0));
    let e_loser = 1.0 / (1.0 + 10f64.powf((rating_winner - rating_loser) / 400.0));

    let new_winner_rating = rating_winner + k * (1.0 - e_winner);
    let new_loser_rating = rating_loser + k * (0.0 - e_loser);

    // Update winner's rating
    sqlx::query("UPDATE users SET rating = $1 WHERE id = $2")
        .bind(new_winner_rating)
        .bind(match_result.winner_id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update loser's rating
    sqlx::query("UPDATE users SET rating = $1 WHERE id = $2")
        .bind(new_loser_rating)
        .bind(match_result.loser_id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert match result
    sqlx::query("INSERT INTO matches (winner_id, loser_id) VALUES ($1, $2)")
        .bind(match_result.winner_id)
        .bind(match_result.loser_id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// 🔹 Main function (✅ No compile-time DATABASE_URL needed)
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = get_database_url().await.expect("Failed to retrieve DATABASE_URL");
    env::set_var("DATABASE_URL", &database_url);
    println!("✅ Successfully retrieved DATABASE_URL from Secrets Manager!");

    // Now connect to the database
    let db = PgPool::connect(&database_url).await?;
    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/match", post(submit_match))
        .nest_service("/", ServeDir::new("static"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())}