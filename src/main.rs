mod updater;

use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Commands:");
        println!("  update-networth  - Update billionaire net worth data");
        println!("  update-bios      - Update billionaire biographies");
        println!("  update-all       - Update both net worth and biographies");
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
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'update-networth', 'update-bios', or 'update-all'");
        }
    }

    Ok(())
}
