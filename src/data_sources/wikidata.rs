use crate::data_sources::DataSource;
use crate::errors::{AppError, Result};
use crate::models::{Billionaire, WikidataResponse};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

pub struct WikidataSource {
    client: Client,
    base_url: String,
}

impl WikidataSource {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("thereplacebook/1.0 (https://github.com/user/thereplacebook)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://query.wikidata.org/sparql".to_string(),
        }
    }

    async fn query_sparql(&self, query: &str) -> Result<WikidataResponse> {
        let response = self
            .client
            .get(&self.base_url)
            .query(&[("query", query), ("format", "json")])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::DataSourceUnavailable("Wikidata".to_string()));
        }

        let data: WikidataResponse = response.json().await?;
        Ok(data)
    }

    fn build_billionaires_query(limit: Option<usize>) -> String {
        let limit_clause = limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default();
        
        format!(
            r#"
            SELECT DISTINCT ?person ?personLabel ?netWorth ?countryLabel ?occupationLabel ?birthDate ?companyLabel WHERE {{
              ?person wdt:P31 wd:Q5 .
              ?person wdt:P2218 ?netWorth .
              FILTER(?netWorth >= 1000000000)
              OPTIONAL {{ ?person wdt:P27 ?country . }}
              OPTIONAL {{ ?person wdt:P106 ?occupation . }}
              OPTIONAL {{ ?person wdt:P569 ?birthDate . }}
              OPTIONAL {{ ?person wdt:P108 ?company . }}
              SERVICE wikibase:label {{ bd:serviceParam wikibase:language "en" . }}
            }}
            ORDER BY DESC(?netWorth)
            {}
            "#,
            limit_clause
        )
    }

    fn build_person_query(name: &str) -> String {
        format!(
            r#"
            SELECT DISTINCT ?person ?personLabel ?netWorth ?countryLabel ?occupationLabel ?birthDate ?companyLabel WHERE {{
              ?person wdt:P31 wd:Q5 .
              ?person rdfs:label "{}"@en .
              OPTIONAL {{ ?person wdt:P2218 ?netWorth . }}
              OPTIONAL {{ ?person wdt:P27 ?country . }}
              OPTIONAL {{ ?person wdt:P106 ?occupation . }}
              OPTIONAL {{ ?person wdt:P569 ?birthDate . }}
              OPTIONAL {{ ?person wdt:P108 ?company . }}
              SERVICE wikibase:label {{ bd:serviceParam wikibase:language "en" . }}
            }}
            "#,
            name
        )
    }

    fn parse_net_worth(value: &str) -> f64 {
        // Parse values like "+1000000000" or "1.5e9"
        value
            .trim_start_matches('+')
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1_000_000_000.0 // Convert to billions
    }
}

#[async_trait]
impl DataSource for WikidataSource {
    async fn fetch_billionaires(&self, limit: Option<usize>) -> Result<Vec<Billionaire>> {
        let query = Self::build_billionaires_query(limit);
        let response = self.query_sparql(&query).await?;
        
        let mut billionaires = Vec::new();
        
        for binding in response.results.bindings {
            if let Some(person_label) = &binding.personLabel {
                let net_worth = binding.netWorth
                    .as_ref()
                    .map(|nw| Self::parse_net_worth(&nw.value))
                    .unwrap_or(1.0);

                let billionaire = Billionaire {
                    name: person_label.value.clone(),
                    net_worth,
                    source_of_wealth: binding.occupationLabel
                        .as_ref()
                        .map(|o| o.value.clone())
                        .unwrap_or_else(|| "Business".to_string()),
                    age: None, // We can calculate from birthDate if needed
                    country: binding.countryLabel
                        .as_ref()
                        .map(|c| c.value.clone())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    industry: binding.occupationLabel
                        .as_ref()
                        .map(|o| o.value.clone())
                        .unwrap_or_else(|| "Business".to_string()),
                    bio: None,
                    company: binding.companyLabel
                        .as_ref()
                        .map(|c| c.value.clone()),
                    philanthropy: None,
                    notable_achievements: None,
                    website: None,
                    twitter_handle: None,
                    linkedin_profile: None,
                    quote: None,
                    birthdate: binding.birthDate
                        .as_ref()
                        .and_then(|bd| chrono::NaiveDate::parse_from_str(&bd.value, "%Y-%m-%d").ok()),
                    image_url: None,
                    parental_wealth: None,
                };
                
                billionaires.push(billionaire);
            }
        }
        
        Ok(billionaires)
    }

    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>> {
        let query = Self::build_person_query(name);
        let response = self.query_sparql(&query).await?;
        
        if let Some(binding) = response.results.bindings.first() {
            if let Some(person_label) = &binding.personLabel {
                let net_worth = binding.netWorth
                    .as_ref()
                    .map(|nw| Self::parse_net_worth(&nw.value))
                    .unwrap_or(1.0);

                let billionaire = Billionaire {
                    name: person_label.value.clone(),
                    net_worth,
                    source_of_wealth: binding.occupationLabel
                        .as_ref()
                        .map(|o| o.value.clone())
                        .unwrap_or_else(|| "Business".to_string()),
                    age: None,
                    country: binding.countryLabel
                        .as_ref()
                        .map(|c| c.value.clone())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    industry: binding.occupationLabel
                        .as_ref()
                        .map(|o| o.value.clone())
                        .unwrap_or_else(|| "Business".to_string()),
                    bio: None,
                    company: binding.companyLabel
                        .as_ref()
                        .map(|c| c.value.clone()),
                    philanthropy: None,
                    notable_achievements: None,
                    website: None,
                    twitter_handle: None,
                    linkedin_profile: None,
                    quote: None,
                    birthdate: binding.birthDate
                        .as_ref()
                        .and_then(|bd| chrono::NaiveDate::parse_from_str(&bd.value, "%Y-%m-%d").ok()),
                    image_url: None,
                    parental_wealth: None,
                };
                
                return Ok(Some(billionaire));
            }
        }
        
        Ok(None)
    }

    fn name(&self) -> &'static str {
        "Wikidata"
    }
}