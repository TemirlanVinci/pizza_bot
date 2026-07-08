import aiohttp
from config import API_BASE

_session: aiohttp.ClientSession | None = None

def get_session() -> aiohttp.ClientSession:
    if _session is None or _session.closed:
        raise RuntimeError("HTTP-сессия не инициализирована")
    return _session

async def init_session() -> None:
    global _session
    _session = aiohttp.ClientSession(base_url=API_BASE)

async def close_session() -> None:
    global _session
    if _session and not _session.closed:
        await _session.close()


async def get(path: str, params: dict | None = None, headers: dict | None = None) -> dict | list | None:
    try:
        async with get_session().get(path, params=params, headers=headers) as r:
            if r.ok:  
                return await r.json()
            print(f"[API GET] Неожиданный статус {r.status} для {path}")
    except aiohttp.ClientError as e:
        print(f"[API GET] Ошибка {path}: {e}")
    return None

async def post(path: str, data: dict, headers: dict | None = None) -> dict | None:
    try:
        async with get_session().post(path, json=data, headers=headers) as r:
            if r.ok:
                return await r.json()
            print(f"[API POST] Неожиданный статус {r.status} для {path}")
    except aiohttp.ClientError as e:
        print(f"[API POST] Ошибка {path}: {e}")
    return None


async def delete(path: str, params: dict | None = None, headers: dict | None = None) -> dict | None:
    try:
        async with get_session().delete(path, params=params, headers=headers) as r:
            if r.ok:
                if r.status == 204:
                    return {"success": True}
                return await r.json()
            print(f"[API DELETE] Неожиданный статус {r.status} для {path}")
    except aiohttp.ClientError as e:
        print(f"[API DELETE] Ошибка {path}: {e}")
    return None