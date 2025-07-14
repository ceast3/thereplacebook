//! Feature implementations for matching and analytics.
//!
//! This module provides the core business logic for:
//! - Matching billionaires by various criteria
//! - Generating analytics and distributions
//! - Computing wealth tier breakdowns

use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

/// Engine for matching billionaires based on various criteria.
/// 
/// The MatchingEngine provides methods to filter and search for
/// billionaires based on industry, country, wealth tiers, and more.
pub struct MatchingEngine {
    pool: PgPool,
}

impl MatchingEngine {
    /// Creates a new MatchingEngine with the given database pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Finds billionaires matching a specific industry.
    /// 
    /// # Arguments
    /// * `industry` - Industry name to search for (case-insensitive partial match)
    /// * `limit` - Maximum number of results (defaults to 50)
    /// 
    /// # Example
    /// ```
    /// let tech_billionaires = engine.find_matches_by_industry("technology", Some(10)).await?;
    /// ```
    pub async fn find_matches_by_industry(&self, industry: &str, limit: Option<usize>) -> Result<Vec<Billionaire>> {
        let limit_clause = limit.unwrap_or(50);
        
        let rows = sqlx::query(
            "SELECT name, net_worth, industry, nationality, source_of_wealth, biography, rating
             FROM users 
             WHERE industry ILIKE $1 
             ORDER BY rating DESC 
             LIMIT $2"
        )
        .bind(format!("%{}%", industry))
        .bind(limit_clause as i64)
        .fetch_all(&self.pool)
        .await?;

        let billionaires: Vec<Billionaire> = rows
            .into_iter()
            .map(|row| Billionaire {
                name: row.get("name"),
                net_worth: self.parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
                source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
                age: None,
                country: row.get::<Option<String>, _>("nationality").unwrap_or_default(),
                industry: row.get::<Option<String>, _>("industry").unwrap_or_default(),
                bio: row.get::<Option<String>, _>("biography"),
                company: None,
                philanthropy: None,
                notable_achievements: None,
                website: None,
                twitter_handle: None,
                linkedin_profile: None,
                quote: None,
                birthdate: None,
                image_url: None,
                parental_wealth: None,
            })
            .collect();

        Ok(billionaires)
    }

    /// Finds billionaires within a specific wealth range.
    /// 
    /// # Arguments
    /// * `min_wealth` - Minimum net worth in billions
    /// * `max_wealth` - Maximum net worth in billions
    /// 
    /// # Example
    /// ```
    /// // Find billionaires worth between $5B and $20B
    /// let mid_tier = engine.find_matches_by_wealth_tier(5.0, 20.0).await?;
    /// ```
    pub async fn find_matches_by_wealth_tier(&self, min_wealth: f64, max_wealth: f64) -> Result<Vec<Billionaire>> {
        let rows = sqlx::query(
            "SELECT name, net_worth, industry, nationality, source_of_wealth, biography, rating
             FROM users 
             WHERE CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) BETWEEN $1 AND $2
             ORDER BY rating DESC"
        )
        .bind(min_wealth)
        .bind(max_wealth)
        .fetch_all(&self.pool)
        .await?;

        let billionaires: Vec<Billionaire> = rows
            .into_iter()
            .map(|row| Billionaire {
                name: row.get("name"),
                net_worth: self.parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
                source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
                age: None,
                country: row.get::<Option<String>, _>("nationality").unwrap_or_default(),
                industry: row.get::<Option<String>, _>("industry").unwrap_or_default(),
                bio: row.get::<Option<String>, _>("biography"),
                company: None,
                philanthropy: None,
                notable_achievements: None,
                website: None,
                twitter_handle: None,
                linkedin_profile: None,
                quote: None,
                birthdate: None,
                image_url: None,
                parental_wealth: None,
            })
            .collect();

        Ok(billionaires)
    }

    /// Finds billionaires from a specific country.
    /// 
    /// # Arguments
    /// * `country` - Country name to search for (case-insensitive partial match)
    /// 
    /// # Example
    /// ```
    /// let us_billionaires = engine.find_matches_by_country("United States").await?;
    /// ```
    pub async fn find_matches_by_country(&self, country: &str) -> Result<Vec<Billionaire>> {
        let rows = sqlx::query(
            "SELECT name, net_worth, industry, nationality, source_of_wealth, biography, rating
             FROM users 
             WHERE nationality ILIKE $1 
             ORDER BY rating DESC"
        )
        .bind(format!("%{}%", country))
        .fetch_all(&self.pool)
        .await?;

        let billionaires: Vec<Billionaire> = rows
            .into_iter()
            .map(|row| Billionaire {
                name: row.get("name"),
                net_worth: self.parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
                source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
                age: None,
                country: row.get::<Option<String>, _>("nationality").unwrap_or_default(),
                industry: row.get::<Option<String>, _>("industry").unwrap_or_default(),
                bio: row.get::<Option<String>, _>("biography"),
                company: None,
                philanthropy: None,
                notable_achievements: None,
                website: None,
                twitter_handle: None,
                linkedin_profile: None,
                quote: None,
                birthdate: None,
                image_url: None,
                parental_wealth: None,
            })
            .collect();

        Ok(billionaires)
    }

    /// Parses net worth string format (e.g., "$12.5B") to numeric value.
    fn parse_net_worth(&self, net_worth_str: &str) -> f64 {
        net_worth_str
            .replace("$", "")
            .replace("B", "")
            .parse::<f64>()
            .unwrap_or(0.0)
    }
}

/// Analytics engine for generating insights and distributions.
/// 
/// The Analytics struct provides methods to analyze the billionaire
/// data and generate various distributions and statistics.
pub struct Analytics {
    pool: PgPool,
}

impl Analytics {
    /// Creates a new Analytics engine with the given database pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Gets the distribution of billionaires by industry.
    /// 
    /// # Returns
    /// * `HashMap<String, usize>` - Map of industry name to count
    /// 
    /// # Example
    /// ```
    /// let distribution = analytics.get_industry_distribution().await?;
    /// for (industry, count) in distribution {
    ///     println!("{}: {} billionaires", industry, count);
    /// }
    /// ```
    pub async fn get_industry_distribution(&self) -> Result<HashMap<String, usize>> {
        let rows = sqlx::query(
            "SELECT industry, COUNT(*) as count FROM users WHERE industry IS NOT NULL GROUP BY industry"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut distribution = HashMap::new();
        for row in rows {
            if let Some(industry) = row.get::<Option<String>, _>("industry") {
                distribution.insert(industry, row.get::<Option<i64>, _>("count").unwrap_or(0) as usize);
            }
        }

        Ok(distribution)
    }

    /// Gets the distribution of billionaires by country.
    /// 
    /// # Returns
    /// * `HashMap<String, usize>` - Map of country name to count
    pub async fn get_country_distribution(&self) -> Result<HashMap<String, usize>> {
        let rows = sqlx::query(
            "SELECT nationality, COUNT(*) as count FROM users WHERE nationality IS NOT NULL GROUP BY nationality"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut distribution = HashMap::new();
        for row in rows {
            if let Some(country) = row.get::<Option<String>, _>("nationality") {
                distribution.insert(country, row.get::<Option<i64>, _>("count").unwrap_or(0) as usize);
            }
        }

        Ok(distribution)
    }

    /// Gets the distribution of billionaires by wealth tiers.
    /// 
    /// Wealth tiers are defined as:
    /// - 1-5B: Entry level billionaires
    /// - 5-20B: Mid-tier billionaires  
    /// - 20-50B: Upper tier
    /// - 50B+: Ultra wealthy
    /// 
    /// # Returns
    /// * `HashMap<String, usize>` - Map of tier name to count
    pub async fn get_wealth_tiers(&self) -> Result<HashMap<String, usize>> {
        let rows = sqlx::query(
            "SELECT 
                CASE 
                    WHEN CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) < 5 THEN '1-5B'
                    WHEN CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) < 20 THEN '5-20B'
                    WHEN CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) < 50 THEN '20-50B'
                    ELSE '50B+'
                END as tier,
                COUNT(*) as count
             FROM users 
             WHERE net_worth IS NOT NULL 
             GROUP BY tier"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tiers = HashMap::new();
        for row in rows {
            if let Some(tier) = row.get::<Option<String>, _>("tier") {
                tiers.insert(tier, row.get::<Option<i64>, _>("count").unwrap_or(0) as usize);
            }
        }

        Ok(tiers)
    }

    /// Gets the top philanthropists based on rating.
    /// 
    /// Note: This currently filters by those who have philanthropy data,
    /// but could be enhanced to score based on donation amounts.
    /// 
    /// # Arguments
    /// * `limit` - Maximum number of results to return
    /// 
    /// # Returns
    /// * `Vec<String>` - Names of top philanthropists
    pub async fn get_top_philanthropists(&self, limit: usize) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT name FROM users 
             WHERE philanthropy IS NOT NULL 
             ORDER BY rating DESC 
             LIMIT $1"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|row| row.get("name")).collect())
    }
}