from aiogram import Router, F
from aiogram.types import CallbackQuery
from api.products import get_products
from api.favorites import get_favorites
from api.cart import cart_add
from keyboards.products import kb_product_actions
from keyboards.main import kb_back_menu
from aiogram.exceptions import TelegramBadRequest
from aiogram.types import InputMediaPhoto


router = Router()


async def show_products(cb: CallbackQuery, product_id: int) -> None:
    product = await get_products(product_id)
    if not product:
        await cb.answer("Товар не найден.", show_alert=True)
        return

    favs = await get_favorites(cb.from_user.id)
    fav_ids = {f["id"] for f in (favs or [])}
    is_fav = product_id in fav_ids

    icon = "❤️" if is_fav else "🤍"
    desc = f"\n📝 {product['description']}\n" if product.get("description") else "\n"
    weight = f"⚖️ {product['weight']} г\n" if product.get("weight") else ""

    text = f"{icon} <b>{product['name']}</b>{desc}{weight}💰 <b>{product['price']} с</b>"
    photo = product.get("main_image")
    markup = kb_product_actions(product_id, is_fav)

    # Убираем состояние "часиков" с нажатой кнопки
    await cb.answer()

    # Сначала гарантированно удаляем старое сообщение
    try:
        await cb.message.delete()
    except TelegramBadRequest:
        pass # Игнорируем, если сообщение уже было удалено

    # Отправляем новое сообщение (с фото или без)
    if photo:
        await cb.message.answer_photo(
            photo=photo,
            caption=text,
            reply_markup=markup,
            parse_mode="HTML"
        )
    else:
        await cb.message.answer(
            text=text,
            reply_markup=markup,
            parse_mode="HTML"
        )

@router.callback_query(F.data.startswith("catprod_"))
async def cb_product(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[1])
    await cb.answer()
    await show_products(cb, product_id)


@router.callback_query(F.data.startswith("cart_add_"))
async def cb_cart_add_from_catalog(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    res = await cart_add(cb.from_user.id, product_id)

    if res and res.get("status") == "success":
        await cb.answer("🛍️ Товар добавлен в корзину!", show_alert=False)
    else:
        await cb.answer("Не удалось добавить товар.", show_alert=True)
