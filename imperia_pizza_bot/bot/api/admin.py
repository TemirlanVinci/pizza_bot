from api.client import get, post, patch


async def get_active_orders(admin_tg_id: int | None = None) -> list | None:
    payload = {"admin_tg_id": admin_tg_id} if admin_tg_id else {}
    return await post("api/v1/admin/orders/active", payload)


async def get_broadcast_users(admin_tg_id: int | None = None) -> list | None:
    payload = {"admin_tg_id": admin_tg_id} if admin_tg_id else {}
    return await post("api/v1/admin/users", payload)


async def update_order_status(order_id: int, admin_tg_id: int, status: str) -> dict | None:
    payload = {
        "admin_tg_id": admin_tg_id,
        "status": status
    }
    return await patch(f"api/v1/admin/orders/{order_id}/status", payload)


async def ban_user(
        admin_tg_id: int,
        user_tg_id: int,
        phone_number: str | None = None,
        ban_reason: str | None = None,
) -> dict | None:
    payload = {
        "admin_tg_id": admin_tg_id,
        "user_tg_id": user_tg_id,
        "phone_number": phone_number,
        "ban_reason": ban_reason
    }
    return await post("api/v1/admin/users/ban", payload)


async def get_admin_list() -> dict | None:
    return await get("api/v1/admin/list")

