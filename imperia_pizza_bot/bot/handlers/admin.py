import logging 

from aiogram import Router, F
from aiogram.filters import Command
from aiogram.types import Message, CallbackQuery
 
from api.admin import get_active_orders, update_order_status, ban_user
from filters import IsAdmin
from keyboards.admin import kb_orders_list, kb_order_detail, kb_ban_confirm, STATUS_LABELS
from formatting import format_datetime
 
router = Router()
router.message.filter(IsAdmin())
router.callback_query.filter(IsAdmin())

logger = logging.getLogger(__name__)

DELIVERY_LABELS = {
    "delivery": "🚚 Доставка",
    "pickup": "Самовывоз",
}


def _find_order(orders: list[dict], order_id: int) -> dict | None:
    for o in orders:
        if o["order_id"] == order_id:
            return o
    return None


def build_orders_list_text(orders: list[dict]) -> str:
    if not orders:
        return "<b>Активные заказы</b>\n\n Сейчас активных заказов нет."
    return (
        f"<b>Активные заказы:</b>\n\n"
        f"Всего: <b>{len(orders)}</b>\n\n"
        f"Выберите заказ для просмотра:"
    )


def build_order_detail_text(order: dict) -> str:
    status_label = STATUS_LABELS.get(order["status"], order["status"])
    delivery_label = DELIVERY_LABELS.get(order["delivery_type"], order["delivery_type"])

    lines = [
        f"<b>Заказ #{order['order_id']}</b>",
        f"Статус: {status_label}",
        f"Тип {delivery_label}",
    ]

    if order.get("delivery_type") == "delivery" and order.get("address"):
        lines.append(f"📍 Адрес: {order['address']}")

    lines.append(f"📞 Телефон: {order['phone_number']}")
    lines.append(f"💰 Сумма: <b>{order['total_price']} с</b>")
    lines.append(f"🕒 Создан: {format_datetime(order['created_at'])}")

    return "\n".join(lines)


async def show_orders_list(target: Message | CallbackQuery) -> None:
    orders = await get_active_orders()

    if orders is None:
        text = "❌ Не удалось загрузить список заказов."
        markup = kb_orders_list([])
    else:
        text = build_orders_list_text(orders)
        markup = kb_orders_list(orders)

    if isinstance(target, CallbackQuery):
        await target.message.edit_text(text, reply_markup=markup, parse_mode="HTML")
    else:
        await target.answer(text, reply_markup=markup, parse_mode="HTML")


@router.message(Command("admin"))
async def cmd_admin(message: Message) -> None:
    await show_orders_list(message)


@router.callback_query(F.data == "admin_orders")
async def cb_admin_orders(cb: CallbackQuery) -> None:
    await cb.answer()
    await show_orders_list(cb)


@router.callback_query(F.data.startswith("admin_order_"))
async def cb_admin_order_detail(cb: CallbackQuery) -> None:
    order_id = int(cb.data.split("_")[2])

    orders = await get_active_orders()
    order = _find_order(orders or [], order_id)

    if order is None:
        await cb.answer("Заказ не найден или уже неактивен.", show_alert=True)
        return

    await cb.answer()
    await cb.message.edit_text(
        build_order_detail_text(order),
        reply_markup=kb_order_detail(order_id, order["status"]),
        parse_mode="HTML"
    )


@router.callback_query(F.data.startswith("admin_status_"))
async def cb_admin_order_status(cb: CallbackQuery) -> None:
    _, _, order_id_raw, new_status = cb.data.split("_")
    order_id = int(order_id_raw)

    result = await update_order_status(order_id, cb.from_user.id, new_status)

    if not result or result.get("status") != "success":
        await cb.answer("Не удалось изменить статус заказа.", show_alert=True)
        return

    logger.info("Admin %s changed order %s status to %s", cb.from_user.id, order_id, new_status)
    await cb.answer(f"Статус обновлён: {STATUS_LABELS.get(new_status, new_status)}")
 
    orders = await get_active_orders()
    order = _find_order(orders or [], order_id)
 
    if order is None:
        return
 
    await cb.message.edit_text(
        build_order_detail_text(order),
        reply_markup=kb_order_detail(order_id, order["status"]),
        parse_mode="HTML",
    )


@router.callback_query(F.data.startswith("admin_ban_ask_"))
async def cb_admin_ban_ask(cb: CallbackQuery) -> None:
    order_id = int(cb.data.split("_")[3])
    await cb.answer()
    await cb.message.edit_text(
        f" Забанить клиента? Он больше не сможет оформлять заказы.",
        reply_markup=kb_ban_confirm(order_id),
    )


@router.callback_query(F.data.startswith("admin_ban_no"))
async def cb_admin_ban_no(cb: CallbackQuery) -> None:
    order_id = int(cb.data.split("_")[3])
    await cb.answer("Отменено")

    orders = await get_active_orders()
    order = _find_order(orders or [], order_id)

    if order is None:
        return

    await cb.message.edit_text(
        build_order_detail_text(order),
        reply_markup=kb_order_detail(order_id, order["status"]),
        parse_mode="HTML",
    )


@router.callback_query(F.data.startswith("admin_ban_yes"))
async def cb_admin_ban_yes(cb: CallbackQuery) -> None:
    order_id = int(cb.data.split("_")[3])
    
    orders = await get_active_orders()
    order = _find_order(orders or [], order_id)

    if order is None:
        await cb.aswer("Заказ не найден.", show_alert=True)
        return

    customer_tg_id = order.get("customer_telegram_id")
    if not customer_tg_id:
        await cb.answer("Клиент не нашелся.", show_alert=True)
        return

    result = await ban_user(
        admin_tg_id=cb.from_user.id,
        user_id=customer_tg_id,
        phone_number=order.get("phone_number"),
        ban_reason="Забанен администратором",
    )

    if not result or result.get("status") != "success":
        await cb.answer("Не удалось забанить клиента.", show_alert=True)
        return

    logger.info("Admin %s banned user %s (order %s)", cb.from_user.id, customer_tg_id, order_id)
    await cb.answer("Клиент забанен 🚫")
    await cb.message.edit_text(
        build_order_detail_text(order),
        reply_markup=kb_order_detail(order_id, order["status"]),
        parse_mode="HTML",
    )