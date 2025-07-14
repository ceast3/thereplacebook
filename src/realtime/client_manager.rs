//! Client connection management for real-time updates.

use super::{RealtimeState, UpdateMessage, Severity};
use std::sync::Arc;
use tracing::{info, warn};

/// Manages client connections and subscriptions
pub struct ClientManager {
    state: Arc<RealtimeState>,
}

impl ClientManager {
    /// Creates a new client manager
    pub fn new(state: Arc<RealtimeState>) -> Self {
        Self { state }
    }

    /// Gets the number of active connections
    pub fn active_connections(&self) -> usize {
        self.state.connections.len()
    }

    /// Gets the number of active subscriptions
    pub fn active_subscriptions(&self) -> usize {
        self.state.subscriptions.len()
    }

    /// Broadcasts a system message to all connected clients
    pub async fn broadcast_system_message(&self, message: String, severity: Severity) {
        let update = UpdateMessage::SystemUpdate { message, severity };
        self.state.broadcast_update(update).await;
    }

    /// Sends a message to a specific client
    pub async fn send_to_client(&self, client_id: &str, update: UpdateMessage) -> bool {
        if let Some(sender) = self.state.connections.get(client_id) {
            sender.send(update).await.is_ok()
        } else {
            false
        }
    }

    /// Disconnects a specific client
    pub fn disconnect_client(&self, client_id: &str) {
        self.state.connections.remove(client_id);
        self.state.subscriptions.remove(client_id);
        info!("Forcefully disconnected client: {}", client_id);
    }

    /// Gets subscription statistics
    pub fn get_subscription_stats(&self) -> SubscriptionStats {
        let mut stats = SubscriptionStats::default();
        
        for sub in self.state.subscriptions.iter() {
            let subscription = sub.value();
            stats.total_subscriptions += 1;
            
            if subscription.all_updates {
                stats.all_updates_count += 1;
            }
            
            stats.billionaire_subscriptions += subscription.billionaires.len();
            stats.industry_subscriptions += subscription.industries.len();
            stats.symbol_subscriptions += subscription.symbols.len();
        }
        
        stats
    }

    /// Monitors client health and removes stale connections
    pub async fn monitor_client_health(self: Arc<Self>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            let mut stale_clients = Vec::new();
            
            // Send ping to all clients and track failures
            for entry in self.state.connections.iter() {
                let (client_id, sender) = entry.pair();
                
                let ping = UpdateMessage::SystemUpdate {
                    message: "ping".to_string(),
                    severity: Severity::Info,
                };
                
                if sender.send(ping).await.is_err() {
                    stale_clients.push(client_id.clone());
                }
            }
            
            // Remove stale connections
            for client_id in stale_clients {
                warn!("Removing stale client: {}", client_id);
                self.disconnect_client(&client_id);
            }
            
            if self.active_connections() > 0 {
                info!(
                    "Client health check: {} active connections, {} subscriptions",
                    self.active_connections(),
                    self.active_subscriptions()
                );
            }
        }
    }
}

/// Statistics about active subscriptions
#[derive(Debug, Default)]
pub struct SubscriptionStats {
    pub total_subscriptions: usize,
    pub all_updates_count: usize,
    pub billionaire_subscriptions: usize,
    pub industry_subscriptions: usize,
    pub symbol_subscriptions: usize,
}

impl SubscriptionStats {
    /// Prints formatted statistics
    pub fn print_summary(&self) {
        info!("=== Subscription Statistics ===");
        info!("Total subscriptions: {}", self.total_subscriptions);
        info!("All updates subscriptions: {}", self.all_updates_count);
        info!("Billionaire subscriptions: {}", self.billionaire_subscriptions);
        info!("Industry subscriptions: {}", self.industry_subscriptions);
        info!("Symbol subscriptions: {}", self.symbol_subscriptions);
    }
}