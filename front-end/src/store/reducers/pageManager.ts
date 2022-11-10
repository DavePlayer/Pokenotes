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

export type Action = IInitPageManager | INextPage | IFirstPage | IPrevPage | ILastPage | IIndexedPage


export const PageManagerReducer = (state: IPageManager = initialPageManager, action: Action) => {
    console.log("changing page manager reducer")
    switch (action.type) {
        case actionType.INDEX:
            const diffrence = Math.ceil((state.length - state.maxElements))
            let start = action.payload - diffrence
            start = start > 0 ? start : 1
            let end = action.payload + diffrence
            end = end <= state.length ? end : state.length
            console.log(diffrence, "updating by index: ", `${start}  ${action.payload}  ${end}`)
            return {
                ...state,
                selectedPage: action.payload,
                startPage: start,
                endPage: end
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