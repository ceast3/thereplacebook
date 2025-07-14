//! Full-text search implementation for biographies and text fields.

use super::{SearchQuery, SearchResult, SearchField, HighlightSnippet, SortBy, SortOrder};
use crate::errors::Result;
use crate::models::Billionaire;
use sqlx::{PgPool, Row};
use tracing::info;

/// Performs full-text search on biographies and other text fields
pub async fn search_biographies(
    pool: &PgPool,
    query: &str,
    search_params: &SearchQuery,
) -> Result<Vec<SearchResult>> {
    info!("Performing full-text search for: {}", query);
    
    // Build the search fields clause
    let search_fields = search_params.search_fields.as_ref()
        .map(|fields| fields.clone())
        .unwrap_or_else(|| vec![SearchField::All]);
    
    let mut results = Vec::new();
    
    // Perform search based on selected fields
    if search_fields.contains(&SearchField::All) || search_fields.contains(&SearchField::Biography) {
        let bio_results = search_biography_field(pool, query, search_params).await?;
        results.extend(bio_results);
    }
    
    if search_fields.contains(&SearchField::All) || search_fields.contains(&SearchField::SourceOfWealth) {
        let wealth_results = search_text_field(pool, query, "source_of_wealth", search_params).await?;
        results.extend(wealth_results);
    }
    
    if search_fields.contains(&SearchField::All) || search_fields.contains(&SearchField::NotableAchievements) {
        let achievement_results = search_text_field(pool, query, "notable_achievements", search_params).await?;
        results.extend(achievement_results);
    }
    
    if search_fields.contains(&SearchField::All) || search_fields.contains(&SearchField::Philanthropy) {
        let philanthropy_results = search_text_field(pool, query, "philanthropy", search_params).await?;
        results.extend(philanthropy_results);
    }
    
    // Deduplicate results by name
    deduplicate_and_sort_results(&mut results, &search_params.sort_by, &search_params.sort_order);
    
    // Apply limit and offset
    let offset = search_params.offset.unwrap_or(0);
    let limit = search_params.limit.unwrap_or(50);
    
    Ok(results.into_iter()
        .skip(offset)
        .take(limit)
        .collect())
}

/// Searches specifically in the biography field with PostgreSQL full-text search
async fn search_biography_field(
    pool: &PgPool,
    query: &str,
    search_params: &SearchQuery,
) -> Result<Vec<SearchResult>> {
    let mut sql = String::from(
        "SELECT 
            id, name, net_worth, source_of_wealth, nationality, industry, 
            biography, company, philanthropy, notable_achievements, rating,
            ts_rank(to_tsvector('english', COALESCE(biography, '')), plainto_tsquery('english', $1)) as rank,
            ts_headline('english', COALESCE(biography, ''), plainto_tsquery('english', $1), 
                'StartSel=<mark>, StopSel=</mark>, MaxWords=50, MinWords=20') as headline
        FROM users 
        WHERE to_tsvector('english', COALESCE(biography, '')) @@ plainto_tsquery('english', $1)"
    );
    
    // Add additional filters
    let mut params: Vec<String> = vec![query.to_string()];
    let mut param_count = 1;
    
    if let Some(industry) = &search_params.industry {
        param_count += 1;
        sql.push_str(&format!(" AND industry ILIKE ${}", param_count));
        params.push(format!("%{}%", industry));
    }
    
    if let Some(country) = &search_params.country {
        param_count += 1;
        sql.push_str(&format!(" AND nationality ILIKE ${}", param_count));
        params.push(format!("%{}%", country));
    }
    
    if let Some(company) = &search_params.company {
        param_count += 1;
        sql.push_str(&format!(" AND company ILIKE ${}", param_count));
        params.push(format!("%{}%", company));
    }
    
    // Add wealth filters
    if let Some(min_wealth) = search_params.min_wealth {
        sql.push_str(&format!(" AND CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) >= {}", min_wealth));
    }
    
    if let Some(max_wealth) = search_params.max_wealth {
        sql.push_str(&format!(" AND CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT) <= {}", max_wealth));
    }
    
    sql.push_str(" ORDER BY rank DESC");
    
    let mut query_builder = sqlx::query(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    
    let rows = query_builder.fetch_all(pool).await?;
    
    let results: Vec<SearchResult> = rows.into_iter().map(|row| {
        let billionaire = Billionaire {
            name: row.get("name"),
            net_worth: parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
            source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
            age: None, // Calculate from birthdate if needed
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
            birthdate: None,
            image_url: None,
            parental_wealth: None,
        };
        
        let rank: f32 = row.get("rank");
        let headline: String = row.get("headline");
        
        SearchResult {
            billionaire,
            relevance_score: rank,
            matched_fields: vec!["biography".to_string()],
            highlight_snippets: vec![HighlightSnippet {
                field: "biography".to_string(),
                snippet: headline,
                start_offset: 0,
                end_offset: 0,
            }],
        }
    }).collect();
    
    Ok(results)
}

/// Searches in a specific text field
async fn search_text_field(
    pool: &PgPool,
    query: &str,
    field_name: &str,
    search_params: &SearchQuery,
) -> Result<Vec<SearchResult>> {
    let sql = format!(
        "SELECT 
            id, name, net_worth, source_of_wealth, nationality, industry, 
            biography, company, philanthropy, notable_achievements, rating, {},
            ts_rank(to_tsvector('english', COALESCE({}, '')), plainto_tsquery('english', $1)) as rank,
            ts_headline('english', COALESCE({}, ''), plainto_tsquery('english', $1), 
                'StartSel=<mark>, StopSel=</mark>, MaxWords=30, MinWords=15') as headline
        FROM users 
        WHERE to_tsvector('english', COALESCE({}, '')) @@ plainto_tsquery('english', $1)",
        field_name, field_name, field_name, field_name
    );
    
    let rows = sqlx::query(&sql)
        .bind(query)
        .fetch_all(pool)
        .await?;
    
    let results: Vec<SearchResult> = rows.into_iter().map(|row| {
        let billionaire = Billionaire {
            name: row.get("name"),
            net_worth: parse_net_worth(&row.get::<Option<String>, _>("net_worth").unwrap_or_default()),
            source_of_wealth: row.get::<Option<String>, _>("source_of_wealth").unwrap_or_default(),
            age: None,
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
            birthdate: None,
            image_url: None,
            parental_wealth: None,
        };
        
        let rank: f32 = row.get("rank");
        let headline: String = row.get("headline");
        
        SearchResult {
            billionaire,
            relevance_score: rank,
            matched_fields: vec![field_name.to_string()],
            highlight_snippets: vec![HighlightSnippet {
                field: field_name.to_string(),
                snippet: headline,
                start_offset: 0,
                end_offset: 0,
            }],
        }
    }).collect();
    
    Ok(results)
}

/// Deduplicates search results and sorts them
fn deduplicate_and_sort_results(
    results: &mut Vec<SearchResult>,
    sort_by: &Option<SortBy>,
    sort_order: &Option<SortOrder>,
) {
    // Sort by name first for deduplication
    results.sort_by(|a, b| a.billionaire.name.cmp(&b.billionaire.name));
    
    // Deduplicate by name, keeping the result with highest relevance
    results.dedup_by(|a, b| {
        if a.billionaire.name == b.billionaire.name {
            // Keep the one with higher relevance score
            if a.relevance_score < b.relevance_score {
                *a = b.clone();
            }
            true
        } else {
            false
        }
    });
    
    // Now sort by the requested criteria
    let sort_by = sort_by.as_ref().unwrap_or(&SortBy::Relevance);
    let sort_order = sort_order.as_ref().unwrap_or(&SortOrder::Desc);
    
    match sort_by {
        SortBy::Relevance => {
            results.sort_by(|a, b| {
                let cmp = a.relevance_score.partial_cmp(&b.relevance_score).unwrap();
                match sort_order {
                    SortOrder::Desc => cmp.reverse(),
                    SortOrder::Asc => cmp,
                }
            });
        }
        SortBy::NetWorth => {
            results.sort_by(|a, b| {
                let cmp = a.billionaire.net_worth.partial_cmp(&b.billionaire.net_worth).unwrap();
                match sort_order {
                    SortOrder::Desc => cmp.reverse(),
                    SortOrder::Asc => cmp,
                }
            });
        }
        SortBy::Name => {
            results.sort_by(|a, b| {
                let cmp = a.billionaire.name.cmp(&b.billionaire.name);
                match sort_order {
                    SortOrder::Desc => cmp.reverse(),
                    SortOrder::Asc => cmp,
                }
            });
        }
        _ => {} // Rating and Age would need additional data
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

/// Highlights search terms in text (simple implementation)
pub fn highlight_text(text: &str, search_terms: &[&str]) -> String {
    let mut highlighted = text.to_string();
    
    for term in search_terms {
        let lowercase_term = term.to_lowercase();
        let mut result = String::new();
        let mut last_end = 0;
        
        for (start, part) in highlighted.to_lowercase().match_indices(&lowercase_term) {
            result.push_str(&highlighted[last_end..start]);
            result.push_str("<mark>");
            result.push_str(&highlighted[start..start + part.len()]);
            result.push_str("</mark>");
            last_end = start + part.len();
        }
        result.push_str(&highlighted[last_end..]);
        highlighted = result;
    }
    
    highlighted
}