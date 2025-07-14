use crate::data_sources::DataSource;
use crate::errors::Result;
use crate::models::{Billionaire, OpenCorpResponse};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

pub struct OpenCorporatesSource {
    client: Client,
    base_url: String,
}

impl OpenCorporatesSource {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("thereplacebook/1.0 (https://github.com/user/thereplacebook)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://api.opencorporates.com/v0.4".to_string(),
        }
    }

    async fn search_officers(&self, name: &str) -> Result<Option<OpenCorpResponse>> {
        let url = format!("{}/officers/search", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .query(&[("q", name), ("format", "json")])
            .send()
            .await?;

        if response.status().is_success() {
            let data: OpenCorpResponse = response.json().await?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl DataSource for OpenCorporatesSource {
    async fn fetch_billionaires(&self, _limit: Option<usize>) -> Result<Vec<Billionaire>> {
        // OpenCorporates doesn't have a direct billionaire endpoint
        // This would be used to enrich existing data with corporate information
        Ok(Vec::new())
    }

    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>> {
        tokio::time::sleep(Duration::from_millis(200)).await; // Rate limiting
        
        if let Some(_corp_data) = self.search_officers(name).await? {
            // Extract corporate information and create a minimal billionaire record
            // This is a simplified implementation - in practice you'd extract
            // company affiliations, roles, etc.
            
            let billionaire = Billionaire {
                name: name.to_string(),
                net_worth: 1.0, // Would need to be enriched from other sources
                source_of_wealth: "Corporate Leadership".to_string(),
                age: None,
                country: "Unknown".to_string(),
                industry: "Business".to_string(),
                bio: None,
                company: None, // Would extract from corp_data
                philanthropy: None,
                notable_achievements: None,
                website: None,
                twitter_handle: None,
                linkedin_profile: None,
                quote: None,
                birthdate: None,
                image_url: None,
                parental_wealth: None,
            };
            
            Ok(Some(billionaire))
        } else {
            Ok(None)
        }
    }

    fn name(&self) -> &'static str {
        "OpenCorporates"
    }
}