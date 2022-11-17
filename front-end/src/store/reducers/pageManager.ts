import { PageManagerActions as actionType } from "../actionTypes"

interface IPageManager {
    selectedPage: number,
    startPage: number,
    endPage: number,
    maxElements: number,
    length: number
}

let initialPageManager: IPageManager = {
    selectedPage: 1,
    length: 7,
    startPage: 1,
    maxElements: 5,
    endPage: 5
}

interface IInitPageManager {
    type: actionType.INIT,
    payload: number // number of pages
}
interface INextPage {
    type: actionType.NEXT,
}
interface IPrevPage {
    type: actionType.PREV,
}
interface IFirstPage {
    type: actionType.FIRST,
}
interface ILastPage {
    type: actionType.LAST,
}
interface IIndexedPage {
    type: actionType.INDEX,
    payload: number
}
interface IsetPagesAmount {
    type: actionType.SETLENGTH,
    payload: number
}

export type Action = IInitPageManager | INextPage | IFirstPage | IPrevPage | ILastPage | IIndexedPage | IsetPagesAmount


const getPageRange = (action: number, state: IPageManager) => {
    const diffrence = Math.floor(state.maxElements / 2)
    let start = action - diffrence
    start = start > 0 ? start : 1
    let end = action + diffrence
    console.log("end begore: ", end)
    end = end <= state.length ? end : state.length
    if (state.selectedPage == 2) end += Math.floor(diffrence / 2)
    if (state.selectedPage == 1 && state.length > 2) end += Math.floor(diffrence)
    if (state.selectedPage == state.length - 1) start -= Math.floor(diffrence / 2)
    if (state.selectedPage == state.length && state.length > 2) start -= diffrence
    console.log(diffrence, "updating by index: ", `${start}  ${action}  ${end}`, state)
    return {
        selectedPage: action,
        endPage: end,
        startPage: start
    }
}

export const PageManagerReducer = (state: IPageManager = initialPageManager, action: Action) => {
    switch (action.type) {
        case actionType.INDEX:
            console.log("changing page manager reducer by index")
            const newManager = {
                ...state,
                ...getPageRange(action.payload, state)
            }
            return newManager
        case actionType.SETLENGTH:
            console.log("changing page manager reducer by set length")
            const maxElementsToDisplay = 40;
            const length = Math.ceil(action.payload / maxElementsToDisplay)
            let maxElements = length
            if (length > 5) maxElements = 5
            return {
                ...state,
                ...getPageRange(state.selectedPage, state),
                maxElements,
                length: length
            }
        case actionType.INIT:
        case actionType.NEXT:
        case actionType.PREV:
        case actionType.FIRST:
        case actionType.LAST:
        default:
            return state
    }
}