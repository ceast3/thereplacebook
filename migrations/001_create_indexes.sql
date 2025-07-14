-- Add indexes for improved query performance

-- Index on name for lookups
CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);

-- Index on industry for filtering
CREATE INDEX IF NOT EXISTS idx_users_industry ON users(industry);

-- Index on nationality for country-based filtering  
CREATE INDEX IF NOT EXISTS idx_users_nationality ON users(nationality);

-- Index on rating for sorting
CREATE INDEX IF NOT EXISTS idx_users_rating ON users(rating DESC);

-- Composite index for industry + rating queries
CREATE INDEX IF NOT EXISTS idx_users_industry_rating ON users(industry, rating DESC);

-- Composite index for nationality + rating queries
CREATE INDEX IF NOT EXISTS idx_users_nationality_rating ON users(nationality, rating DESC);

-- Index on net_worth for wealth-based queries (using functional index for parsed values)
CREATE INDEX IF NOT EXISTS idx_users_net_worth_parsed ON users(
    CAST(REPLACE(REPLACE(net_worth, '$', ''), 'B', '') AS FLOAT)
);

-- Add constraints
ALTER TABLE users ADD CONSTRAINT unique_user_name UNIQUE (name);
ALTER TABLE matches ADD CONSTRAINT fk_winner FOREIGN KEY (winner_id) REFERENCES users(id);
ALTER TABLE matches ADD CONSTRAINT fk_loser FOREIGN KEY (loser_id) REFERENCES users(id);