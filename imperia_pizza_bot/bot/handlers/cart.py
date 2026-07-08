from aiogram import Router, F
from aiogram.types import CallbackQuery
from api.cart import get_cart, cart_add, cart_decrement, cart_delete 
from keyboards.cart import kb_cart
from keyboards.main import kb_back_menu

router = Router()


def build_cart_text(cart: dict) -> str:
    lines = ["🛒 <b>Ваша корзина:</b>\n"]
    for item in cart["items"]:
        lines.append(
            f"<b>{item['name']}</b>\n"
            f"💰 {item['price']} c × {item['quantity']} шт = <b>{item['total_item_price']} c</b>\n"
        )
    lines.append(
        f"——————————————————————\n"
        f"Всего товаров: <b>{cart['total_quantity']}</b>\n"
        f"Итого к оплате: <b>{cart['final_price']} c</b>"
    )
    return "\n".join(lines)


async def show_cart(cb: CallbackQuery) -> None:
    cart = await get_cart(cb.from_user.id)
    
    async def render_message(text: str, markup):
        if cb.message.photo or cb.message.video or cb.message.document:
            await cb.message.delete()
            await cb.message.answer(text, reply_markup=markup, parse_mode="HTML")
        else:
            await cb.message.edit_text(text, reply_markup=markup, parse_mode="HTML")

    if cart is None:
        await render_message("❌ Ошибка загрузки корзины.", kb_back_menu())
        return

    items = cart.get("items", [])    
    if not items:
        await render_message("🛒 <b>Корзина пуста</b>\n\nДобавляйте товары через меню!", kb_back_menu())
        return
    
    await render_message(build_cart_text(cart), kb_cart(items))

@router.callback_query(F.data == "cart")
async def cb_cart(cb: CallbackQuery) -> None:
    await cb.answer()
    await show_cart(cb)


@router.callback_query(F.data.startswith("cart_inc_"))
async def cb_cart_inc(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    
    res = await cart_add(cb.from_user.id, product_id)
    
    if res and res.get("status") == "success":
        await cb.answer() 
        await show_cart(cb)  
    else:
        await cb.answer("Произошла ошибка при добавлении.", show_alert=True)


@router.callback_query(F.data.startswith("cart_dec_"))
async def cb_cart_dec(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    
    res = await cart_decrement(cb.from_user.id, product_id)
    
    if res and res.get("status") == "success":
        await cb.answer()
        await show_cart(cb)  
    else:
        await cb.answer("Произошла ошибка.", show_alert=True)


@router.callback_query(F.data.startswith("cart_del_"))
async def cb_cart_del(cb: CallbackQuery) -> None:
    product_id = int(cb.data.split("_")[2])
    
    res = await cart_delete(cb.from_user.id, product_id)

    if res and res.get("status") == "success":
        await cb.answer("🗑️ Удалено из корзины")
        await show_cart(cb)  
    else:
        await cb.answer("Произошла ошибка при удалении.", show_alert=True)


@router.callback_query(F.data.startswith("cart_count_"))
async def cb_cart_count_click(cb: CallbackQuery) -> None:
    await cb.answer("Используйте ➖ и ➕ для изменения количества", show_alert=False)