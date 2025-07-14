use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Billionaire {
    pub name: String,
    pub net_worth: f64,
    pub source_of_wealth: String,
    pub age: Option<u32>,
    pub country: String,
    pub industry: String,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub philanthropy: Option<String>,
    pub notable_achievements: Option<String>,
    pub website: Option<String>,
    pub twitter_handle: Option<String>,
    pub linkedin_profile: Option<String>,
    pub quote: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub parental_wealth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WikidataResponse {
    pub results: WikidataResults,
}

#[derive(Debug, Deserialize)]
pub struct WikidataResults {
    pub bindings: Vec<WikidataBinding>,
}

#[derive(Debug, Deserialize)]
pub struct WikidataBinding {
    pub person: Option<WikidataValue>,
    pub personLabel: Option<WikidataValue>,
    pub netWorth: Option<WikidataValue>,
    pub countryLabel: Option<WikidataValue>,
    pub occupationLabel: Option<WikidataValue>,
    pub birthDate: Option<WikidataValue>,
    pub companyLabel: Option<WikidataValue>,
}

#[derive(Debug, Deserialize)]
pub struct WikidataValue {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenCorpResponse {
    pub results: Vec<OpenCorpCompany>,
}

#[derive(Debug, Deserialize)]
pub struct OpenCorpCompany {
    pub company: OpenCorpCompanyDetails,
}

#[derive(Debug, Deserialize)]
pub struct OpenCorpCompanyDetails {
    pub name: String,
    pub jurisdiction_code: Option<String>,
    pub company_type: Option<String>,
    pub current_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WikipediaSummary {
    pub title: String,
    pub extract: Option<String>,
    pub thumbnail: Option<WikipediaThumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct WikipediaThumbnail {
    pub source: String,
}