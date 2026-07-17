from aiogram import Router
from aiogram.filters import CommandStart
from aiogram.types import Message, CallbackQuery, InputMediaPhoto
from aiogram.exceptions import TelegramBadRequest

from keyboards.main import kb_main_menu
from api.users import register_user

router = Router()


async def show_home(target: Message | CallbackQuery) -> None:
    text = "<b>Империя Пиццы</b>\n\nДобро пожаловать!\n\nВыберите раздел:"
    
    if isinstance(target, CallbackQuery):
        
        if target.message.photo:
            try:
                await target.message.delete()
            except TelegramBadRequest as e:
                pass
            await target.message.answer(text, reply_markup=kb_main_menu())
        else:
            await target.message.edit_text(text, reply_markup=kb_main_menu())
    else:
        await target.answer(text, reply_markup=kb_main_menu())


@router.message(CommandStart())
async def cmd_start(message: Message) -> None:
    await register_user(message.from_user.id)
    await show_home(message)


@router.callback_query(lambda c: c.data == "home")
async def cb_home(cb: CallbackQuery) -> None:
    await cb.answer()
    await show_home(cb)