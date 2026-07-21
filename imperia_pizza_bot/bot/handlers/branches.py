from aiogram import Router, F
from aiogram.types import CallbackQuery
from aiogram.fsm.context import FSMContext
from aiogram.filters.callback_data import CallbackData

from api.branches import get_branches, get_branch_detail
from keyboards.branches import kb_branches_list, kb_back_to_branches, BRANCHES_LIMIT

router = Router()

@router.callback_query(F.data == "branches_list")
@router.callback_query(F.data.startswith("branchpage_"))
async def cb_branches_page(cb: CallbackQuery, state: FSMContext) -> None:
    if cb.data == "branches_list":
        offset = 0
    else:
        offset = int(cb.data.split("_")[1])
        
    await state.update_data(branch_offset=offset)
        
    branches = await get_branches(limit=BRANCHES_LIMIT, offset=offset)

    if not branches and offset == 0:
        await cb.answer("Произошла ошибка при получении филиалов.", show_alert=True)
        return
    elif not branches:
        await cb.answer("Больше филиалов не найдено.", show_alert=True)
        return

    await cb.message.edit_text(
        text="📍 **Наши филиалы**\nВыберите удобный для вас филиал:",
        reply_markup=kb_branches_list(branches, offset),
        parse_mode="Markdown"
    )
    await cb.answer()


@router.callback_query(F.data.startswith("branch_"))
async def cb_branch_detail(cb: CallbackQuery, state: FSMContext) -> None:
    _, branch_id, active_flag = cb.data.split("_")
    
    if active_flag == "0":
        await cb.answer("Филиал временно недоступен", show_alert=True)
        return
        
    branch_detail = await get_branch_detail(int(branch_id))
    if not branch_detail:
        await cb.answer("Не удалось загрузить данные филиала.", show_alert=True)
        return
        
    user_data = await state.get_data()
    saved_offset = user_data.get("branch_offset", 0)
    
    text = (
        f"🏢 **Филиал:** {branch_detail.get('name')}\n"
        f"📍 **Адрес:** {branch_detail.get('address')}\n"
        f"🕒 **Режим работы:** {branch_detail.get('work_hours')}\n"
        f"📞 **Телефон:** {branch_detail.get('phone')}\n\n"
        f"🔗 [Открыть на карте 2ГИС]({branch_detail.get('map_link')})"
    )
    
    await cb.message.edit_text(
        text=text,
        reply_markup=kb_back_to_branches(saved_offset),
        parse_mode="Markdown",
        disable_web_page_preview=False
    )
    await cb.answer()


@router.callback_query(F.data == "noop")
async def cb_noop(cb: CallbackQuery) -> None:
    await cb.answer()