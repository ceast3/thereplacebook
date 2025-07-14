//! REST API endpoints for search functionality.

use super::{SearchEngine, SearchQuery, SearchResult, fuzzy_match};
use crate::errors::AppError;
use axum::{
    extract::{Query, State, Path},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

/// API state containing search engine
pub struct SearchApiState {
    pub search_engine: Arc<SearchEngine>,
    pub pool: PgPool,
}

/// Response wrapper for API endpoints
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

/// Search request parameters for GET endpoint
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub industry: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub min_wealth: Option<f64>,
    pub max_wealth: Option<f64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Handles search requests
pub async fn search_handler(
    Query(params): Query<SearchParams>,
    State(state): State<Arc<SearchApiState>>,
) -> impl IntoResponse {
    let query = SearchQuery {
        query: params.q,
        industry: params.industry,
        country: params.country,
        company: params.company,
        min_wealth: params.min_wealth,
        max_wealth: params.max_wealth,
        limit: params.limit,
        offset: params.offset,
        ..Default::default()
    };

    match state.search_engine.search(query).await {
        Ok(results) => (StatusCode::OK, Json(ApiResponse::success(results))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<SearchResult>>::error(e.to_string())),
        ),
    }
}

/// Autocomplete suggestions endpoint
#[derive(Debug, Deserialize)]
pub struct AutocompleteParams {
    pub q: String,
    pub field: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AutocompleteResponse {
    pub suggestions: Vec<String>,
}

pub async fn autocomplete_handler(
    Query(params): Query<AutocompleteParams>,
    State(state): State<Arc<SearchApiState>>,
) -> impl IntoResponse {
    let field = params.field.as_deref().unwrap_or("name");
    
    let suggestions = match field {
        "name" => {
            match fuzzy_match::fuzzy_match_names(&state.pool, &params.q, 0.3).await {
                Ok(matches) => matches.into_iter().map(|m| m.name).collect(),
                Err(_) => vec![],
            }
        }
        "company" => {
            match fuzzy_match::fuzzy_match_companies(&state.pool, &params.q, 0.3).await {
                Ok(matches) => matches.into_iter().map(|m| m.name).collect(),
                Err(_) => vec![],
            }
        }
        _ => vec![],
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(AutocompleteResponse { suggestions })),
    )
}

/// Fuzzy name search endpoint
pub async fn fuzzy_search_handler(
    Path(name): Path<String>,
    State(state): State<Arc<SearchApiState>>,
) -> impl IntoResponse {
    match fuzzy_match::find_similar_names(&state.pool, &name, 3).await {
        Ok(names) => (StatusCode::OK, Json(ApiResponse::success(names))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<String>>::error(e.to_string())),
        ),
    }
}

/// Creates the search API router
pub fn create_search_router(pool: PgPool) -> axum::Router {
    let state = Arc::new(SearchApiState {
        search_engine: Arc::new(SearchEngine::new(pool.clone())),
        pool,
    });

    axum::Router::new()
        .route("/api/search", axum::routing::get(search_handler))
        .route("/api/autocomplete", axum::routing::get(autocomplete_handler))
        .route("/api/fuzzy/:name", axum::routing::get(fuzzy_search_handler))
        .with_state(state)
}