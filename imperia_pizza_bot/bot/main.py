import asyncio
import logging
from aiogram import Bot, Dispatcher, F
from aiogram.types import CallbackQuery
from aiogram.fsm.storage.memory import MemoryStorage
from aiogram.client.default import DefaultBotProperties

from config import BOT_TOKEN
from api.client import init_session, close_session
from handlers import start, categories, products, favorites, cart

logging.basicConfig(level=logging.INFO)

bot = Bot(token=BOT_TOKEN, default=DefaultBotProperties(parse_mode="HTML"))
dp = Dispatcher(storage=MemoryStorage()) 


dp.include_router(start.router)
dp.include_router(categories.router)
dp.include_router(products.router)
dp.include_router(favorites.router)
dp.include_router(cart.router)


@dp.callback_query(F.data == "noop")
async def cb_noop(cb: CallbackQuery) -> None:
    await cb.answer()

async def main() -> None:
    await init_session()
    try:
        await dp.start_polling(bot)
    finally:
        await close_session()
        await bot.session.close()

if __name__ == "__main__":
    asyncio.run(main())
    