use crate::data_sources::DataSource;
use crate::errors::Result;
use crate::models::{Billionaire, WikipediaSummary};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

pub struct WikipediaSource {
    client: Client,
    base_url: String,
}

impl WikipediaSource {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("thereplacebook/1.0 (https://github.com/user/thereplacebook)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://en.wikipedia.org/api/rest_v1/page/summary".to_string(),
        }
    }

    async fn fetch_summary(&self, name: &str) -> Result<Option<WikipediaSummary>> {
        let url = format!("{}/{}", self.base_url, name.replace(" ", "_"));
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let summary: WikipediaSummary = response.json().await?;
            Ok(Some(summary))
        } else {
            Ok(None)
        }
    }

    fn extract_wealth_info(extract: &str) -> Option<f64> {
        // Simple pattern matching for wealth mentions
        let wealth_patterns = [
            r"net worth.*?\$([0-9.]+)\s*billion",
            r"worth.*?\$([0-9.]+)\s*billion",
            r"\$([0-9.]+)\s*billion.*?net worth",
            r"fortune.*?\$([0-9.]+)\s*billion",
        ];
        
        for pattern in &wealth_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(captures) = re.captures(extract) {
                    if let Some(amount) = captures.get(1) {
                        if let Ok(value) = amount.as_str().parse::<f64>() {
                            return Some(value);
                        }
                    }
                }
            }
        }
        
        None
    }

    fn extract_industry(extract: &str) -> String {
        let industry_keywords = [
            ("technology", "Technology"),
            ("software", "Technology"),
            ("internet", "Technology"),
            ("computer", "Technology"),
            ("automotive", "Automotive"),
            ("retail", "Retail"),
            ("fashion", "Fashion"),
            ("real estate", "Real Estate"),
            ("finance", "Finance"),
            ("investment", "Finance"),
            ("pharmaceutical", "Healthcare"),
            ("energy", "Energy"),
            ("oil", "Energy"),
            ("media", "Media"),
            ("entertainment", "Entertainment"),
        ];

        let extract_lower = extract.to_lowercase();
        for (keyword, industry) in &industry_keywords {
            if extract_lower.contains(keyword) {
                return industry.to_string();
            }
        }

        "Business".to_string()
    }
}

#[async_trait]
impl DataSource for WikipediaSource {
    async fn fetch_billionaires(&self, _limit: Option<usize>) -> Result<Vec<Billionaire>> {
        // Wikipedia doesn't have a direct billionaire list API
        // This would need to be combined with other sources
        // For now, return empty vector
        Ok(Vec::new())
    }

    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>> {
        tokio::time::sleep(Duration::from_millis(100)).await; // Rate limiting
        
        if let Some(summary) = self.fetch_summary(name).await? {
            let extract = summary.extract.unwrap_or_default();
            
            let billionaire = Billionaire {
                name: summary.title.clone(),
                net_worth: Self::extract_wealth_info(&extract).unwrap_or(1.0),
                source_of_wealth: "Various".to_string(),
                age: None,
                country: "Unknown".to_string(),
                industry: Self::extract_industry(&extract),
                bio: Some(extract),
                company: None,
                philanthropy: None,
                notable_achievements: None,
                website: None,
                twitter_handle: None,
                linkedin_profile: None,
                quote: None,
                birthdate: None,
                image_url: summary.thumbnail.map(|t| t.source),
                parental_wealth: None,
            };
            
            Ok(Some(billionaire))
        } else {
            Ok(None)
        }
    }

    fn name(&self) -> &'static str {
        "Wikipedia"
    }
}