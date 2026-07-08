from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton
from config import PRODUCTS_LIMIT


def kb_products(products: list, cat_id: int, offset: int, has_next: bool) -> InlineKeyboardMarkup:
    rows = [
        [InlineKeyboardButton(
            text=f"{p['name']} - {p['price']} c",
            callback_data=f"catprod_{p['id']}",
        )]
        for p in products
    ]

    nav = []
    if offset > 0:
        nav.append(InlineKeyboardButton(text="⬅️ Назад", callback_data=f"cat_{cat_id}_{offset - PRODUCTS_LIMIT}"))
    if has_next:
        nav.append(InlineKeyboardButton(text="Вперёд ➡️", callback_data=f"cat_{cat_id}_{offset + PRODUCTS_LIMIT}"))
    if nav:
        page = offset // PRODUCTS_LIMIT + 1
        nav_center = [InlineKeyboardButton(text=f"стр. {page}", callback_data="noop")]
        rows.append([nav[0], nav_center[0]] if len(nav) == 1 and offset == 0
                    else [nav[0], nav_center[0], nav[1]] if len(nav) == 2
                    else [nav_center[0], nav[0]])
    
    rows.append([InlineKeyboardButton(text="🍕 К меню", callback_data="menu")])
    rows.append([InlineKeyboardButton(text="🏠 Главная", callback_data="home")])
    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_product_actions(product_id: int, is_fav: bool) -> InlineKeyboardMarkup:
    fav_fext = "Убрать из избранного" if is_fav else "Добавить в избранное"
    fav_cb = f"fav_del_{product_id}" if is_fav else f"fav_add_{product_id}"

    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="🛒В корзину", callback_data=f"cart_add_{product_id}")],
            [InlineKeyboardButton(text=fav_fext, callback_data=fav_cb)],
            [InlineKeyboardButton(text="🍕 К меню", callback_data="menu")],
            [InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ]
    )


