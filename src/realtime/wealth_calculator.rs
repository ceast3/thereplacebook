//! Wealth calculation engine for real-time net worth updates.
//!
//! This module handles complex wealth calculations including:
//! - Stock portfolio valuations
//! - Currency conversions
//! - Private company valuations
//! - Real estate and other assets

use super::{RealtimeState, UpdateMessage, StockPosition};
use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, warn};
use serde::{Serialize, Deserialize};
use dashmap::DashMap;

/// Calculates real-time wealth changes
pub struct WealthCalculator {
    pool: PgPool,
    state: Arc<RealtimeState>,
    /// Exchange rates cache
    exchange_rates: DashMap<String, f64>,
}

/// Represents different asset types for wealth calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    PublicStock { symbol: String, shares: f64 },
    PrivateEquity { company: String, stake: f64, valuation: f64 },
    RealEstate { properties: Vec<Property> },
    Cryptocurrency { holdings: Vec<CryptoHolding> },
    Other { description: String, value: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub location: String,
    pub value: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoHolding {
    pub symbol: String,
    pub amount: f64,
    pub price_usd: f64,
}

/// Complete wealth profile for a billionaire
#[derive(Debug, Clone)]
pub struct WealthProfile {
    pub name: String,
    pub assets: Vec<AssetType>,
    pub total_net_worth: f64,
    pub currency: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl WealthCalculator {
    /// Creates a new wealth calculator
    pub fn new(pool: PgPool, state: Arc<RealtimeState>) -> Self {
        let mut calculator = Self {
            pool,
            state,
            exchange_rates: DashMap::new(),
        };
        
        // Initialize with some default exchange rates
        calculator.exchange_rates.insert("USD".to_string(), 1.0);
        calculator.exchange_rates.insert("EUR".to_string(), 1.08);
        calculator.exchange_rates.insert("GBP".to_string(), 1.27);
        calculator.exchange_rates.insert("JPY".to_string(), 0.0067);
        calculator.exchange_rates.insert("CNY".to_string(), 0.14);
        
        calculator
    }

    /// Calculates total net worth for a billionaire
    pub async fn calculate_net_worth(&self, name: &str) -> Result<f64> {
        let profile = self.get_wealth_profile(name).await?;
        let mut total_usd = 0.0;
        
        for asset in &profile.assets {
            match asset {
                AssetType::PublicStock { symbol, shares } => {
                    if let Some(holdings) = self.state.holdings.read().await.holdings.get(name) {
                        if let Some(position) = holdings.iter().find(|p| &p.symbol == symbol) {
                            total_usd += position.value;
                        }
                    }
                }
                AssetType::PrivateEquity { stake, valuation, .. } => {
                    total_usd += stake * valuation;
                }
                AssetType::RealEstate { properties } => {
                    for property in properties {
                        let value_usd = self.convert_to_usd(property.value, &property.currency);
                        total_usd += value_usd;
                    }
                }
                AssetType::Cryptocurrency { holdings } => {
                    for holding in holdings {
                        total_usd += holding.amount * holding.price_usd;
                    }
                }
                AssetType::Other { value, .. } => {
                    total_usd += value;
                }
            }
        }
        
        Ok(total_usd / 1_000_000_000.0) // Convert to billions
    }

    /// Gets the complete wealth profile for a billionaire
    async fn get_wealth_profile(&self, name: &str) -> Result<WealthProfile> {
        // In production, this would query the database for detailed asset information
        // For now, we'll create sample profiles
        
        let assets = match name {
            "Elon Musk" => vec![
                AssetType::PublicStock {
                    symbol: "TSLA".to_string(),
                    shares: 411_000_000.0,
                },
                AssetType::PrivateEquity {
                    company: "SpaceX".to_string(),
                    stake: 0.42,
                    valuation: 150_000_000_000.0,
                },
                AssetType::PrivateEquity {
                    company: "Neuralink".to_string(),
                    stake: 0.90,
                    valuation: 5_000_000_000.0,
                },
                AssetType::PrivateEquity {
                    company: "The Boring Company".to_string(),
                    stake: 0.90,
                    valuation: 5_700_000_000.0,
                },
            ],
            "Jeff Bezos" => vec![
                AssetType::PublicStock {
                    symbol: "AMZN".to_string(),
                    shares: 990_000_000.0,
                },
                AssetType::PrivateEquity {
                    company: "Blue Origin".to_string(),
                    stake: 1.0,
                    valuation: 12_000_000_000.0,
                },
                AssetType::RealEstate {
                    properties: vec![
                        Property {
                            name: "Beverly Hills Mansion".to_string(),
                            location: "California, USA".to_string(),
                            value: 165_000_000.0,
                            currency: "USD".to_string(),
                        },
                        Property {
                            name: "Washington D.C. Mansion".to_string(),
                            location: "Washington, USA".to_string(),
                            value: 23_000_000.0,
                            currency: "USD".to_string(),
                        },
                    ],
                },
            ],
            _ => vec![
                AssetType::Other {
                    description: "Diversified holdings".to_string(),
                    value: 1_000_000_000.0,
                },
            ],
        };
        
        Ok(WealthProfile {
            name: name.to_string(),
            assets,
            total_net_worth: 0.0, // Will be calculated
            currency: "USD".to_string(),
            last_updated: chrono::Utc::now(),
        })
    }

    /// Converts an amount to USD using cached exchange rates
    fn convert_to_usd(&self, amount: f64, currency: &str) -> f64 {
        if currency == "USD" {
            return amount;
        }
        
        if let Some(rate) = self.exchange_rates.get(currency) {
            amount * *rate
        } else {
            warn!("Unknown currency: {}, treating as USD", currency);
            amount
        }
    }

    /// Updates exchange rates from external API
    pub async fn update_exchange_rates(&self) -> Result<()> {
        // In production, fetch from a currency API
        // For now, we'll simulate with small fluctuations
        
        for mut entry in self.exchange_rates.iter_mut() {
            let (currency, rate) = entry.pair_mut();
            if currency != "USD" {
                // Simulate small fluctuations (±0.5%)
                let change = (rand::random::<f64>() - 0.5) * 0.01;
                *rate *= 1.0 + change;
            }
        }
        
        info!("Updated {} exchange rates", self.exchange_rates.len());
        Ok(())
    }

    /// Monitors for significant wealth changes and broadcasts updates
    pub async fn monitor_wealth_changes(self: Arc<Self>) {
        let mut wealth_cache: DashMap<String, f64> = DashMap::new();
        
        loop {
            // Check every minute for wealth changes
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            
            // Get list of billionaires from database
            if let Ok(billionaires) = self.get_top_billionaires(100).await {
                for billionaire in billionaires {
                    if let Ok(new_worth) = self.calculate_net_worth(&billionaire.name).await {
                        let previous_worth = wealth_cache.get(&billionaire.name)
                            .map(|v| *v)
                            .unwrap_or(billionaire.net_worth);
                        
                        let change = new_worth - previous_worth;
                        let change_percent = (change / previous_worth) * 100.0;
                        
                        // Notify if change is significant (> 1% or > $1B)
                        if change_percent.abs() > 1.0 || change.abs() > 1.0 {
                            let update = UpdateMessage::WealthUpdate {
                                name: billionaire.name.clone(),
                                previous_net_worth: previous_worth,
                                new_net_worth: new_worth,
                                change_percentage: change_percent,
                                reason: self.determine_change_reason(&billionaire.name, change_percent).await,
                            };
                            
                            self.state.broadcast_update(update).await;
                        }
                        
                        wealth_cache.insert(billionaire.name, new_worth);
                    }
                }
            }
        }
    }

    /// Gets top billionaires from database
    async fn get_top_billionaires(&self, limit: usize) -> Result<Vec<Billionaire>> {
        let rows = sqlx::query(
            "SELECT name, net_worth, source_of_wealth, industry, nationality 
             FROM users 
             ORDER BY rating DESC 
             LIMIT $1"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|row| {
            use sqlx::Row;
            Billionaire {
                name: row.get("name"),
                net_worth: self.parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
                source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
                industry: row.get::<Option<String>, _>("industry").unwrap_or_default(),
                country: row.get::<Option<String>, _>("nationality").unwrap_or_default(),
                age: None,
                bio: None,
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
            }
        }).collect())
    }

    /// Parses net worth string to numeric value
    fn parse_net_worth(&self, net_worth_str: &str) -> f64 {
        net_worth_str
            .replace("$", "")
            .replace("B", "")
            .parse::<f64>()
            .unwrap_or(0.0)
    }

    /// Determines the reason for a wealth change
    async fn determine_change_reason(&self, name: &str, change_percent: f64) -> String {
        // In production, this would analyze news, stock movements, etc.
        if change_percent > 0.0 {
            format!("Portfolio gains of {:.1}%", change_percent)
        } else {
            format!("Portfolio losses of {:.1}%", change_percent.abs())
        }
    }
}

// Add this to avoid compilation error for rand
use rand::Rng as _;