import os
from dotenv import load_dotenv

load_dotenv()

BOT_TOKEN = os.getenv("BOT_TOKEN")
API_BASE = os.getenv("API_BASE")
BOT_SECRET = os.getenv("BOT_SECRET")

PRODUCTS_LIMIT = 5