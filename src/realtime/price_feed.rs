//! Price feed integration for real-time stock data.
//!
//! This module integrates with various free stock data APIs to fetch
//! real-time price updates that affect billionaire net worth.

use super::{RealtimeState, UpdateMessage, StockPosition};
use crate::errors::{Result, AppError};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};
use dashmap::DashMap;

/// Manages connections to various price feed sources
pub struct PriceFeedManager {
    state: Arc<RealtimeState>,
    /// Cache of latest stock prices
    price_cache: DashMap<String, StockPrice>,
}

/// Represents current stock price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Yahoo Finance API response structure (unofficial)
#[derive(Debug, Deserialize)]
struct YahooQuote {
    #[serde(rename = "regularMarketPrice")]
    price: f64,
    #[serde(rename = "regularMarketChange")]
    change: f64,
    #[serde(rename = "regularMarketChangePercent")]
    change_percent: f64,
}

/// Alpha Vantage API response (free tier)
#[derive(Debug, Deserialize)]
struct AlphaVantageQuote {
    #[serde(rename = "Global Quote")]
    quote: AlphaVantageGlobalQuote,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageGlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

impl PriceFeedManager {
    /// Creates a new price feed manager
    pub fn new(state: Arc<RealtimeState>) -> Self {
        Self {
            state,
            price_cache: DashMap::new(),
        }
    }

    /// Starts the price feed monitoring loop
    pub async fn start_monitoring(self: Arc<Self>) {
        info!("Starting price feed monitoring");
        
        // Update prices every 30 seconds (respecting rate limits)
        let mut interval = interval(Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            // Get all unique stock symbols we need to track
            let symbols = self.get_tracked_symbols().await;
            
            if !symbols.is_empty() {
                info!("Updating prices for {} symbols", symbols.len());
                
                // Fetch prices in batches to respect rate limits
                for chunk in symbols.chunks(5) {
                    for symbol in chunk {
                        if let Err(e) = self.update_stock_price(symbol).await {
                            warn!("Failed to update price for {}: {}", symbol, e);
                        }
                    }
                    // Rate limit: wait between batches
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                
                // Calculate wealth updates based on new prices
                self.calculate_wealth_updates().await;
            }
        }
    }

    /// Gets all stock symbols that need tracking
    async fn get_tracked_symbols(&self) -> Vec<String> {
        let holdings = self.state.holdings.read().await;
        let mut symbols = std::collections::HashSet::new();
        
        // Get symbols from billionaire holdings
        for entry in holdings.holdings.iter() {
            for position in entry.value() {
                symbols.insert(position.symbol.clone());
            }
        }
        
        // Get symbols from client subscriptions
        for sub in self.state.subscriptions.iter() {
            for symbol in &sub.value().symbols {
                symbols.insert(symbol.clone());
            }
        }
        
        symbols.into_iter().collect()
    }

    /// Updates the price for a single stock symbol
    async fn update_stock_price(&self, symbol: &str) -> Result<()> {
        // Try multiple data sources in order of preference
        match self.fetch_yahoo_finance(symbol).await {
            Ok(price) => {
                self.price_cache.insert(symbol.to_string(), price.clone());
                self.broadcast_stock_update(price).await;
                return Ok(());
            }
            Err(e) => {
                warn!("Yahoo Finance failed for {}: {}", symbol, e);
            }
        }
        
        // Fallback to Alpha Vantage (requires free API key)
        match self.fetch_alpha_vantage(symbol).await {
            Ok(price) => {
                self.price_cache.insert(symbol.to_string(), price.clone());
                self.broadcast_stock_update(price).await;
                return Ok(());
            }
            Err(e) => {
                warn!("Alpha Vantage failed for {}: {}", symbol, e);
            }
        }
        
        Err(AppError::DataSourceUnavailable(format!(
            "All price feeds failed for {}",
            symbol
        )))
    }

    /// Fetches price from Yahoo Finance (unofficial API)
    async fn fetch_yahoo_finance(&self, symbol: &str) -> Result<StockPrice> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "TheReplacebook/1.0")
            .send()
            .await?;
        
        let data: serde_json::Value = response.json().await?;
        
        // Parse the complex Yahoo Finance response
        let price = data["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| AppError::Validation("Invalid price data".to_string()))?;
        
        let previous_close = data["chart"]["result"][0]["meta"]["previousClose"]
            .as_f64()
            .unwrap_or(price);
        
        let change = price - previous_close;
        let change_percent = (change / previous_close) * 100.0;
        
        Ok(StockPrice {
            symbol: symbol.to_string(),
            price,
            change,
            change_percent,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Fetches price from Alpha Vantage (requires API key)
    async fn fetch_alpha_vantage(&self, symbol: &str) -> Result<StockPrice> {
        // Note: In production, store API key in environment variable
        let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
            .unwrap_or_else(|_| "demo".to_string());
        
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol, api_key
        );
        
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let data: AlphaVantageQuote = response.json().await?;
        
        Ok(StockPrice {
            symbol: symbol.to_string(),
            price: data.quote.price.parse().unwrap_or(0.0),
            change: data.quote.change.parse().unwrap_or(0.0),
            change_percent: data.quote.change_percent.trim_end_matches('%').parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Broadcasts a stock price update to connected clients
    async fn broadcast_stock_update(&self, price: StockPrice) {
        // Find affected billionaires
        let holdings = self.state.holdings.read().await;
        let mut affected_billionaires = Vec::new();
        
        for entry in holdings.holdings.iter() {
            let (name, positions) = entry.pair();
            if positions.iter().any(|p| p.symbol == price.symbol) {
                affected_billionaires.push(name.clone());
            }
        }
        
        let update = UpdateMessage::StockUpdate {
            symbol: price.symbol,
            price: price.price,
            change: price.change,
            affected_billionaires,
        };
        
        self.state.broadcast_update(update).await;
    }

    /// Calculates wealth updates based on current stock prices
    async fn calculate_wealth_updates(&self) {
        let mut holdings = self.state.holdings.write().await;
        
        for entry in holdings.holdings.iter() {
            let (name, positions) = entry.pair();
            let mut total_change = 0.0;
            let mut previous_total = 0.0;
            let mut new_total = 0.0;
            
            for position in positions.iter() {
                if let Some(current_price) = self.price_cache.get(&position.symbol) {
                    let old_value = position.shares * position.last_price;
                    let new_value = position.shares * current_price.price;
                    
                    previous_total += old_value;
                    new_total += new_value;
                    total_change += new_value - old_value;
                }
            }
            
            if total_change.abs() > 1_000_000.0 { // Only notify for changes > $1M
                let change_percentage = (total_change / previous_total) * 100.0;
                
                let update = UpdateMessage::WealthUpdate {
                    name: name.clone(),
                    previous_net_worth: previous_total / 1_000_000_000.0, // Convert to billions
                    new_net_worth: new_total / 1_000_000_000.0,
                    change_percentage,
                    reason: "Stock portfolio value change".to_string(),
                };
                
                self.state.broadcast_update(update).await;
            }
        }
    }
}

/// Loads sample billionaire holdings for demonstration
pub async fn load_sample_holdings(state: Arc<RealtimeState>) {
    let mut holdings = state.holdings.write().await;
    
    // Sample data - in production, load from database
    holdings.holdings.insert(
        "Elon Musk".to_string(),
        vec![
            StockPosition {
                symbol: "TSLA".to_string(),
                shares: 411_000_000.0, // Approximate
                last_price: 250.0,
                value: 102_750_000_000.0,
            },
        ],
    );
    
    holdings.holdings.insert(
        "Jeff Bezos".to_string(),
        vec![
            StockPosition {
                symbol: "AMZN".to_string(),
                shares: 990_000_000.0, // Approximate
                last_price: 150.0,
                value: 148_500_000_000.0,
            },
        ],
    );
    
    holdings.holdings.insert(
        "Warren Buffett".to_string(),
        vec![
            StockPosition {
                symbol: "BRK-A".to_string(),
                shares: 230_000.0, // Approximate Class A shares
                last_price: 550_000.0,
                value: 126_500_000_000.0,
            },
        ],
    );
    
    info!("Loaded sample holdings for {} billionaires", holdings.holdings.len());
}