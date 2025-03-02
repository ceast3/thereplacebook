import os
import requests
from bs4 import BeautifulSoup
from PIL import Image
from io import BytesIO
import psycopg2
from sqlalchemy import create_engine

# Database connection settings
DB_NAME = "your_db"
DB_USER = "your_user"
DB_PASS = "your_password"
DB_HOST = "localhost"
DB_PORT = "5432"

# Forbes Billionaires URL
URL = "https://www.forbes.com/real-time-billionaires/"

# Create a directory to store images
IMAGE_DIR = "billionaire_images"
if not os.path.exists(IMAGE_DIR):
    os.makedirs(IMAGE_DIR)

# Fetch the Forbes Billionaires page
headers = {"User-Agent": "Mozilla/5.0"}
response = requests.get(URL, headers=headers)

if response.status_code == 200:
    soup = BeautifulSoup(response.text, "html.parser")

    # Extract billionaire details
    billionaires = []
    for row in soup.find_all("div", class_="table-row")[:25]:  # Top 25
        try:
            name = row.find("div", class_="personName").text.strip()
            net_worth = row.find("div", class_="netWorth").text.strip()
            image_url = row.find("img")["src"] if row.find("img") else ""

            # Download and resize image to 300px
            if image_url:
                img_response = requests.get(image_url)
                if img_response.status_code == 200:
                    img = Image.open(BytesIO(img_response.content))
                    img = img.resize((300, 300))  # Resize to 300px
                    image_filename = f"{IMAGE_DIR}/{name.replace(' ', '_')}.jpg"
                    img.save(image_filename)
                else:
                    image_filename = ""  # Default if image download fails
            else:
                image_filename = ""

            billionaires.append((name, image_filename, net_worth))

        except AttributeError:
            continue  # Skip invalid rows

    # Insert into PostgreSQL
    try:
        engine = create_engine(f"postgresql://{DB_USER}:{DB_PASS}@{DB_HOST}:{DB_PORT}/{DB_NAME}")
        conn = psycopg2.connect(dbname=DB_NAME, user=DB_USER, password=DB_PASS, host=DB_HOST, port=DB_PORT)
        cur = conn.cursor()

        insert_query = """INSERT INTO users (name, image_url, net_worth) VALUES (%s, %s, %s);"""
        cur.executemany(insert_query, billionaires)

        conn.commit()
        cur.close()
        conn.close()

        print("✅ Data successfully inserted into PostgreSQL.")

    except Exception as e:
        print(f"❌ Database Error: {e}")

else:
    print("❌ Failed to retrieve Forbes webpage.")