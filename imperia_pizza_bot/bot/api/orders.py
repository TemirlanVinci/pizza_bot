from api.client import get, post

async def create_order(payload: dict) -> dict | None:
    return await post("api/v1/orders", payload)

async def get_user_orders(user_id: int) -> dict | None:
    return await get("api/v1/orders", params={"user_id": user_id})

async def get_order_detail(order_id: int) -> dict | None:
    return await get(f"api/v1/orders/{order_id}")