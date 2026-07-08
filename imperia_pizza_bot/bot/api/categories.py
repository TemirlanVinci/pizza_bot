from api.client import get

async def get_categories() -> list | None:
    return await get("api/v1/categories")