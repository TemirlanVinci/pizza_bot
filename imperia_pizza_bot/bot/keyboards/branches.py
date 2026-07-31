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

        # Для самовывоза делаем отдельный префикс отклика
        rows.append([
            InlineKeyboardButton(
                text=status_text,
                callback_data=f"order_pickbr_{b_id}"
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

    rows.append([
        InlineKeyboardButton(text="❌ Отмена", callback_data="order_cancel")
    ])

    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_back_to_branches(offset: int = 0) -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="⬅️ К списку филиалов", callback_data=f"branchpage_{offset}"),
            InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ])
