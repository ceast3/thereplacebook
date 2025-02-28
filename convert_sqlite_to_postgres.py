import re

with open("dump.sql", "r") as f:
    content = f.read()

# Convert SQLite syntax to PostgreSQL
content = re.sub(r"PRAGMA.*;\n", "", content)  # Remove SQLite-specific settings
content = content.replace("AUTOINCREMENT", "SERIAL")  # Convert AUTOINCREMENT to SERIAL
content = content.replace("INTEGER PRIMARY KEY", "SERIAL PRIMARY KEY")  # Adjust primary key format
content = content.replace("BOOLEAN", "BOOLEAN DEFAULT FALSE")  # Ensure boolean defaults
content = re.sub(r"INSERT INTO \"(.*?)\"", r"INSERT INTO \1", content)  # Remove double quotes

# Write the fixed SQL to a new file
with open("dump_postgres.sql", "w") as f:
    f.write(content)

print("Converted dump file saved as dump_postgres.sql")