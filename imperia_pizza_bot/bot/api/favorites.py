from api.client import get, post, delete

async def get_favorites(user_id: int, limit: int = 5, offset: int = 0) -> list | None:
    params = {
        "user_id": user_id,
        "limit": limit,
        "offset": offset
    }
    return await get("api/v1/favorites", params=params)


async def add_favorite(user_id: int, product_id: int) -> dict | None:
    payload = {
        "user_id": user_id,
        "product_id": product_id
    }
    return await post("api/v1/favorites", payload)


async def delete_favorite(user_id: int, product_id: int) -> dict | None:
    return await delete(f"api/v1/favorites/{product_id}", params={"user_id": user_id})