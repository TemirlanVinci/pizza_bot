from api.client import post

async def register_user(telegram_id: int) -> dict | None:
    payload = {
        "telegram_id": telegram_id
    }
    return await post("api/v1/users/register", payload)