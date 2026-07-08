from api.client import get, post, delete


async def get_cart(user_id: int) -> dict | None:
    return await get("api/v1/cart", params={"user_id": user_id})


async def cart_add(user_id: int, product_id: int) -> dict | None:
    payload = {
        "user_id": user_id,
        "product_id": product_id
    }
    return await post("api/v1/cart/add", payload)

async def cart_decrement(user_id: int, product_id: int) -> dict | None:
    payload = {
        "user_id": user_id,
        "product_id": product_id
    }
    return await post("api/v1/cart/decrement", payload)


async def cart_delete(user_id: int, product_id: int) -> dict | None:
    return await delete(f"api/v1/cart/item/{product_id}", params={"user_id": user_id})