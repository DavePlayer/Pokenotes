import { PageManagerActions as actionType } from "../actionTypes"

interface IPageManager {
    selectedPage: number,
    startPage: number,
    endPage: number,
    length: number
}

const initialPageManager: IPageManager = {
    selectedPage: 0,
    length: 1,
    startPage: 0,
    endPage: 1
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
        case actionType.INIT:
        case actionType.NEXT:
        case actionType.PREV:
        case actionType.FIRST:
        case actionType.LAST:
        case actionType.INDEX:
        default:
            return state
    }
}