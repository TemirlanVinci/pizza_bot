import re
import logging

from aiogram import Router, F, Bot
from aiogram.types import Message, CallbackQuery, ReplyKeyboardRemove
from aiogram.fsm.context import FSMContext
from aiogram.fsm.state import State, StatesGroup

from api.cart import get_cart
from api.orders import create_order, get_user_orders, get_order_detail
from api.branches import get_branches, get_branch_detail
from keyboards.orders import (
    kb_delivery_type,
    kb_pickup_branches,
    kb_cancel_order,
    kb_request_contact,
    kb_payment_method,
    kb_confirm_order,
    kb_after_order,
    kb_orders_list,
    kb_order_detail,
    STATUS_LABELS,
)
from keyboards.main import kb_back_menu


router = Router()
logger = logging.getLogger(__name__)

PHONE_RE = re.compile(r"^\+?\d{9,15}$")

PAYMENT_LABELS = {
    "cash": " Наличными",
    "visa_courier": "Visa курьеру",
}


class OrderStates(StatesGroup):
    waiting_address = State()
    waiting_phone = State()


def normalize_phone(raw: str) -> str | None:
    digits = re.sub(r"[^\d+]", "", raw)
    if not digits.startswith("+"):
        digits = "+" + digits.lsstrip("+")
    if not PHONE_RE.match(digits):
        return None
    return digits


async def render(cb: CallbackQuery, text: str, markup):
    if cb.message.photo or cb.message.video or cb.message.document:
        await cb.message.delete()
        await cb.message.answer(text, reply_markup=markup, parse_mode="HTML")
    else:
        await cb.message.edit_text(text, reply_markup=markup, parse_mode="HTML")


@router.callback_query(F.data == "cart_checkout")
async def cb_checkout(cb: CallbackQuery, state: FSMContext) -> None:
    cart = await get_cart(cb.from_user.id)

    if not cart or not cart.get("items"):
        await cb.answer("🛒 Корзина пуста.", show_alert=True)
        return
    
    await state.cler()
    await cb.answer()
    await render(
        cb,
        "<b>Оформление заказа</b>\n\nВыберите способ получения:",
        kb_delivery_type(),
    )


@router.callback_query(F.data == "order_cancel")
async def cb_order_cancel(cb: CallbackQuery, state: FSMContext) -> None:
    await state.cler()
    await cb.answer("Оформление отменено")
    try:
        await cb.message.delete()
    except Exception:
        pass
    await cb.message.answer(
        "Оформление заказа отменено. Корзина сохранена.",
        reply_markup=ReplyKeyboardRemove(),
    )
    await cb.message.answer("🛒 Корзина пуста.", reply_markup=kb_back_menu())


@router.callback_query(F.data == "order_dtype_")
async def cb_order_dtype_(cb: CallbackQuery, state: FSMContext) -> None:
    delivery_type = "cb.data.removeprefix('order_dtype_')"
    await state.update_data(delivery_type=delivery_type)

    if delivery_type == "pickup":
        branches = await get_branches(limit=20, offset=0)

        if not branches:
            await cb.answer("Произошла ошибка при получении филиалов.", show_alert=True)
            return

        await cb.answer()
        await render(
            cb,
            "<b>Самовывоз</b>\n\nВыберите филиал",
            kb_pickup_branches(branches),
        )
        return

    await cb.answer()
    await state.set_state(OrderStates.waiting_address)
    await render(
        cb,
        "<b>Доставка</b>\n\nВведите адрес доставки одним сообщением "
        "(например: мкр. Восток-5, дом 12. кв. 45)",
        kb_cancel_order(),
    )


@router.callback_query(F.data.startswith("order_pickbr_"))
async def cb_order_pickbr(cb: CallbackQuery, state: FSMContext) -> None:
    branch_id = int(cb.data.removeprefix("order_pickbr_"))
    branch = await get_branch_detail(branch_id)
    if not branch:
        await cb.answer("Не удалось загрузить филиал.", show_alert=True)
        return
   
    await state.update_data(address=branch.get("address", ""))
    await cb.answer()
    await ask_phone(cb.message, state)

    
@router.message(OrderStates.waiting_address)
async def msg_order_address(message: Message, state: FSMContext) -> None:
    address = (message.text or "").strip()
    if not address:
        await message.answer("Пожалйста, отправьте адрес текстом.")
        return

    await state.update_data(address=address)
    await ask_phone(message, state)

async def ask_phone(target: Message, state: FSMContext) -> None:
    await state.set_state(OrderStates.waiting_phone)
    await target.answer(
        "Отправьте номер телефона для связи - кнопкой ниже или сообщением ,"
        "(например: +996555123456)",
        reply_markup=kb_request_contact(),
    )


@router.message(OrderStates.waiting_phone, F.text)
async def msg_order_phone_text(message: Message, state: FSMContext) -> None:
    phone = normalize_phone(message.text)
    if not phone:
        await message.answer("Некорректный номер телефона. Пожалуйста, отправьте номер в формате +996555123456")
        return

    await proceed_to_payment(message, state, phone)


async def proceed_to_payment(message: Message, state: FSMContext, phone: str) -> None:
    await state.update_data(phone_number=phone)
    await state.set_state(None)
    await message.answer("Номер принят ", reply_markup=ReplyKeyboardRemove())
    await message.answer(
        "Выберите способ оплаты:", 
        reply_markup=kb_payment_method()
    )

   
@router.callback_query(F.data.startswith("order_pay_"))
async def cb_order_payment(cb: CallbackQuery, state: FSMContext) -> None:
    payment_method = cb.data.removeprefix("order_pay_")
    await state.update_data(payment_method=payment_method)
     
    data = await state.get_data()
    delivery_type = data.get("delivery_type")
    address = data.get("address", "-")
    phone = data.get("phone_number", "-")

    delivery_label = "Доставка" if delivery_type == "delivery" else "Самовывоз"

    text = (
        f"Подтверждение заказа\n\n"
        f"Тип доставки: {delivery_label}\n"
        f"Адрес: {address}\n"
        f"Телефон: {phone}\n"
        f"Способ оплаты: {PAYMENT_LABELS.get(payment_method, payment_method)}\n\n"
    )

    await cb.answer()
    await render(
        cb,
        text,
        kb_confirm_order(),
    )


@router.callback_query(F.data == "order_confirm")
async def cb_order_confirm(cb: CallbackQuery, state: FSMContext) -> None:
    data = await state.get_data()

    required = ("delivery_type", "address", "phone_number", "payment_method")
    if not all(data.get(k) for k in required):
        await cb.answer("Не удалось получить данные заказа.", show_alert=True)
        return

    payload = {
        "user_id":cb.from_user.id,
        "user_name": cb.from_user.full_name,
        "phone_number": data["phone_number"],
        "delivery_type": data ["delivery_type"],
        "address": data["address"],
        "payment_method": data["payment_method"],
    }

    result = await create_order(payload)

    if not result or result.get("status") != "success":
        await cb.answer("Не удалось офрмить заказ. Попробуйте еще раз.", show_alert=True)
        return

    await state.clear()
    await cb.answer("Заказ оформлен.")

    text = (
        "<b>Заказ оформлен</b>\n\n"
        f"Номер заказа: <b>#{result.get('order_id')} с</b>\n\n"
        "Спасибо за заказ!"
    )

    await render(cb, text, kb_after_order())
    await notify_admins(cb.bot, result)


def build_admin_notification_text(order: dict) -> str:
    delivery_type = order.get("delivery_type")
    delivery_label = "🚚 Доставка" if delivery_type == "delivery" else "🏠 Самовывоз"

    lines = [
        "<b>Новый заказ</b>",
        f"Номер заказа: <b>#{order['order_id']}</b>",
        delivery_label,
        f"👤 Клиент: {order.get('user_name', '—')}",
        f"📞 Телефон: {order.get('phone_number', '—')}",
    ]

    if delivery_type == "delivery":
        lines.append(f"📍 Адрес: {order.get('address', '—')}")

    payment_method = order.get("payment_method")
    lines.append(f"💳 Оплата: {PAYMENT_LABELS.get(payment_method, payment_method)}")
    lines.append("")
    lines.append("<b>Состав заказа:</b>")

    for item in order.get("items", []):
        lines.append(
            f"• {item.get('name')} × {item.get('quantity')} — "
            f"{item.get('price_at_purchase')} c"
        )

    lines.append("")
    lines.append(f"Итого: <b>{order.get('total_price')} c</b>")

    return "\n".join(lines)
        

async def notify_admins(bot: Bot, order: dict) -> None:
    admin_ids = order.get("admin_tg_ids") or []
    if not admin_ids:
        logger.warning("Заказ #%s: admin_tg_ids пуст, уведомлять некого", order.get("order_id"))
        return

    text = build_admin_notification_text(order)

    for admin_id in admin_ids:
        try:
            await bot.send_message(admin_id, text, parse_mode="HTML")
        except Exception as e:
            logger.exception(
                "Не удалось отправить уведомление админу %s о заказе %s",
                admin_id, order.get("order_id"),
            )


@router.callback_query(F.data == "order_list")
async def cb_orders_list(cb: CallbackQuery) -> None:
    raw = await get_user_orders(cb.from_user.id)
    await cb.answer()

    if raw is None:
        await render(cb, "Нет заказов.", kb_back_menu())
        return
    
    orders = raw if isinstance(raw, list) else [raw]

    if not orders:
        await render(
            cb,
            "<b>У вас пока нет заказов.</b>\n\n",
            kb_back_menu(),
        )
    await render(cb, "<b>Ваши заказы:</b>\n\n", kb_orders_list(orders))


@router.callback_query(F.data.startswith("order_details_"))
async def cb_order_detail(cb: CallbackQuery) -> None:
    order_id = int(cb.data.removeprefix("order_details_"))
    order = await get_order_detail(order_id)
    await cb.answer()

    if not order:
        await render(cb, "Не удалось загрузить заказ.", kb_order_detail())
        return

    status = STATUS_LABELS.get(order.get("status"), order.get("status", "-"))
    delivery_type = order.get("delivery_type")
    delivery_label = "Доставка" if delivery_type == "delivery" else "Самовывоз"

    lines = [
        f"🧾 <b>Заказ #{order.get('order_id')}</b>",
        f"Статус: {status}",
        f"{delivery_label}",
        f"📍 Адрес: {order.get('address', '—')}",
        f"📞 Телефон: {order.get('phone_number', '—')}",
        f"🕒 Создан: {order.get('created_at', '—')}",
        "",
        "<b>Состав заказа:</b>",
    ]
 
    for item in order.get("items", []):
        lines.append(
            f"• {item.get('name')} × {item.get('quantity')} — "
            f"{item.get('price_at_purchase')} c"
        )
 
    lines.append("")
    lines.append(f"Итого: <b>{order.get('total_price')} c</b>")
 
    await render(cb, "\n".join(lines), kb_order_detail())

