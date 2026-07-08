from api.client import get
from config import PRODUCTS_LIMIT

async def get_products(product_id: int) -> list | None:
    return await get(f"api/v1/products/{product_id}")


async def get_products_by_category(cat_id: int, offset: int = 0) -> list | None:
    return await get(f"api/v1/products?category_id={cat_id}&limit={PRODUCTS_LIMIT}&offset={offset}")
