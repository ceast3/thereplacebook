//! Data source module for aggregating billionaire information from multiple APIs.
//!
//! This module provides a trait-based architecture for implementing various
//! data sources and a manager to coordinate fetching from all sources.

pub mod wikidata;
pub mod wikipedia;
pub mod opencorporates;

use crate::errors::Result;
use crate::models::Billionaire;
use async_trait::async_trait;

/// Trait that all data sources must implement.
/// 
/// This allows for a plugin-like architecture where new data sources
/// can be easily added without modifying existing code.
#[async_trait]
pub trait DataSource {
    /// Fetches a list of billionaires from this data source.
    /// 
    /// # Arguments
    /// * `limit` - Optional limit on the number of results to return
    /// 
    /// # Returns
    /// * `Result<Vec<Billionaire>>` - List of billionaires or error
    async fn fetch_billionaires(&self, limit: Option<usize>) -> Result<Vec<Billionaire>>;
    
    /// Fetches detailed information about a specific person.
    /// 
    /// # Arguments
    /// * `name` - Name of the person to search for
    /// 
    /// # Returns
    /// * `Result<Option<Billionaire>>` - Person details if found, None if not found
    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>>;
    
    /// Returns the name of this data source for logging and debugging.
    fn name(&self) -> &'static str;
}

/// Manages multiple data sources and coordinates data fetching.
/// 
/// The DataSourceManager is responsible for:
/// - Initializing all available data sources
/// - Fetching data from multiple sources in parallel
/// - Deduplicating results by billionaire name
/// - Handling errors gracefully from individual sources
pub struct DataSourceManager {
    sources: Vec<Box<dyn DataSource + Send + Sync>>,
}

impl DataSourceManager {
    /// Creates a new DataSourceManager with all available data sources.
    /// 
    /// # Example
    /// ```
    /// let manager = DataSourceManager::new();
    /// let billionaires = manager.fetch_from_all_sources(Some(100)).await;
    /// ```
    pub fn new() -> Self {
        let mut sources: Vec<Box<dyn DataSource + Send + Sync>> = Vec::new();
        sources.push(Box::new(wikidata::WikidataSource::new()));
        sources.push(Box::new(wikipedia::WikipediaSource::new()));
        sources.push(Box::new(opencorporates::OpenCorporatesSource::new()));
        
        Self { sources }
    }

    /// Fetches billionaire data from all configured sources.
    /// 
    /// This method:
    /// 1. Queries each data source (errors are logged but don't stop the process)
    /// 2. Aggregates all results
    /// 3. Deduplicates by name (keeping the first occurrence)
    /// 4. Returns the combined list
    /// 
    /// # Arguments
    /// * `limit` - Optional limit to pass to each data source
    /// 
    /// # Returns
    /// * `Vec<Billionaire>` - Deduplicated list of billionaires from all sources
    pub async fn fetch_from_all_sources(&self, limit: Option<usize>) -> Vec<Billionaire> {
        let mut all_billionaires = Vec::new();
        
        // Fetch from each source sequentially (could be parallelized with futures::join_all)
        for source in &self.sources {
            match source.fetch_billionaires(limit).await {
                Ok(mut billionaires) => {
                    tracing::info!("Fetched {} billionaires from {}", billionaires.len(), source.name());
                    all_billionaires.append(&mut billionaires);
                }
                Err(e) => {
                    // Log error but continue with other sources
                    tracing::error!("Failed to fetch from {}: {}", source.name(), e);
                }
            }
        }
        
        // Deduplicate by name - could be improved with fuzzy matching
        all_billionaires.sort_by(|a, b| a.name.cmp(&b.name));
        all_billionaires.dedup_by(|a, b| a.name == b.name);
        
        all_billionaires
    }

    /// Enriches person data by searching across all data sources.
    /// 
    /// Searches for a person by name across all configured data sources
    /// and returns the first match found. This is useful for getting
    /// additional details about a specific billionaire.
    /// 
    /// # Arguments
    /// * `name` - Name of the person to search for
    /// 
    /// # Returns
    /// * `Option<Billionaire>` - First matching person found, or None
    /// 
    /// # Example
    /// ```
    /// if let Some(enriched) = manager.enrich_person_data("Elon Musk").await {
    ///     println!("Found bio: {:?}", enriched.bio);
    /// }
    /// ```
    pub async fn enrich_person_data(&self, name: &str) -> Option<Billionaire> {
        for source in &self.sources {
            if let Ok(Some(person)) = source.fetch_person_details(name).await {
                return Some(person);
            }
        }
        None
    }
}