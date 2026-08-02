from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

STATUS_LABELS = {
    "confirmed": "🟢 Подтвержден",
    "cooking": "🟡 Готовится",
    "delivering": "🟡 Курьер в пути",
    "completed": "🟢 Выполнен",
    "cancelled": "🔴 Отменен",
}


NEXT_STATUSES = {
    "confirmed": ["cooking", "cancelled"],
    "cooking": ["delivering", "cancelled"],
    "delivering": ["completed", "cancelled"]
}


def kb_orders_list(orders: list) -> InlineKeyboardMarkup:
    rows = []

    for order in orders:
        label = STATUS_LABELS.get(order["status"], order["status"])
        rows.append([
            InlineKeyboardButton(
                text=f"#{order['order_id']} · {label} · {order['total_price']} c",
                callback_data=f"admin_order_{order['order_id']}"
            )
        ])

    rows.append([InlineKeyboardButton(
        text="🔄 Обновить", callback_data="admin_orders"
    )])

    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_order_detail(order_id: int, status: str) -> InlineKeyboardMarkup:
    rows = []

    for next_status in  NEXT_STATUSES.get(status, []):
        rows.append([
            InlineKeyboardButton(
                text=f"{STATUS_LABELS.get(next_status, next_status)}",
                callback_data=f"admin_status_{order_id}_{next_status}"
            )
        ])

    rows.append([
        InlineKeyboardButton(
            text="🏠 Главная", callback_data=f"admin_ban_ask_{order_id}")
    ])
    rows.append([
        InlineKeyboardButton(
            text="К списку заказов", callback_data="admin_orders")])

    return InlineKeyboardMarkup(inline_keyboard=rows)

def kb_ban_confirm(order_id: int) -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(inline_keyboard=[
        [
            InlineKeyboardButton(
                text="🚫 Забанить", callback_data=f"admin_ban_yes_{order_id}"),
            InlineKeyboardButton(
                text="❌ Отмена", callback_data=f"admin_ban_no_{order_id}"
                )
        ]
    ])