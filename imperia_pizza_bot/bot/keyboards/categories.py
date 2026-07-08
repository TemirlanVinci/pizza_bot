from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

CATS_PER_PAGE = 12
CATS_PER_ROW = 3


def kb_categories(categories: list | dict, offset: int = 0) -> InlineKeyboardMarkup:
    if isinstance(categories, dict):
        extracted_list = None
        for val in categories.values():
            if isinstance(val, list):
                extracted_list = val
                break
        
        if extracted_list is not None:
            categories = extracted_list
        else:
            first_value = next(iter(categories.values())) if categories else None
            if isinstance(first_value, dict):
                categories = [
                    {"id": k, "name": v.get("name", f"Категория {k}")} 
                    for k, v in categories.items()
                ]
            else:
                categories = [{"id": k, "name": v} for k, v in categories.items()]

    page_cats = categories[offset : offset + CATS_PER_PAGE]
    rows = []

    for i in range(0, len(page_cats), CATS_PER_ROW):
        row = page_cats[i : i + CATS_PER_ROW]
        rows.append([
                InlineKeyboardButton(
                    text=cat["name"],
                    callback_data=f"cat_{cat['id']}",
                )
                for cat in row
            ])

    has_prev = offset > 0
    has_next = offset + CATS_PER_PAGE < len(categories)

    if has_prev or has_next:
        nav = []
        if has_prev:
            nav.append(InlineKeyboardButton(text="⬅️ Назад", callback_data=f"menu_{offset - CATS_PER_PAGE}"))
        page_num = offset // CATS_PER_PAGE + 1
        total_pages = (len(categories) + CATS_PER_PAGE - 1) // CATS_PER_PAGE
        nav.append(InlineKeyboardButton(text=f"{page_num}/{total_pages}", callback_data="noop"))
        if has_next:
            nav.append(InlineKeyboardButton(text="Вперёд ➡️", callback_data=f"menu_{offset + CATS_PER_PAGE}"))

        rows.append(nav)

    rows.append([InlineKeyboardButton(text="🏠 Главная", callback_data="home")])

    return InlineKeyboardMarkup(inline_keyboard=rows)