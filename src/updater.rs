use crate::data_sources::DataSourceManager;
use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::{PgPool, Row};
use tracing::{info, error};


pub async fn update_billionaire_data(pool: &PgPool) -> Result<()> {
    info!("Fetching latest billionaire data from multiple sources...");
    
    let data_manager = DataSourceManager::new();
    let billionaires = data_manager.fetch_from_all_sources(Some(500)).await;
    
    info!("Found {} billionaires from all sources", billionaires.len());

    for billionaire in billionaires {
        if let Err(e) = update_billionaire_record(pool, &billionaire).await {
            error!("Failed to update {}: {}", billionaire.name, e);
        }
    }

    info!("Updated billionaire data successfully");
    Ok(())
}

async fn update_billionaire_record(pool: &PgPool, billionaire: &Billionaire) -> Result<()> {
    let net_worth_str = format!("${:.1}B", billionaire.net_worth);
    
    // Check if billionaire exists
    let existing = sqlx::query("SELECT id FROM users WHERE name = $1")
        .bind(&billionaire.name)
        .fetch_optional(pool)
        .await?;

    if let Some(_) = existing {
        // Update existing record
        sqlx::query(
            "UPDATE users SET 
                net_worth = $1, 
                industry = $2, 
                nationality = $3,
                source_of_wealth = $4,
                biography = COALESCE($5, biography)
            WHERE name = $6"
        )
        .bind(&net_worth_str)
        .bind(&billionaire.industry)
        .bind(&billionaire.country)
        .bind(&billionaire.source_of_wealth)
        .bind(&billionaire.bio)
        .bind(&billionaire.name)
        .execute(pool)
        .await?;
        
        info!("Updated: {} - {}", billionaire.name, net_worth_str);
    } else {
        // Insert new billionaire
        let default_image = "https://via.placeholder.com/300x300?text=No+Image";
        
        sqlx::query(
            "INSERT INTO users (
                name, image_url, net_worth, industry, nationality, 
                source_of_wealth, biography, rating
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&billionaire.name)
        .bind(default_image)
        .bind(&net_worth_str)
        .bind(&billionaire.industry)
        .bind(&billionaire.country)
        .bind(&billionaire.source_of_wealth)
        .bind(&billionaire.bio)
        .bind(1200.0) // Default rating
        .execute(pool)
        .await?;
        
        info!("Added new billionaire: {} - {}", billionaire.name, net_worth_str);
    }

    Ok(())
}

pub async fn populate_top_100(pool: &PgPool) -> Result<()> {
    info!("Fetching top 100 billionaires from multiple sources...");
    
    let data_manager = DataSourceManager::new();
    let mut billionaires = data_manager.fetch_from_all_sources(Some(200)).await;
    
    // Sort by net worth and take top 100
    billionaires.sort_by(|a, b| b.net_worth.partial_cmp(&a.net_worth).unwrap_or(std::cmp::Ordering::Equal));
    billionaires.truncate(100);
    
    info!("Processing top {} billionaires", billionaires.len());

    // Clear existing data
    sqlx::query("DELETE FROM matches").execute(pool).await?;
    sqlx::query("DELETE FROM users").execute(pool).await?;
    info!("Cleared existing data");

    // Insert top 100 billionaires
    for (rank, billionaire) in billionaires.iter().enumerate() {
        if let Err(e) = insert_billionaire_with_rank(pool, billionaire, rank + 1).await {
            error!("Failed to insert {}: {}", billionaire.name, e);
        }
    }

    info!("Successfully populated database with top 100 billionaires!");
    Ok(())
}

async fn insert_billionaire_with_rank(pool: &PgPool, billionaire: &Billionaire, rank: usize) -> Result<()> {
    let net_worth_str = format!("${:.1}B", billionaire.net_worth);
    let default_image = "https://via.placeholder.com/300x300?text=No+Image";
    
    // Higher rank = higher rating (reverse the rank for rating calculation)
    let rating = 1200.0 + ((101 - rank) as f64 * 10.0);
    
    sqlx::query(
        "INSERT INTO users (
            name, image_url, net_worth, industry, nationality, 
            source_of_wealth, biography, rating
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&billionaire.name)
    .bind(default_image)
    .bind(&net_worth_str)
    .bind(&billionaire.industry)
    .bind(&billionaire.country)
    .bind(&billionaire.source_of_wealth)
    .bind(&billionaire.bio)
    .bind(rating)
    .execute(pool)
    .await?;
    
    info!("#{}: {} - {} (Rating: {:.0})", rank, billionaire.name, net_worth_str, rating);
    Ok(())
}

pub async fn update_bio_from_wikipedia(pool: &PgPool, name: &str) -> Result<()> {
    let data_manager = DataSourceManager::new();
    
    if let Some(enriched_data) = data_manager.enrich_person_data(name).await {
        let mut update_fields = Vec::new();
        let mut values: Vec<&str> = Vec::new();
        
        if let Some(bio) = &enriched_data.bio {
            update_fields.push("biography = $1");
            values.push(bio);
        }
        
        if let Some(company) = &enriched_data.company {
            update_fields.push("company = $2");
            values.push(company);
        }
        
        if !update_fields.is_empty() {
            let query = format!(
                "UPDATE users SET {} WHERE name = ${}",
                update_fields.join(", "),
                values.len() + 1
            );
            
            let mut query_builder = sqlx::query(&query);
            for value in values {
                query_builder = query_builder.bind(value);
            }
            query_builder = query_builder.bind(name);
            
            query_builder.execute(pool).await?;
            info!("Updated enriched data for: {}", name);
        }
    }

    Ok(())
}

pub async fn update_all_bios(pool: &PgPool) -> Result<()> {
    let rows = sqlx::query("SELECT name FROM users WHERE biography IS NULL OR biography = ''")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let name: String = row.get("name");
        if let Err(e) = update_bio_from_wikipedia(pool, &name).await {
            error!("Failed to update bio for {}: {}", name, e);
        }
        // Rate limit Wikipedia requests
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}