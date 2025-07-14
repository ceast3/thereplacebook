mod updater;
mod errors;
mod models;
mod data_sources;
mod features;

use sqlx::PgPool;
use std::env;
use tracing_subscriber;
use features::{MatchingEngine, Analytics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Commands:");
        println!("  update-networth     - Update billionaire net worth data");
        println!("  update-bios         - Update billionaire biographies");
        println!("  update-all          - Update both net worth and biographies");
        println!("  top-100             - Populate database with top 100 billionaires");
        println!("  match-industry <industry> - Find billionaires by industry");
        println!("  match-country <country>   - Find billionaires by country");
        println!("  analytics           - Show industry and country distributions");
        println!("  wealth-tiers        - Show wealth tier distribution");
        return Ok(());
    }

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/thereplacebook".to_string());
    
    let pool = PgPool::connect(&database_url).await?;

    match args[1].as_str() {
        "update-networth" => {
            updater::update_billionaire_data(&pool).await?;
        }
        "update-bios" => {
            updater::update_all_bios(&pool).await?;
        }
        "update-all" => {
            updater::update_billionaire_data(&pool).await?;
            updater::update_all_bios(&pool).await?;
        }
        "top-100" => {
            updater::populate_top_100(&pool).await?;
        }
        "match-industry" => {
            if args.len() < 3 {
                println!("Usage: {} match-industry <industry>", args[0]);
                return Ok(());
            }
            let matching_engine = MatchingEngine::new(pool);
            let matches = matching_engine.find_matches_by_industry(&args[2], Some(10)).await?;
            println!("Found {} billionaires in {} industry:", matches.len(), args[2]);
            for billionaire in matches {
                println!("- {} (${:.1}B) from {}", billionaire.name, billionaire.net_worth, billionaire.country);
            }
        }
        "match-country" => {
            if args.len() < 3 {
                println!("Usage: {} match-country <country>", args[0]);
                return Ok(());
            }
            let matching_engine = MatchingEngine::new(pool);
            let matches = matching_engine.find_matches_by_country(&args[2]).await?;
            println!("Found {} billionaires from {}:", matches.len(), args[2]);
            for billionaire in matches {
                println!("- {} (${:.1}B) - {}", billionaire.name, billionaire.net_worth, billionaire.industry);
            }
        }
        "analytics" => {
            let analytics = Analytics::new(pool);
            let industry_dist = analytics.get_industry_distribution().await?;
            let country_dist = analytics.get_country_distribution().await?;
            
            println!("Industry Distribution:");
            for (industry, count) in industry_dist {
                println!("  {}: {}", industry, count);
            }
            
            println!("\\nCountry Distribution:");
            for (country, count) in country_dist {
                println!("  {}: {}", country, count);
            }
        }
        "wealth-tiers" => {
            let analytics = Analytics::new(pool);
            let tiers = analytics.get_wealth_tiers().await?;
            
            println!("Wealth Tier Distribution:");
            for (tier, count) in tiers {
                println!("  {}: {} billionaires", tier, count);
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use one of the available commands listed above");
        }
    }

    Ok(())
}
