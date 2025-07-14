//! Fuzzy matching implementation for name and company searches.

use crate::errors::Result;
use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};

/// Result of a fuzzy match search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyMatchResult {
    pub name: String,
    pub similarity: f32,
    pub field_matched: String,
}

/// Performs fuzzy name matching using PostgreSQL trigram similarity
pub async fn fuzzy_match_names(
    pool: &PgPool,
    search_term: &str,
    threshold: f32,
) -> Result<Vec<FuzzyMatchResult>> {
    let query = "
        SELECT name, similarity(name, $1) as sim
        FROM users
        WHERE similarity(name, $1) > $2
        ORDER BY sim DESC
        LIMIT 20
    ";
    
    let rows = sqlx::query(query)
        .bind(search_term)
        .bind(threshold)
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter().map(|row| FuzzyMatchResult {
        name: row.get("name"),
        similarity: row.get("sim"),
        field_matched: "name".to_string(),
    }).collect())
}

/// Performs fuzzy company matching
pub async fn fuzzy_match_companies(
    pool: &PgPool,
    search_term: &str,
    threshold: f32,
) -> Result<Vec<FuzzyMatchResult>> {
    let query = "
        SELECT DISTINCT company, similarity(company, $1) as sim
        FROM users
        WHERE company IS NOT NULL 
        AND similarity(company, $1) > $2
        ORDER BY sim DESC
        LIMIT 20
    ";
    
    let rows = sqlx::query(query)
        .bind(search_term)
        .bind(threshold)
        .fetch_all(pool)
        .await?;
    
    Ok(rows.into_iter().map(|row| FuzzyMatchResult {
        name: row.get("company"),
        similarity: row.get("sim"),
        field_matched: "company".to_string(),
    }).collect())
}

/// Finds similar names using Levenshtein distance
pub async fn find_similar_names(
    pool: &PgPool,
    name: &str,
    max_distance: i32,
) -> Result<Vec<String>> {
    // First check if pg_trgm extension supports levenshtein
    let check_extension = sqlx::query("SELECT 1 FROM pg_extension WHERE extname = 'fuzzystrmatch'")
        .fetch_optional(pool)
        .await?;
    
    if check_extension.is_none() {
        // Try to create the extension
        let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS fuzzystrmatch")
            .execute(pool)
            .await;
    }
    
    let query = "
        SELECT name
        FROM users
        WHERE levenshtein(lower(name), lower($1)) <= $2
        AND name != $1
        ORDER BY levenshtein(lower(name), lower($1))
        LIMIT 10
    ";
    
    match sqlx::query(query)
        .bind(name)
        .bind(max_distance)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => {
            Ok(rows.into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect())
        }
        Err(_) => {
            // Fallback to trigram similarity if levenshtein is not available
            let fallback_query = "
                SELECT name
                FROM users
                WHERE similarity(name, $1) > 0.3
                AND name != $1
                ORDER BY similarity(name, $1) DESC
                LIMIT 10
            ";
            
            let rows = sqlx::query(fallback_query)
                .bind(name)
                .fetch_all(pool)
                .await?;
            
            Ok(rows.into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect())
        }
    }
}

/// Suggests corrections for misspelled names
pub async fn suggest_name_corrections(
    pool: &PgPool,
    misspelled: &str,
) -> Result<Vec<String>> {
    // Use a combination of trigram similarity and soundex matching
    let query = "
        SELECT DISTINCT name
        FROM users
        WHERE (
            similarity(name, $1) > 0.4
            OR (
                soundex(name) = soundex($1)
                AND length(name) BETWEEN length($1) - 3 AND length($1) + 3
            )
        )
        ORDER BY similarity(name, $1) DESC
        LIMIT 5
    ";
    
    match sqlx::query(query)
        .bind(misspelled)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => {
            Ok(rows.into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect())
        }
        Err(_) => {
            // Fallback to simple trigram similarity
            let fallback_query = "
                SELECT name
                FROM users
                WHERE similarity(name, $1) > 0.4
                ORDER BY similarity(name, $1) DESC
                LIMIT 5
            ";
            
            let rows = sqlx::query(fallback_query)
                .bind(misspelled)
                .fetch_all(pool)
                .await?;
            
            Ok(rows.into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect())
        }
    }
}

/// Performs phonetic matching for names that sound similar
pub async fn phonetic_match(
    pool: &PgPool,
    name: &str,
) -> Result<Vec<String>> {
    // Try to use metaphone for better phonetic matching
    let query = "
        SELECT DISTINCT name
        FROM users
        WHERE dmetaphone(name) = dmetaphone($1)
        AND name != $1
        LIMIT 10
    ";
    
    match sqlx::query(query)
        .bind(name)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => {
            Ok(rows.into_iter()
                .map(|row| row.get::<String, _>("name"))
                .collect())
        }
        Err(_) => {
            // Fallback to soundex
            let fallback_query = "
                SELECT DISTINCT name
                FROM users
                WHERE soundex(name) = soundex($1)
                AND name != $1
                LIMIT 10
            ";
            
            match sqlx::query(fallback_query)
                .bind(name)
                .fetch_all(pool)
                .await
            {
                Ok(rows) => {
                    Ok(rows.into_iter()
                        .map(|row| row.get::<String, _>("name"))
                        .collect())
                }
                Err(_) => Ok(vec![]) // No phonetic matching available
            }
        }
    }
}