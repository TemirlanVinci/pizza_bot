from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton
from aiogram.utils.keyboard import InlineKeyboardBuilder

def kb_favorites(favs: list) -> InlineKeyboardMarkup:
    builder = InlineKeyboardBuilder()
    
    for item in favs:
        button_text = f"🛍️ {item['name']} — {item['price']} c"
        callback_data = f"prod_view_{item['id']}"
        
        builder.row(InlineKeyboardButton(text=button_text, callback_data=callback_data))
    
    builder.row(InlineKeyboardButton(text="🍕 К меню", callback_data="menu"))
    builder.row(InlineKeyboardButton(text="🏠 Главная", callback_data="home"))
    
    return builder.as_markup()