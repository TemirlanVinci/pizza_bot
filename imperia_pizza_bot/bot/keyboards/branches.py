from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

BRANCHES_LIMIT = 5

def kb_pickup_branches(branches: list, offset: int = 0) -> InlineKeyboardMarkup:
    rows = []

    for b in branches:
        if not isinstance(b, dict):
            continue

        b_id = b.get("id") or b.get("branch_id")
        name = b.get("name", "Филиал")
        address = b.get("address") or b.get("location") or ""

        is_active_raw = b.get("is_active", True)
        is_active = str(is_active_raw).lower() not in ("false", "0", "none")

        full_name = f"{name} ({address})" if address else name
        status_text = full_name if is_active else f"❌ {full_name} (Закрыт)"

        # Определяем контекст по типу хендлера, который обрабатывает этот список. 
        # Если это просмотр филиалов из главного меню, используем префикс просмотра "branch_", 
        # иначе оставляем "order_pickbr_" для оформления заказа.
        # В данном контексте универсальнее использовать просмотр филиалов, 
        # а при чекауте можно передавать отдельный флаг или использовать отдельную клавиатуру.
        # Здесь делаем безопасный роутинг через просмотр деталей:
        
        # Если бот открыт из меню филиалов, то клик должен вести на детальную информацию:
        cb_data = f"branch_{b_id}_1" if is_active else f"branch_{b_id}_0"

        rows.append([
            InlineKeyboardButton(
                text=status_text,
                callback_data=cb_data
            )
        ])
    
    nav = []
    if offset > 0:
        nav.append(InlineKeyboardButton(text="⬅️ Назад", callback_data=f"branchpage_{offset - BRANCHES_LIMIT}"))

    if len(branches) == BRANCHES_LIMIT:
        nav.append(InlineKeyboardButton(text="Вперёд ➡️", callback_data=f"branchpage_{offset + BRANCHES_LIMIT}"))
            
    if nav:
        page = offset // BRANCHES_LIMIT + 1
        nav_center = InlineKeyboardButton(text=f"{page}", callback_data="noop")

        if len(nav) == 2:
            rows.append([nav[0], nav_center, nav[1]])
        elif offset == 0:
            rows.append([nav_center, nav[0]])
        else:
            rows.append([nav[0], nav_center])

    rows.append([
        InlineKeyboardButton(text="🏠 Главная", callback_data="home")
    ])

    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_back_to_branches(offset: int = 0) -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [
                InlineKeyboardButton(text="⬅️ К списку", callback_data=f"branchpage_{offset}"),
                InlineKeyboardButton(text="🏠 Главная", callback_data="home")
            ]
        ])