# The Replacebook - Billionaire Ranking App

A Rust-based billionaire ranking application that aggregates data from multiple publicly available sources and provides an interactive way to explore, compare, and analyze billionaire data.

## Features

### Data Sources
- **Wikidata**: Structured biographical and wealth data via SPARQL queries
- **Wikipedia**: Rich biographical content and images
- **OpenCorporates**: Company ownership and executive information
- **No Forbes dependency**: Uses only publicly available, license-free data

### Core Functionality
- Billionaire data collection and enrichment
- Industry-based matching and filtering
- Geographic analysis and country-based grouping
- Wealth tier analysis (1-5B, 5-20B, 20-50B, 50B+)
- Analytics and distribution reporting

### New Commands
- `match-industry <industry>` - Find billionaires by industry
- `match-country <country>` - Find billionaires by country
- `analytics` - Show industry and country distributions
- `wealth-tiers` - Show wealth distribution across tiers

## Usage

### Basic Commands
```bash
# Populate database with top 100 billionaires
cargo run top-100

# Update all billionaire data
cargo run update-all

# Find tech billionaires
cargo run match-industry technology

# Find US billionaires
cargo run match-country "United States"

# View analytics
cargo run analytics
cargo run wealth-tiers
```

### Database Setup
Ensure PostgreSQL is running and set the DATABASE_URL environment variable:
```bash
export DATABASE_URL="postgresql://username:password@localhost/thereplacebook"
```

Apply database optimizations:
```bash
psql -d thereplacebook -f migrations/001_create_indexes.sql
```

## Architecture

### Data Flow
1. **Data Sources**: Multiple public APIs provide different aspects of billionaire data
2. **Data Aggregation**: DataSourceManager coordinates fetching from all sources
3. **Deduplication**: Merges data by name to create comprehensive profiles
4. **Storage**: PostgreSQL with optimized indexes for fast queries
5. **Analytics**: Real-time analysis of industry, geographic, and wealth distributions

### Key Modules
- `data_sources/`: Modular data source implementations
- `features.rs`: Matching engine and analytics
- `errors.rs`: Comprehensive error handling
- `models.rs`: Data structures for all billionaire information

## Performance Optimizations

### Database
- Indexed columns for fast lookups (name, industry, nationality, rating)
- Composite indexes for common query patterns
- Functional index on parsed net worth values
- Foreign key constraints for data integrity

### API Handling
- Rate limiting to respect external APIs
- Concurrent processing where possible
- Graceful error handling and fallbacks
- Structured logging for monitoring

## Development

### Dependencies
- `reqwest`: HTTP client for API calls
- `sqlx`: Async PostgreSQL driver
- `serde`: JSON serialization
- `tracing`: Structured logging
- `async-trait`: Async trait support
- `regex`: Pattern matching for data extraction

### Contributing
1. Add new data sources by implementing the `DataSource` trait
2. Extend analytics by adding methods to the `Analytics` struct
3. Improve matching algorithms in the `MatchingEngine`
4. Add database migrations for schema changes

## Future Enhancements
- Real-time wealth tracking and alerts
- Social media integration for sentiment analysis
- Investment portfolio tracking from public filings
- Philanthropy impact scoring
- Interactive web interface
- Machine learning for wealth prediction models