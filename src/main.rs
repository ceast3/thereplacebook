mod updater;
mod errors;
mod models;
mod data_sources;
mod features;
mod realtime;
mod search;

use sqlx::PgPool;
use std::env;
use tracing_subscriber;
use features::{MatchingEngine, Analytics};
use std::sync::Arc;
use realtime::{RealtimeState, websocket, price_feed, wealth_calculator, client_manager};
use search::{SearchEngine, SearchQuery, SearchField};

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
        println!("  websocket-server    - Start real-time WebSocket server");
        println!("  search <query>      - Full-text search in biographies");
        println!("  search-init         - Initialize search indexes");
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
        "websocket-server" => {
            start_websocket_server(pool).await?;
        }
        "search" => {
            if args.len() < 3 {
                println!("Usage: {} search <query>", args[0]);
                return Ok(());
            }
            let search_engine = SearchEngine::new(pool);
            let query = SearchQuery {
                query: Some(args[2..].join(" ")),
                ..Default::default()
            };
            let results = search_engine.search(query).await?;
            
            println!("Found {} results:", results.len());
            for (i, result) in results.iter().enumerate().take(10) {
                println!("\n{}. {} (${:.1}B) - {}", 
                    i + 1, 
                    result.billionaire.name, 
                    result.billionaire.net_worth,
                    result.billionaire.industry
                );
                println!("   Relevance: {:.2}", result.relevance_score);
                for snippet in &result.highlight_snippets {
                    println!("   {}: {}", snippet.field, snippet.snippet);
                }
            }
        }
        "search-init" => {
            println!("Initializing search indexes...");
            let search_engine = SearchEngine::new(pool);
            search_engine.create_search_indexes().await?;
            println!("Search indexes created successfully!");
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use one of the available commands listed above");
        }
    }

    Ok(())
}

/// Starts the WebSocket server for real-time updates
async fn start_websocket_server(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use axum::{Router, routing::get};
    use tower_http::cors::{CorsLayer, Any};
    
    println!("Starting WebSocket server on port 3000...");
    
    // Create shared state
    let state = Arc::new(RealtimeState::new());
    
    // Load sample holdings data
    price_feed::load_sample_holdings(state.clone()).await;
    
    // Start background tasks
    let price_manager = Arc::new(price_feed::PriceFeedManager::new(state.clone()));
    let wealth_calc = Arc::new(wealth_calculator::WealthCalculator::new(pool, state.clone()));
    let client_mgr = Arc::new(client_manager::ClientManager::new(state.clone()));
    
    // Spawn background tasks
    tokio::spawn(price_manager.start_monitoring());
    tokio::spawn(wealth_calc.monitor_wealth_changes());
    tokio::spawn(client_mgr.monitor_client_health());
    
    // Create WebSocket router
    let ws_router = websocket::create_websocket_router();
    
    // Create main router with CORS
    let app = Router::new()
        .nest("/", ws_router)
        .route("/health", get(|| async { "OK" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("WebSocket server listening on ws://localhost:3000/ws");
    println!("Health check available at http://localhost:3000/health");
    println!("Press Ctrl+C to stop the server");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
