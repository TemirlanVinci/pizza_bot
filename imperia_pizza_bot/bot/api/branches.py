from api.client import get

async def get_branches(limit: int = 20, offset: int = 0) -> list | None:
    params = {
        "limit": limit,
        "offset": offset
    }
    return await get("api/v1/branch", params=params)

async def get_branch_detail(branch_id: int) -> dict | None:
    return await get(f"api/v1/branch/{branch_id}")