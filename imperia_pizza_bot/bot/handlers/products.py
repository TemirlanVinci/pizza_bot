from aiogram import Router, F
from aiogram.types import CallbackQuery
from api.products import get_products
from api.favorites import get_favorites
from api.cart import cart_add
from keyboards.products import kb_product_actions
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

    desc = f"\n{product['description']}\n\n" if product.get("description") else "\n"
    weight = f"⚖️ {product['weight']} г\n\n" if product.get("weight") else ""

    text = f"<b>{product['name']}\n</b>{desc}{weight}💰 <b>{product['price']} с</b>"
    photo = product.get("image_url")
    markup = kb_product_actions(product_id, is_fav)

    await cb.answer()

    has_photo_now = bool(cb.message.photo)

    try:
        if photo:
            if has_photo_now:
                await cb.message.edit_media(
                    media=InputMediaPhoto(
                        media=photo,
                        caption=text,
                        parse_mode="HTML"
                    ),
                    reply_markup=markup
                )
            else:
                try:
                    await cb.message.delete()
                except TelegramBadRequest:
                    pass
                await cb.message.answer_photo(
                    photo=photo,
                    caption=text,
                    parse_mode="HTML",
                    reply_markup=markup
                )
        else:
            if has_photo_now:
                try:
                    await cb.message.delete()
                except TelegramBadRequest:
                    pass
                await cb.message.answer(text=text, reply_markup=markup, parse_mode="HTML")
            else:
                await cb.message.edit_text(text=text, reply_markup=markup, parse_mode="HTML")

    except TelegramBadRequest as e:
        if "message is not modified" in e.message:
            raise e
    

@router.callback_query(F.data.startswith("catprod_"))
async def cb_product(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[1])
    
    await show_products(cb, product_id)


@router.callback_query(F.data.startswith("cart_add_"))
async def cb_cart_add_from_catalog(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    res = await cart_add(cb.from_user.id, product_id)

    if res and res.get("status") == "success":
        await cb.answer("🛍️ Товар добавлен в корзину!", show_alert=False)
    else:
        await cb.answer("Не удалось добавить товар.", show_alert=True)
