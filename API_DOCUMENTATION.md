# API Documentation - The Replacebook

This document provides detailed information about the external APIs used by The Replacebook and how to integrate new data sources.

## 📊 Current Data Sources

### 1. Wikidata SPARQL API

**Endpoint**: `https://query.wikidata.org/sparql`  
**Documentation**: [Wikidata Query Service](https://www.wikidata.org/wiki/Wikidata:SPARQL_query_service)  
**Rate Limit**: ~5 requests/second  
**Authentication**: None required

#### Example Query
```sparql
SELECT DISTINCT ?person ?personLabel ?netWorth ?countryLabel ?occupationLabel WHERE {
  ?person wdt:P31 wd:Q5 .                    # Instance of human
  ?person wdt:P2218 ?netWorth .              # Net worth property
  FILTER(?netWorth >= 1000000000)            # Billionaires only
  OPTIONAL { ?person wdt:P27 ?country . }    # Country of citizenship
  OPTIONAL { ?person wdt:P106 ?occupation . } # Occupation
  SERVICE wikibase:label { bd:serviceParam wikibase:language "en" . }
}
ORDER BY DESC(?netWorth)
LIMIT 100
```

#### Key Properties
- `P31`: Instance of
- `P2218`: Net worth
- `P27`: Country of citizenship
- `P106`: Occupation
- `P569`: Date of birth
- `P108`: Employer
- `P735`: Given name
- `P734`: Family name

#### Implementation Example
```rust
async fn query_wikidata(query: &str) -> Result<WikidataResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://query.wikidata.org/sparql")
        .query(&[("query", query), ("format", "json")])
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

### 2. Wikipedia REST API

**Endpoint**: `https://en.wikipedia.org/api/rest_v1/`  
**Documentation**: [Wikipedia REST API](https://en.wikipedia.org/api/rest_v1/)  
**Rate Limit**: 200 requests/second  
**Authentication**: None required (User-Agent recommended)

#### Available Endpoints
```
GET /page/summary/{title}      # Page summary with extract
GET /page/mobile-html/{title}  # Mobile-optimized HTML
GET /page/metadata/{title}     # Page metadata
GET /page/related/{title}      # Related pages
```

#### Example Request
```rust
async fn get_wikipedia_summary(name: &str) -> Result<WikipediaSummary> {
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        name.replace(" ", "_")
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "TheReplacebook/1.0")
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

#### Response Structure
```json
{
  "title": "Elon Musk",
  "pageid": 909036,
  "extract": "Elon Reeve Musk is a businessman and investor...",
  "thumbnail": {
    "source": "https://upload.wikimedia.org/...",
    "width": 243,
    "height": 320
  },
  "originalimage": {
    "source": "https://upload.wikimedia.org/...",
    "width": 2436,
    "height": 3200
  }
}
```

### 3. OpenCorporates API

**Endpoint**: `https://api.opencorporates.com/v0.4/`  
**Documentation**: [OpenCorporates API](https://api.opencorporates.com/documentation/API-Reference)  
**Rate Limit**: 500 requests/day (free tier)  
**Authentication**: API key for higher limits

#### Available Endpoints
```
GET /companies/search         # Search for companies
GET /officers/search         # Search for officers
GET /companies/{jurisdiction}/{company_number}  # Company details
GET /officers/{id}           # Officer details
```

#### Example Usage
```rust
async fn search_company_officers(name: &str) -> Result<Vec<Officer>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.opencorporates.com/v0.4/officers/search")
        .query(&[
            ("q", name),
            ("format", "json"),
            ("per_page", "50")
        ])
        .send()
        .await?;
    
    let data: OpenCorpResponse = response.json().await?;
    Ok(data.results.officers)
}
```

## 🔧 Adding New Data Sources

### Step 1: Research the API

Before implementing, research:
- Rate limits and authentication requirements
- Data quality and coverage
- API stability and maintenance
- Terms of service and licensing

### Step 2: Define Data Mapping

Map external data to our `Billionaire` model:

```rust
pub struct Billionaire {
    pub name: String,              // Required
    pub net_worth: f64,            // In billions
    pub source_of_wealth: String,  // Business sector
    pub age: Option<u32>,          // Calculate from birthdate
    pub country: String,           // Nationality
    pub industry: String,          // Primary industry
    pub bio: Option<String>,       // Biography
    pub company: Option<String>,   // Primary company
    // ... additional fields
}
```

### Step 3: Implement the DataSource Trait

```rust
#[async_trait]
pub trait DataSource {
    /// Fetch a list of billionaires from this source
    async fn fetch_billionaires(&self, limit: Option<usize>) -> Result<Vec<Billionaire>>;
    
    /// Fetch detailed information about a specific person
    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>>;
    
    /// Return the name of this data source
    fn name(&self) -> &'static str;
}
```

### Step 4: Handle Rate Limiting

Implement rate limiting to respect API constraints:

```rust
use tokio::time::{sleep, Duration};

pub struct RateLimiter {
    delay_ms: u64,
}

impl RateLimiter {
    pub async fn wait(&self) {
        sleep(Duration::from_millis(self.delay_ms)).await;
    }
}
```

### Step 5: Error Handling

Use our custom error types:

```rust
use crate::errors::{AppError, Result};

async fn fetch_data(url: &str) -> Result<Data> {
    let response = client.get(url).send().await
        .map_err(|e| AppError::Http(e))?;
    
    if !response.status().is_success() {
        return Err(AppError::DataSourceUnavailable(
            "API returned error status".to_string()
        ));
    }
    
    response.json().await
        .map_err(|e| AppError::Json(e))
}
```

## 📝 Data Source Best Practices

### 1. Caching
```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Cache<T> {
    data: HashMap<String, (T, Instant)>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    pub fn get(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .filter(|(_, time)| time.elapsed() < self.ttl)
            .map(|(data, _)| data.clone())
    }
    
    pub fn insert(&mut self, key: String, value: T) {
        self.data.insert(key, (value, Instant::now()));
    }
}
```

### 2. Retry Logic
```rust
async fn fetch_with_retry<T, F>(
    operation: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> futures::future::BoxFuture<'static, Result<T>>,
{
    let mut retries = 0;
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if retries < max_retries => {
                retries += 1;
                let delay = Duration::from_millis(100 * 2u64.pow(retries));
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### 3. Data Validation
```rust
impl Billionaire {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(AppError::Validation("Name cannot be empty".to_string()));
        }
        
        if self.net_worth < 0.0 {
            return Err(AppError::Validation("Net worth cannot be negative".to_string()));
        }
        
        // Additional validation...
        Ok(())
    }
}
```

## 🌐 Potential Future Data Sources

### Bloomberg API
- **Pros**: Real-time financial data, comprehensive coverage
- **Cons**: Expensive, requires commercial license
- **Alternative**: Use Bloomberg's public website data carefully

### Reuters API
- **Pros**: News and financial data
- **Cons**: Requires subscription
- **Use Case**: Sentiment analysis and news tracking

### SEC EDGAR
- **Pros**: Free, official US government data
- **Cons**: US-only, complex XML parsing
- **Endpoint**: `https://www.sec.gov/edgar/`

### CrunchBase API
- **Pros**: Startup and investment data
- **Cons**: Requires paid subscription
- **Use Case**: Tech billionaire tracking

### Yahoo Finance
- **Pros**: Free tier available
- **Cons**: Rate limits, unofficial API
- **Use Case**: Stock ownership data

## 🔒 Security Considerations

1. **API Keys**: Store in environment variables, never commit
2. **Rate Limiting**: Implement to avoid IP bans
3. **Data Validation**: Sanitize all external data
4. **HTTPS Only**: Always use secure connections
5. **User Agent**: Set appropriate User-Agent headers

## 📊 Data Quality Metrics

Track data quality for each source:

```rust
pub struct DataQualityMetrics {
    pub completeness: f32,      // % of fields populated
    pub accuracy: f32,          // Verified accuracy rate
    pub timeliness: Duration,   // Age of data
    pub consistency: f32,       // Cross-source agreement
}
```

## 🧪 Testing Data Sources

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Mock};

    #[tokio::test]
    async fn test_api_parsing() {
        let _m = mock("GET", "/api/endpoint")
            .with_status(200)
            .with_body(r#"{"name": "Test", "wealth": 1.5}"#)
            .create();
        
        let result = fetch_data(&mockito::server_url()).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
```rust
#[tokio::test]
#[ignore] // Run with --ignored flag
async fn test_live_api() {
    let source = WikidataSource::new();
    let result = source.fetch_billionaires(Some(5)).await;
    
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}
```

## 📈 Monitoring and Metrics

Implement monitoring for each data source:

```rust
use prometheus::{Counter, Histogram};

lazy_static! {
    static ref API_REQUESTS: Counter = Counter::new(
        "api_requests_total", "Total API requests"
    ).unwrap();
    
    static ref API_LATENCY: Histogram = Histogram::new(
        "api_request_duration_seconds", "API request latency"
    ).unwrap();
}
```

---

For questions or additions to this documentation, please open an issue or submit a PR!