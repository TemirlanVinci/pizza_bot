from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton
from aiogram.utils.keyboard import InlineKeyboardBuilder

def kb_cart(items: list) -> InlineKeyboardMarkup:
    builder = InlineKeyboardBuilder()
    
    for item in items:
        pid = item['product_id']
        
        builder.row(InlineKeyboardButton(
            text=f"🛍️ {item['name']} — {item['price']} c", 
            callback_data=f"prod_view_{pid}"
        ))
        
        builder.row(
            InlineKeyboardButton(text="➖", callback_data=f"cart_dec_{pid}"),
            InlineKeyboardButton(text=f"{item['quantity']} шт.", callback_data=f"cart_count_{pid}"),
            InlineKeyboardButton(text="➕", callback_data=f"cart_inc_{pid}"),
            InlineKeyboardButton(text="🗑️", callback_data=f"cart_del_{pid}")
        )
    
    builder.row(InlineKeyboardButton(text="📱 Оформить заказ", callback_data="cart_checkout"))
    builder.row(InlineKeyboardButton(text="🏠 Главная", callback_data="home"))
    
    return builder.as_markup()