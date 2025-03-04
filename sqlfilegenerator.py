import requests
from bs4 import BeautifulSoup

# URL of Forbes Real-Time Billionaires page
url = "https://www.forbes.com/real-time-billionaires/"

# Send a GET request to Forbes
headers = {"User-Agent": "Mozilla/5.0"}
response = requests.get(url, headers=headers)

# Ensure the request was successful
if response.status_code != 200:
    print("❌ Failed to retrieve data from Forbes.")
    exit()

# Parse the HTML content
soup = BeautifulSoup(response.text, "html.parser")

# Find the section containing billionaires' data
billionaires_section = soup.find("div", {"class": "rtb-table"})

# Extract billionaire data
billionaires = billionaires_section.find_all("div", {"class": "rtb-row"})

# Prepare SQL queries
sql_statements = []

for billionaire in billionaires[:100]:  # Limit to top 100
    try:
        name = billionaire.find("div", {"class": "personName"}).text.strip().replace("'", "''")
        net_worth = billionaire.find("div", {"class": "netWorth"}).text.strip().replace("$", "").replace(",", "")
        image_tag = billionaire.find("img", {"class": "profile-image"})
        image_url = image_tag["src"] if image_tag else ""

        sql_query = f"""
        INSERT INTO users (name, image_url, net_worth)
        VALUES ('{name}', '{image_url}', '${net_worth} billion')
        ON CONFLICT (name) DO UPDATE
        SET image_url = EXCLUDED.image_url,
            net_worth = EXCLUDED.net_worth;
        """
        sql_statements.append(sql_query)

    except AttributeError:
        continue  # Skip any missing data rows

# Write SQL queries to a file
with open("billionaires.sql", "w", encoding="utf-8") as file:
    file.write("\n".join(sql_statements))

print("✅ SQL file 'billionaires.sql' generated successfully.")