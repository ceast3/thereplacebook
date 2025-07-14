use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

pub struct MatchingEngine {
    pool: PgPool,
}

impl MatchingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

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

    fn parse_net_worth(&self, net_worth_str: &str) -> f64 {
        net_worth_str
            .replace("$", "")
            .replace("B", "")
            .parse::<f64>()
            .unwrap_or(0.0)
    }
}

pub struct Analytics {
    pool: PgPool,
}

impl Analytics {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

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