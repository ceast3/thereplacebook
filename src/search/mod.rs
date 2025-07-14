//! Advanced search functionality for The Replacebook.
//!
//! This module provides full-text search capabilities, fuzzy matching,
//! and multi-criteria filtering for billionaire data.

pub mod full_text;
pub mod fuzzy_match;
pub mod filters;
pub mod api;

use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Text to search for in biographies and other text fields
    pub query: Option<String>,
    /// Filter by industry (partial match)
    pub industry: Option<String>,
    /// Filter by country/nationality (partial match)
    pub country: Option<String>,
    /// Filter by company affiliation
    pub company: Option<String>,
    /// Minimum net worth in billions
    pub min_wealth: Option<f64>,
    /// Maximum net worth in billions
    pub max_wealth: Option<f64>,
    /// Minimum age
    pub min_age: Option<u32>,
    /// Maximum age
    pub max_age: Option<u32>,
    /// Search in specific fields only
    pub search_fields: Option<Vec<SearchField>>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
    /// Sort order
    pub sort_by: Option<SortBy>,
    /// Sort direction
    pub sort_order: Option<SortOrder>,
}

/// Fields that can be searched
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SearchField {
    Biography,
    SourceOfWealth,
    NotableAchievements,
    Philanthropy,
    Quote,
    All,
}

/// Sort options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    NetWorth,
    Name,
    Rating,
    Age,
}

/// Sort direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub billionaire: Billionaire,
    pub relevance_score: f32,
    pub matched_fields: Vec<String>,
    pub highlight_snippets: Vec<HighlightSnippet>,
}

/// Highlighted text snippet showing matches
#[derive(Debug, Clone, Serialize)]
pub struct HighlightSnippet {
    pub field: String,
    pub snippet: String,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Main search engine
pub struct SearchEngine {
    pool: PgPool,
}

impl SearchEngine {
    /// Creates a new search engine
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Performs a comprehensive search based on the query parameters
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>> {
        // If there's a text query, use full-text search
        if let Some(text_query) = &query.query {
            full_text::search_biographies(&self.pool, text_query, &query).await
        } else {
            // Otherwise, use filter-based search
            filters::filter_search(&self.pool, &query).await
        }
    }

    /// Gets search suggestions based on partial input
    pub async fn get_suggestions(&self, partial: &str, field: SearchField) -> Result<Vec<String>> {
        match field {
            SearchField::Biography => {
                // For biography, we don't suggest (too much text)
                Ok(vec![])
            }
            _ => {
                // For other fields, get distinct values
                let column = match field {
                    SearchField::SourceOfWealth => "source_of_wealth",
                    SearchField::NotableAchievements => "notable_achievements",
                    SearchField::Philanthropy => "philanthropy",
                    SearchField::Quote => "quote",
                    _ => return Ok(vec![]),
                };

                let query_str = format!(
                    "SELECT DISTINCT {} FROM users WHERE {} ILIKE $1 LIMIT 10",
                    column, column
                );

                let rows = sqlx::query(&query_str)
                    .bind(format!("%{}%", partial))
                    .fetch_all(&self.pool)
                    .await?;

                Ok(rows
                    .into_iter()
                    .filter_map(|row| row.get::<Option<String>, _>(0))
                    .collect())
            }
        }
    }

    /// Creates database indexes for better search performance
    pub async fn create_search_indexes(&self) -> Result<()> {
        // Create GIN index for full-text search on biography
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_biography_fts ON users 
             USING GIN (to_tsvector('english', biography))"
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for other searchable text fields
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_source_of_wealth_fts ON users 
             USING GIN (to_tsvector('english', source_of_wealth))"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_notable_achievements_fts ON users 
             USING GIN (to_tsvector('english', notable_achievements))"
        )
        .execute(&self.pool)
        .await?;

        // Create trigram indexes for fuzzy matching
        sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_trgm")
            .execute(&self.pool)
            .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_name_trgm ON users 
             USING GIN (name gin_trgm_ops)"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_company_trgm ON users 
             USING GIN (company gin_trgm_ops)"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: None,
            industry: None,
            country: None,
            company: None,
            min_wealth: None,
            max_wealth: None,
            min_age: None,
            max_age: None,
            search_fields: None,
            limit: Some(50),
            offset: Some(0),
            sort_by: Some(SortBy::Relevance),
            sort_order: Some(SortOrder::Desc),
        }
    }
}