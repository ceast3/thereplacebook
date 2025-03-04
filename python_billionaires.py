import os
import requests
from bs4 import BeautifulSoup
import psycopg2
from sqlalchemy import create_engine
from dotenv import load_dotenv

# Load environment variables from .env
load_dotenv()

# Get Database URL from .env
DATABASE_URL = os.getenv("DATABASE_URL")

# Connect to PostgreSQL
def connect_to_postgres():
    return psycopg2.connect(DATABASE_URL)

# Scrape Forbes Billionaires
def scrape_billionaires():
    URL = "https://www.forbes.com/real-time-billionaires/"
    headers = {"User-Agent": "Mozilla/5.0"}
    response = requests.get(URL, headers=headers)

    if response.status_code != 200:
        print("❌ Failed to retrieve Forbes webpage.")
        return []

    soup = BeautifulSoup(response.text, "html.parser")
    billionaires = []

    for row in soup.find_all("div", class_="table-row"):  # Collects all billionaires
        try:
            name = row.find("div", class_="personName").text.strip()
            net_worth = row.find("div", class_="netWorth").text.strip()
            image_url = row.find("img")["src"] if row.find("img") else ""

            billionaires.append((name, image_url, net_worth))

        except AttributeError:
            continue  # Skip invalid rows

    return billionaires

# Insert data into PostgreSQL
def insert_into_db(billionaires):
    conn = connect_to_postgres()
    cur = conn.cursor()

    insert_query = """INSERT INTO billionaires (name, image_url, net_worth) VALUES (%s, %s, %s) ON CONFLICT (name) DO NOTHING;"""
    cur.executemany(insert_query, billionaires)

    conn.commit()
    cur.close()
    conn.close()

    print(f"✅ Successfully inserted {len(billionaires)} billionaires into PostgreSQL.")

# Main execution
if __name__ == "__main__":
    billionaires = scrape_billionaires()
    if billionaires:
        insert_into_db(billionaires)