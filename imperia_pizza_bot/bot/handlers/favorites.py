from aiogram import Router, F
from aiogram.types import CallbackQuery
from api.favorites import get_favorites, add_favorite, delete_favorite
from keyboards.favorites import kb_favorites
from keyboards.main import kb_back_menu
from handlers.products import show_products

router = Router()


@router.callback_query(F.data == "favorites")
async def cb_favorites(cb: CallbackQuery) -> None:
    await cb.answer()
    favs = await get_favorites(cb.from_user.id)

    has_photo = bool(cb.message.photo)

    if favs is None:
        if has_photo:
            await cb.message.delete()
            await cb.message.answer("Ошибки загрузки избранного.", reply_markup=kb_back_menu())
        else:
            await cb.message.edit_text("Ошибки загрузки избранного.", reply_markup=kb_back_menu())
        return
    
    if not favs:
        text_empty = "🤍 <b>Избранное пусто</b>\n\nДобавляйте товары через меню"
        if has_photo:
            await cb.message.delete()
            await cb.message.answer(text_empty, reply_markup=kb_back_menu())
        else:
            await cb.message.edit_text(text_empty, reply_markup=kb_back_menu())
        return
    
    text_fav = "❤️ <b>Ваше избранное:</b>\n\nНажмите на блюдо, чтобы перейти к заказу."
    if has_photo:
        await cb.message.delete()
        await cb.message.answer(text_fav, reply_markup=kb_favorites(favs))
    else:
        await cb.message.edit_text(text_fav, reply_markup=kb_favorites(favs))


@router.callback_query(F.data.startswith("prod_view_"))
async def cb_view_favorite_product(cb: CallbackQuery) -> None:
    await cb.answer()
    product_id = int(cb.data.split("_")[2])
    
    await show_products(cb, product_id)

@router.callback_query(F.data.startswith("fav_add_"))
async def cb_fav_add(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    res = await add_favorite(cb.from_user.id, product_id)

    if res and res.get("status") == "success":
        await cb.answer("❤️ Добавлено в избранное", show_alert=True)
        await show_products(cb, product_id)
    else:
        await cb.answer("Произошла ошибка.", show_alert=True)


@router.callback_query(F.data.startswith("fav_del_"))
async def cb_fav_del(cb: CallbackQuery) -> None:
    
    product_id = int(cb.data.split("_")[2])
    res = await delete_favorite(cb.from_user.id, product_id)

    if res and res.get("status") == "success":
        await cb.answer("🤍 Удалено из избранного", show_alert=True)
        await show_products(cb, product_id)
    else:
        await cb.answer("Произошла ошибка.", show_alert=True)