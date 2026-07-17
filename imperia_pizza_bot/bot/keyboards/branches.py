from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

BRANCHES_LIMIT = 5

def kb_branches_list(branches: list, offset: int) -> InlineKeyboardMarkup:
    rows = []
    for b in branches:
        b_id = b.get("id")
        name = b.get("name", "Без названия")
        is_active_raw = b.get("is_active", False)
        
        is_active = str(is_active_raw).lower() not in ("false", "0", "none")
        status_text = name if is_active else f"❌ {name} (Закрыт)"
        
        active_flag = 1 if is_active else 0
        
        rows.append([
            InlineKeyboardButton(
                text=status_text,
                callback_data=f"branch_{b_id}_{active_flag}"
            )
        ])

    nav = []
    if offset > 0:
        nav.append(InlineKeyboardButton(text="⬅️ Назад", callback_data=f"branchpage_{offset - BRANCHES_LIMIT}"))
    
    if len(branches) == BRANCHES_LIMIT:
        nav.append(InlineKeyboardButton(text="Вперёд ➡️", callback_data=f"branchpage_{offset + BRANCHES_LIMIT}"))
        
    if nav:
        page = offset // BRANCHES_LIMIT + 1
        nav_center = InlineKeyboardButton(text=f"стр. {page}", callback_data="noop")
        
        if len(nav) == 2:
            rows.append([nav[0], nav_center, nav[1]]) 
        elif offset == 0:
            rows.append([nav_center, nav[0]])        
        else:
            rows.append([nav[0], nav_center])         
    
    rows.append([InlineKeyboardButton(text="🍕 К меню", callback_data="menu")])
    rows.append([InlineKeyboardButton(text="🏠 Главная", callback_data="home")])
    
    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_back_to_branches(offset: int = 0) -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="⬅️ К списку филиалов", callback_data=f"branchpage_{offset}")],
            [InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ])
