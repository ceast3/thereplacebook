# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
TheReplaceBook is a web application that allows users to vote on pairs of billionaires in a ranking system using Elo ratings. It's built with a Rust backend using Axum web framework and a vanilla HTML/CSS/JavaScript frontend.

## Build and Development Commands

### Rust Backend
- **Build**: `cargo build`
- **Run locally**: `cargo run`
- **Build for production**: `cargo build --release`
- **Build static binary**: `cargo build --release --target=x86_64-unknown-linux-musl`

### Docker
- **Build image**: `docker build -t thereplacebook .`
- **Run container**: `docker run -p 80:80 thereplacebook`

### Database Operations
- **Connect to PostgreSQL**: Use `DATABASE_URL` environment variable or AWS Secrets Manager
- **Run SQL scripts**: Files in `PSQL_scripts/` directory contain schema and data migration scripts
- **Convert SQLite to PostgreSQL**: `python convert_sqlite_to_postgres.py`

## Architecture

### Backend (Rust)
- **Framework**: Axum web server with Tokio async runtime
- **Database**: PostgreSQL via SQLx with connection pooling
- **Authentication**: AWS Secrets Manager for database credentials
- **Static Files**: Served from `static/` directory via tower-http
- **Port**: Runs on port 80

### Database Schema
- **users table**: id, name, image_url, net_worth, biography, company, rating (Elo-based)
- **matches table**: id, winner_id, loser_id (tracks voting results)

### API Endpoints
- `GET /users` - Fetch all users ordered by rating
- `POST /match` - Submit voting results (updates Elo ratings)
- `GET /` - Serves static frontend files

### Frontend
- **Technology**: Vanilla HTML/CSS/JavaScript with Tailwind CSS
- **Features**: Responsive design, hover overlays, real-time leaderboard updates
- **Voting System**: Click-based interface for pairwise comparisons

### Data Pipeline
- **Web Scraping**: Python scripts (`python_billionaires.py`, `billionairescraper.py`) for Forbes data
- **Data Migration**: Scripts to convert SQLite dumps to PostgreSQL format
- **Database Population**: SQL files in `PSQL_scripts/` for schema setup and data insertion

## Environment Setup
- Set `DATABASE_URL` for local development
- AWS credentials required for production (Secrets Manager access)
- PostgreSQL database required

## Deployment
- Multi-stage Docker build using musl for static linking
- Alpine Linux base image for minimal footprint
- AWS integration for secrets management
- Designed to run on port 80 in containerized environments