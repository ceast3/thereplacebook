use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct ForbesResponse {
    billionaires: Vec<Billionaire>,
}

#[derive(Debug, Deserialize)]
struct Billionaire {
    name: String,
    #[serde(rename = "netWorth")]
    net_worth: f64,
    source: String,
    age: Option<u32>,
    country: String,
    industry: String,
    bio: Option<String>,
}

pub async fn update_billionaire_data(pool: &PgPool) -> Result<()> {
    println!("Fetching latest billionaire data...");
    
    // Fetch from Forbes Real Time Billionaires API
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.forbes.com/forbesapi/person/rtb/0/position/true.json")
        .header("User-Agent", "Mozilla/5.0 (compatible; billionaire-updater)")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch Forbes data: {}", response.status()));
    }

    let forbes_data: ForbesResponse = response.json().await?;
    
    println!("Found {} billionaires", forbes_data.billionaires.len());

    for billionaire in forbes_data.billionaires {
        update_billionaire_record(pool, &billionaire).await?;
    }

    println!("Updated billionaire data successfully");
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
        .bind(&billionaire.source)
        .bind(&billionaire.bio)
        .bind(&billionaire.name)
        .execute(pool)
        .await?;
        
        println!("Updated: {} - {}", billionaire.name, net_worth_str);
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
        .bind(&billionaire.source)
        .bind(&billionaire.bio)
        .bind(1200.0) // Default rating
        .execute(pool)
        .await?;
        
        println!("Added new billionaire: {} - {}", billionaire.name, net_worth_str);
    }

    Ok(())
}

pub async fn update_bio_from_wikipedia(pool: &PgPool, name: &str) -> Result<()> {
    let client = reqwest::Client::new();
    
    // Search Wikipedia for the person
    let search_url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        name.replace(" ", "_")
    );
    
    let response = client
        .get(&search_url)
        .header("User-Agent", "Mozilla/5.0 (compatible; billionaire-updater)")
        .send()
        .await?;

    if response.status().is_success() {
        let wiki_data: serde_json::Value = response.json().await?;
        
        if let Some(extract) = wiki_data.get("extract").and_then(|e| e.as_str()) {
            // Update biography in database
            sqlx::query("UPDATE users SET biography = $1 WHERE name = $2")
                .bind(extract)
                .bind(name)
                .execute(pool)
                .await?;
                
            println!("Updated biography for: {}", name);
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
            println!("Failed to update bio for {}: {}", name, e);
        }
        // Rate limit Wikipedia requests
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}