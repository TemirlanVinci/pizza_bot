from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton
from aiogram.utils.keyboard import InlineKeyboardBuilder

def kb_cart(items: list) -> InlineKeyboardMarkup:
    builder = InlineKeyboardBuilder()
    
    for item in items:
        pid = item['product_id']
        
        # Убрали лишние эмодзи в названии товара для читаемости
        builder.row(InlineKeyboardButton(
            text=f"{item['name']} — {item['price']} сом", 
            callback_data=f"prod_view_{pid}"
        ))

        builder.row(
            InlineKeyboardButton(text="➖", callback_data=f"cart_dec_{pid}"),
            InlineKeyboardButton(text=f"{item['quantity']} шт.", callback_data=f"cart_count_{pid}"),
            InlineKeyboardButton(text="➕", callback_data=f"cart_inc_{pid}"),
            InlineKeyboardButton(text="🗑️", callback_data=f"cart_del_{pid}")
        )
    
    # Сделали акцент на кнопке оформления заказа
    builder.row(InlineKeyboardButton(text="✅ Оформить заказ", callback_data="cart_checkout"))
    
    # Дали возможность быстро вернуться к меню
    builder.row(
        InlineKeyboardButton(text="🍽 К меню", callback_data="menu"),
        InlineKeyboardButton(text="🏠 Главная", callback_data="home")
    )
    
    return builder.as_markup()