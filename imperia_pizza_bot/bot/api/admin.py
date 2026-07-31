from api.client import get, post, patch


async def get_active_orders() -> list | None:
    return await get("api/v1/admin/orders")


async def get_users() -> list | None:
    return await get("api/v1/admin/users", {})


async def update_order_status(order_id: int, admin_tg_id: int, status: str) -> dict | None:
    payload = {
        "admin_tg_id": admin_tg_id,
        "status": status
    }
    return await patch("api/v1/admin/orders/{order_id}/status", payload)


async def get_admin_list() -> dict | None:
    return await get("api/v1/admin/list")

