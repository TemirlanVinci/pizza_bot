from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

def kb_favorites(favs: list) -> InlineKeyboardMarkup:
    rows = []
    for item in favs:
        rows.append([InlineKeyboardButton(
            text=f"🛍️ {item['name']} — {item['price']} c",
            callback_data=f"prod_view_{item['id']}"
        )])
    rows.append([
        InlineKeyboardButton(text="🍽 К меню", callback_data="menu"),
        InlineKeyboardButton(text="🏠 Главная", callback_data="home"),
    ])
    return InlineKeyboardMarkup(inline_keyboard=rows)