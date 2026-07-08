from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

def kb_main_menu() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="🍕 Меню", callback_data="menu")],
            [InlineKeyboardButton(text="❤️ Избранное", callback_data="favorites")],
            [InlineKeyboardButton(text="🛒 Корзина", callback_data="cart")]
        ])


def kb_back_menu() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="🍕 Меню", callback_data="menu")],
            [InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ])
