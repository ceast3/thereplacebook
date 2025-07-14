//! Filter-based search implementation for multi-criteria queries.

use super::{SearchQuery, SearchResult, SortBy, SortOrder};
use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::{PgPool, Row};

/// Performs a filter-based search without full-text requirements
pub async fn filter_search(
    pool: &PgPool,
    query: &SearchQuery,
) -> Result<Vec<SearchResult>> {
    let mut sql = String::from(
        "SELECT 
            id, name, net_worth, source_of_wealth, nationality, industry, 
            biography, company, philanthropy, notable_achievements, rating,
            birthdate
        FROM users 
        WHERE 1=1"
    );
    
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 0;
    
    // Add industry filter
    if let Some(industry) = &query.industry {
        param_count += 1;
        sql.push_str(&format!(" AND industry ILIKE ${}", param_count));
        params.push(format!("%{}%", industry));
    }
    
    // Add country filter
    if let Some(country) = &query.country {
        param_count += 1;
        sql.push_str(&format!(" AND nationality ILIKE ${}", param_count));
        params.push(format!("%{}%", country));
    }
    
    // Add company filter
    if let Some(company) = &query.company {
        param_count += 1;
        sql.push_str(&format!(" AND company ILIKE ${}", param_count));
        params.push(format!("%{}%", company));
    }
    
    // Add wealth range filters
    if let Some(min_wealth) = query.min_wealth {
        sql.push_str(&format!(
            " AND CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) >= {}",
            min_wealth
        ));
    }
    
    if let Some(max_wealth) = query.max_wealth {
        sql.push_str(&format!(
            " AND CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) <= {}",
            max_wealth
        ));
    }
    
    // Add age filters (if birthdate is available)
    if query.min_age.is_some() || query.max_age.is_some() {
        sql.push_str(" AND birthdate IS NOT NULL");
        
        if let Some(min_age) = query.min_age {
            sql.push_str(&format!(
                " AND EXTRACT(YEAR FROM AGE(birthdate)) >= {}",
                min_age
            ));
        }
        
        if let Some(max_age) = query.max_age {
            sql.push_str(&format!(
                " AND EXTRACT(YEAR FROM AGE(birthdate)) <= {}",
                max_age
            ));
        }
    }
    
    // Add sorting
    let sort_by = query.sort_by.as_ref().unwrap_or(&SortBy::Rating);
    let sort_order = query.sort_order.as_ref().unwrap_or(&SortOrder::Desc);
    
    match sort_by {
        SortBy::NetWorth => {
            sql.push_str(" ORDER BY CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT)");
        }
        SortBy::Name => {
            sql.push_str(" ORDER BY name");
        }
        SortBy::Rating => {
            sql.push_str(" ORDER BY rating");
        }
        SortBy::Age => {
            sql.push_str(" ORDER BY EXTRACT(YEAR FROM AGE(birthdate))");
        }
        _ => {
            sql.push_str(" ORDER BY rating");
        }
    }
    
    match sort_order {
        SortOrder::Desc => sql.push_str(" DESC"),
        SortOrder::Asc => sql.push_str(" ASC"),
    }
    
    // Add limit and offset
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    // Execute query
    let mut query_builder = sqlx::query(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    
    let rows = query_builder.fetch_all(pool).await?;
    
    // Convert to SearchResult
    let results: Vec<SearchResult> = rows.into_iter().map(|row| {
        let age = if let Some(birthdate) = row.get::<Option<chrono::NaiveDate>, _>("birthdate") {
            Some(calculate_age(birthdate))
        } else {
            None
        };
        
        let billionaire = Billionaire {
            name: row.get("name"),
            net_worth: parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
            source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
            age,
            country: row.get::<Option<String>, _>("nationality").unwrap_or_default(),
            industry: row.get::<Option<String>, _>("industry").unwrap_or_default(),
            bio: row.get("biography"),
            company: row.get("company"),
            philanthropy: row.get("philanthropy"),
            notable_achievements: row.get("notable_achievements"),
            website: None,
            twitter_handle: None,
            linkedin_profile: None,
            quote: None,
            birthdate: row.get("birthdate"),
            image_url: None,
            parental_wealth: None,
        };
        
        SearchResult {
            billionaire,
            relevance_score: 1.0, // No relevance score for filter-based search
            matched_fields: vec!["filter".to_string()],
            highlight_snippets: vec![],
        }
    }).collect();
    
    Ok(results)
}

/// Builds a complex multi-criteria query
pub struct QueryBuilder {
    conditions: Vec<String>,
    params: Vec<String>,
    param_count: usize,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            conditions: vec!["1=1".to_string()],
            params: Vec::new(),
            param_count: 0,
        }
    }
    
    pub fn add_text_condition(&mut self, field: &str, value: &str) {
        self.param_count += 1;
        self.conditions.push(format!("{} ILIKE ${}", field, self.param_count));
        self.params.push(format!("%{}%", value));
    }
    
    pub fn add_numeric_condition(&mut self, field: &str, operator: &str, value: f64) {
        self.conditions.push(format!("{} {} {}", field, operator, value));
    }
    
    pub fn add_in_condition(&mut self, field: &str, values: &[String]) {
        if values.is_empty() {
            return;
        }
        
        let placeholders: Vec<String> = values.iter().enumerate().map(|(i, _)| {
            self.param_count += 1;
            format!("${}", self.param_count)
        }).collect();
        
        self.conditions.push(format!("{} IN ({})", field, placeholders.join(", ")));
        self.params.extend(values.iter().cloned());
    }
    
    pub fn build(&self) -> (String, Vec<String>) {
        let where_clause = self.conditions.join(" AND ");
        (where_clause, self.params.clone())
    }
}

/// Calculates age from birthdate
fn calculate_age(birthdate: chrono::NaiveDate) -> u32 {
    let today = chrono::Local::now().naive_local().date();
    let age = today.year() - birthdate.year();
    
    if today.month() < birthdate.month() || 
       (today.month() == birthdate.month() && today.day() < birthdate.day()) {
        (age - 1) as u32
    } else {
        age as u32
    }
}

/// Parses net worth string to numeric value
fn parse_net_worth(net_worth_str: &str) -> f64 {
    net_worth_str
        .replace("$", "")
        .replace("B", "")
        .parse::<f64>()
        .unwrap_or(0.0)
}