pub mod wikidata;
pub mod wikipedia;
pub mod opencorporates;

use crate::errors::Result;
use crate::models::Billionaire;
use async_trait::async_trait;

#[async_trait]
pub trait DataSource {
    async fn fetch_billionaires(&self, limit: Option<usize>) -> Result<Vec<Billionaire>>;
    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>>;
    fn name(&self) -> &'static str;
}

pub struct DataSourceManager {
    sources: Vec<Box<dyn DataSource + Send + Sync>>,
}

impl DataSourceManager {
    pub fn new() -> Self {
        let mut sources: Vec<Box<dyn DataSource + Send + Sync>> = Vec::new();
        sources.push(Box::new(wikidata::WikidataSource::new()));
        sources.push(Box::new(wikipedia::WikipediaSource::new()));
        sources.push(Box::new(opencorporates::OpenCorporatesSource::new()));
        
        Self { sources }
    }

    pub async fn fetch_from_all_sources(&self, limit: Option<usize>) -> Vec<Billionaire> {
        let mut all_billionaires = Vec::new();
        
        for source in &self.sources {
            match source.fetch_billionaires(limit).await {
                Ok(mut billionaires) => {
                    tracing::info!("Fetched {} billionaires from {}", billionaires.len(), source.name());
                    all_billionaires.append(&mut billionaires);
                }
                Err(e) => {
                    tracing::error!("Failed to fetch from {}: {}", source.name(), e);
                }
            }
        }
        
        // Deduplicate by name
        all_billionaires.sort_by(|a, b| a.name.cmp(&b.name));
        all_billionaires.dedup_by(|a, b| a.name == b.name);
        
        all_billionaires
    }

    pub async fn enrich_person_data(&self, name: &str) -> Option<Billionaire> {
        for source in &self.sources {
            if let Ok(Some(person)) = source.fetch_person_details(name).await {
                return Some(person);
            }
        }
        None
    }
}