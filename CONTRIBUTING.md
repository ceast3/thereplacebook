# Contributing to The Replacebook

Thank you for your interest in contributing to The Replacebook! This document provides guidelines and instructions for contributing to the project.

## 🤝 Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:
- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Respect differing viewpoints and experiences

## 🚀 Getting Started

### Prerequisites
1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/thereplacebook.git
   cd thereplacebook
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/ceast3/thereplacebook.git
   ```

### Development Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL (macOS)
brew install postgresql

# Install PostgreSQL (Ubuntu/Debian)
sudo apt-get install postgresql postgresql-contrib

# Set up the database
createdb thereplacebook_dev
export DATABASE_URL="postgresql://localhost/thereplacebook_dev"

# Run migrations
psql -d thereplacebook_dev -f migrations/001_create_indexes.sql

# Build the project
cargo build

# Run tests
cargo test
```

## 📝 Contributing Process

### 1. Find or Create an Issue
- Check existing issues for something you'd like to work on
- If creating a new issue, provide clear description and context
- Comment on the issue to indicate you're working on it

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 3. Make Your Changes
- Write clean, idiomatic Rust code
- Follow existing code patterns and conventions
- Add tests for new functionality
- Update documentation as needed

### 4. Code Style Guidelines

#### Rust Code Style
```rust
// Use descriptive variable names
let billionaire_count = 100; // Good
let n = 100; // Avoid

// Use proper error handling
fn fetch_data() -> Result<Data, AppError> {
    // Prefer ? operator over unwrap()
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

// Document public functions
/// Fetches billionaire data from multiple sources and returns
/// a deduplicated list sorted by net worth.
pub async fn fetch_billionaires(limit: Option<usize>) -> Result<Vec<Billionaire>> {
    // Implementation
}

// Use strong types
pub struct NetWorth(f64);  // Better than using raw f64
```

#### Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_billionaire_parsing() {
        // Arrange
        let input = create_test_data();
        
        // Act
        let result = parse_billionaire(input).await;
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Test Billionaire");
    }
}
```

### 5. Commit Your Changes
```bash
# Use conventional commit messages
git commit -m "feat: add Bloomberg data source integration"
git commit -m "fix: handle empty responses from Wikipedia API"
git commit -m "docs: update API documentation for new endpoints"
git commit -m "perf: optimize database queries with prepared statements"
```

Commit Message Format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions or fixes
- `chore:` Build process or auxiliary tool changes

### 6. Run Quality Checks
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Check for security vulnerabilities
cargo audit
```

### 7. Push and Create Pull Request
```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title describing the change
- Reference to related issue(s)
- Description of what changed and why
- Screenshots/examples if applicable

## 🏗️ Adding a New Data Source

One of the most valuable contributions is adding new data sources. Here's how:

### 1. Create the Data Source Module
```rust
// src/data_sources/your_source.rs
use crate::data_sources::DataSource;
use crate::errors::Result;
use crate::models::Billionaire;
use async_trait::async_trait;

pub struct YourSource {
    client: reqwest::Client,
    base_url: String,
}

impl YourSource {
    pub fn new() -> Self {
        // Initialize your data source
    }
}

#[async_trait]
impl DataSource for YourSource {
    async fn fetch_billionaires(&self, limit: Option<usize>) -> Result<Vec<Billionaire>> {
        // Implement fetching logic
    }

    async fn fetch_person_details(&self, name: &str) -> Result<Option<Billionaire>> {
        // Implement detail fetching
    }

    fn name(&self) -> &'static str {
        "YourSource"
    }
}
```

### 2. Register the Data Source
```rust
// src/data_sources/mod.rs
pub mod your_source;

impl DataSourceManager {
    pub fn new() -> Self {
        let mut sources: Vec<Box<dyn DataSource + Send + Sync>> = Vec::new();
        // ... existing sources ...
        sources.push(Box::new(your_source::YourSource::new()));
        Self { sources }
    }
}
```

### 3. Add Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_your_source_fetching() {
        let source = YourSource::new();
        let result = source.fetch_billionaires(Some(10)).await;
        assert!(result.is_ok());
    }
}
```

## 🧪 Testing Guidelines

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Aim for >80% code coverage

### Integration Tests
```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_data_pipeline() {
    // Test complete data flow from sources to database
}
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

## 📚 Documentation

### Code Documentation
- Document all public APIs
- Include examples in doc comments
- Explain complex algorithms

### README Updates
- Update feature list when adding new functionality
- Keep examples current
- Update performance benchmarks

### API Documentation
```rust
/// Fetches billionaire data with advanced filtering options.
/// 
/// # Arguments
/// * `filter` - Filter criteria for the query
/// * `limit` - Maximum number of results to return
/// 
/// # Examples
/// ```
/// let filter = Filter::new()
///     .industry("technology")
///     .min_wealth(10.0);
/// let results = fetch_filtered(filter, Some(50)).await?;
/// ```
pub async fn fetch_filtered(filter: Filter, limit: Option<usize>) -> Result<Vec<Billionaire>> {
    // Implementation
}
```

## 🔍 Review Process

### What We Look For
1. **Code Quality**: Clean, readable, efficient code
2. **Tests**: Adequate test coverage
3. **Documentation**: Clear comments and updated docs
4. **Performance**: No significant performance regressions
5. **Security**: No security vulnerabilities introduced

### Review Timeline
- Initial review within 48 hours
- Subsequent reviews within 24 hours
- Most PRs merged within a week

## 💡 Areas for Contribution

### High Priority
- [ ] Add more data sources (Bloomberg, Reuters, etc.)
- [ ] Implement caching layer with Redis
- [ ] Create web API with Rocket/Actix
- [ ] Add Docker support

### Medium Priority
- [ ] Improve error messages
- [ ] Add more analytics functions
- [ ] Create data visualization tools
- [ ] Implement data validation

### Good First Issues
- [ ] Add more unit tests
- [ ] Improve documentation
- [ ] Fix typos and formatting
- [ ] Add code examples

## 🐛 Reporting Bugs

### Bug Report Template
```markdown
**Description**
Clear description of the bug

**To Reproduce**
1. Run command '...'
2. See error

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., macOS 13.0]
- Rust Version: [e.g., 1.70.0]
- PostgreSQL Version: [e.g., 14.5]

**Additional Context**
Any other relevant information
```

## 📞 Getting Help

- **Discord**: Join our Discord server (coming soon)
- **GitHub Discussions**: Ask questions in Discussions
- **Email**: thereplacebook@example.com

## 🎉 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Given credit in commit messages

Thank you for contributing to The Replacebook! 🚀