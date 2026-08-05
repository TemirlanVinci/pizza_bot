from aiogram import Router, F
from aiogram.types import CallbackQuery
from api.categories import get_categories
from api.products import get_products_by_category
from keyboards.categories import kb_categories
from keyboards.products import kb_products
from keyboards.main import kb_back_menu
from config import PRODUCTS_LIMIT
from aiogram.exceptions import TelegramBadRequest

router = Router()


async def show_menu(cb: CallbackQuery, offset: int = 0) -> None:
    categories = await get_categories()
    
    if not categories:
        try:
            if cb.message.photo:
                await cb.message.delete()
                await cb.message.answer(caption="❌ Не удалось загрузить меню. Попробуйте позже.", reply_markup=kb_back_menu())
            else:
                await cb.message.edit_text("❌ Не удалось загрузить меню. Попробуйте позже.", reply_markup=kb_back_menu())
        except TelegramBadRequest as e:
            if "message is not modified" in e.message:
                await cb.answer()
        return

    # UI-правка: Сделали заголовок визуально приятнее
    text = "📖 <b>Меню</b>\n\nВыберите нужную категорию, чтобы посмотреть блюда:"
    reply_markup = kb_categories(categories, offset)

    try:
        if cb.message.photo:
            try:
                await cb.message.delete()
            except TelegramBadRequest as e:
                pass
            await cb.message.answer(text=text, reply_markup=reply_markup)
        else:
            await cb.message.edit_text(text=text, reply_markup=reply_markup)
            
    except TelegramBadRequest as e:
        if "message is not modified" in e.message:
            await cb.message.edit_reply_markup(reply_markup=reply_markup)
            await cb.answer()
        else:
            raise e
        

@router.callback_query(F.data == "menu")
async def cb_menu(cb: CallbackQuery) -> None:
    await cb.answer()
    await show_menu(cb, offset=0)


@router.callback_query(F.data.startswith("menu_"))
async def cb_menu_page(cb: CallbackQuery) -> None:
    offset = int(cb.data.split("_")[1])
    await cb.answer()    
    await show_menu(cb, offset=offset)


@router.callback_query(F.data.startswith("cat_"))
async def cb_category(cb: CallbackQuery) -> None:
    parts = cb.data.split("_")
    cat_id = int(parts[1])
    offset = int(parts[2]) if len(parts) > 2 else 0
    await cb.answer()

    products = await get_products_by_category(cat_id, offset)
    
    if not products:
        if cb.message.photo:
            await cb.message.edit_caption(caption="❌ Не удалось загрузить товары. Попробуйте позже.", reply_markup=kb_back_menu())
        else:
            await cb.message.edit_text("❌ Не удалось загрузить товары. Попробуйте позже.", reply_markup=kb_back_menu())
        return

    cats = await get_categories()
    if isinstance(cats, dict):
        cats = cats.get("categories", list(cats.values()))

    cat_name = next(
        (c["name"] for c in (cats or []) if isinstance(c, dict) and c.get("id") == cat_id), 
        "Категория"
    )
    has_next = len(products) == PRODUCTS_LIMIT
    
    # UI-правка: Убрали дублирование "стр. X" в тексте и добавили призыв к действию
    text = f"🍕 <b>{cat_name}</b>\n\nВыберите блюдо для просмотра подробностей и добавления в корзину:"
    reply_markup = kb_products(products, cat_id, offset, has_next)

    if cb.message.photo:
        await cb.message.edit_caption(caption=text, reply_markup=reply_markup)
    else:
        await cb.message.edit_text(text, reply_markup=reply_markup)