//! Real-time data updates module for live billionaire wealth tracking.
//!
//! This module provides WebSocket-based real-time updates for:
//! - Live stock price changes affecting net worth
//! - Currency exchange rate fluctuations
//! - Breaking news affecting wealth
//! - Portfolio value updates

pub mod websocket;
pub mod price_feed;
pub mod wealth_calculator;
pub mod client_manager;

use crate::errors::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// Real-time update message sent to clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UpdateMessage {
    /// Net worth update for a specific billionaire
    WealthUpdate {
        name: String,
        previous_net_worth: f64,
        new_net_worth: f64,
        change_percentage: f64,
        reason: String,
    },
    /// Stock price update affecting multiple billionaires
    StockUpdate {
        symbol: String,
        price: f64,
        change: f64,
        affected_billionaires: Vec<String>,
    },
    /// Breaking news that might affect wealth
    NewsUpdate {
        headline: String,
        summary: String,
        affected_billionaires: Vec<String>,
        impact: ImpactLevel,
    },
    /// System status update
    SystemUpdate {
        message: String,
        severity: Severity,
    },
}

/// Impact level of news on billionaire wealth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
    Unknown,
}

/// Severity level for system messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Subscription preferences for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Subscribe to specific billionaires by name
    pub billionaires: Vec<String>,
    /// Subscribe to specific industries
    pub industries: Vec<String>,
    /// Subscribe to specific stock symbols
    pub symbols: Vec<String>,
    /// Subscribe to all updates (caution: high volume)
    pub all_updates: bool,
}

/// Global state for real-time updates
pub struct RealtimeState {
    /// Active WebSocket connections mapped by client ID
    pub connections: DashMap<String, tokio::sync::mpsc::Sender<UpdateMessage>>,
    /// Client subscriptions
    pub subscriptions: DashMap<String, Subscription>,
    /// Billionaire stock holdings for wealth calculations
    pub holdings: Arc<RwLock<BillionaireHoldings>>,
}

/// Maps billionaires to their stock holdings
#[derive(Debug, Clone, Default)]
pub struct BillionaireHoldings {
    /// Map of billionaire name to their stock positions
    pub holdings: DashMap<String, Vec<StockPosition>>,
}

/// Represents a stock position held by a billionaire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPosition {
    pub symbol: String,
    pub shares: f64,
    pub last_price: f64,
    pub value: f64,
}

impl RealtimeState {
    /// Creates a new real-time state manager
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            subscriptions: DashMap::new(),
            holdings: Arc::new(RwLock::new(BillionaireHoldings::default())),
        }
    }

    /// Broadcasts an update to all subscribed clients
    pub async fn broadcast_update(&self, update: UpdateMessage) {
        let affected_names = match &update {
            UpdateMessage::WealthUpdate { name, .. } => vec![name.clone()],
            UpdateMessage::StockUpdate { affected_billionaires, .. } => affected_billionaires.clone(),
            UpdateMessage::NewsUpdate { affected_billionaires, .. } => affected_billionaires.clone(),
            UpdateMessage::SystemUpdate { .. } => vec![],
        };

        // Send to subscribed clients
        for entry in self.connections.iter() {
            let (client_id, sender) = entry.pair();
            
            if let Some(subscription) = self.subscriptions.get(client_id) {
                if self.should_send_update(&subscription, &update, &affected_names) {
                    let _ = sender.send(update.clone()).await;
                }
            }
        }
    }

    /// Determines if an update should be sent to a client based on their subscription
    fn should_send_update(
        &self,
        subscription: &Subscription,
        update: &UpdateMessage,
        affected_names: &[String],
    ) -> bool {
        if subscription.all_updates {
            return true;
        }

        // Check if any affected billionaires are in the subscription
        for name in affected_names {
            if subscription.billionaires.iter().any(|n| n == name) {
                return true;
            }
        }

        // Check stock symbols for stock updates
        if let UpdateMessage::StockUpdate { symbol, .. } = update {
            if subscription.symbols.iter().any(|s| s == symbol) {
                return true;
            }
        }

        // Always send system updates
        matches!(update, UpdateMessage::SystemUpdate { .. })
    }
}