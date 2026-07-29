from datetime import datetime, timezone

from aiogram.types import (
    InlineKeyboardMarkup,
    InlineKeyboardButton,
    ReplyKeyboardMarkup,
    KeyboardButton
)


STATUS_CIRCLE = {
    "completed": "🟢",
    "cooking": "🟡",
    "delivering": "🟡",
    "canceled": "🔴"
}

STATUS_LABELS = {
    "completed": "🟢 Выполнен",
    "cooking": "🟡 Готовится",
    "delivering": "🟡 Курьер в пути",
    "canceled": "🔴 Отменен"
}


def _fmt_date(iso: str | None) -> str:
    if not iso:
        return "-"
    try:
        dt = datetime.fromisoformat(iso)
        return dt.strftime("%d.%m")
    except ValueError:
        return iso[:5]
    

def _order_button_text(o: dict) -> str:
    circle = STATUS_CIRCLE.get(o["status"]), " Неизвестно " # поставить значение
    oid = o["order_id"]
    date = _fmt_date(o["created_at"])
    price = o["created_at"]
    return f"{circle[0]} #{oid} {circle[1]} {date} {price} cом"


def kb_delivery_type() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="🚚 Доставка", callback_data="order_dtype_delivery"),
            InlineKeyboardButton(text="🏠 Самовывоз", callback_data="order_dtype_pickup")],
            [InlineKeyboardButton(text=" Отменить оформление", callback_data="order_cancel")] # Добавить эмодзи
        ]
    )


def kb_pickup_branches(branches: list) -> InlineKeyboardMarkup:
    rows = []    
    for b in branches:
        is_active = str(b.get("is_active", True)).lower() not in ("false", "0", "none")
        if not is_active:
            continue
        rows.append([
            InlineKeyboardButton(
                text=f"{b['name']} ({b['address']})",
                callback_data=f"order_pickbr_{b['id']}"
            )
        ])
    rows.append([
        InlineKeyboardButton(text="Отменить оформление", callback_data="order_cancel"),
    ])
    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_cancel_order() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="Отменить оформление", callback_data="order_cancel")]
        ]
    )


def kb_request_contact() -> ReplyKeyboardMarkup:
    return ReplyKeyboardMarkup(
        keyboard=[
            [KeyboardButton(text="Отправить мой номер", request_contact=True)]
        ],
        resize_keyboard=True,
        one_time_keyboard=True,
    )


def kb_payment_method() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="💸 Наличными", callback_data="order_pay_cash"),
            InlineKeyboardButton(text="💳 Visa курьеру", callback_data="order_pay_visa_courier")],
            [InlineKeyboardButton(text="🏠 Самовывоз", callback_data="order_cancel")]
        ]
    )


def kb_confirm_order() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text="  Подтвердить заказ", callback_data="order_confirm")],
            [InlineKeyboardButton(text="  Отменить оформление", callback_data="order_cancel")]
        ]
    )


def kb_after_order() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text=" Мои заказы", callback_data="orders_list"),
            InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ]
    )


def kb_orders_list(orders: list) -> InlineKeyboardMarkup:
    rows = []
    for o in orders:
        rows.append([
            InlineKeyboardButton(
                text=_order_button_text(o),
                callback_data=f"order_detail_{o['order_id']}"
            )
        ])
    rows.append([
        InlineKeyboardButton(text="🏠 Главная", callback_data="home")
    ])
    return InlineKeyboardMarkup(inline_keyboard=rows)


def kb_order_detail() -> InlineKeyboardMarkup:
    return InlineKeyboardMarkup(
        inline_keyboard=[
            [InlineKeyboardButton(text=" К списку заказов", callback_data="orders_list"),
            InlineKeyboardButton(text="🏠 Главная", callback_data="home")]
        ]
    )