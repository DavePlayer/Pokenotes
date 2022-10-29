import { IsEmptyObj } from "@reduxjs/toolkit/dist/tsHelpers"
import { elementalsActions as actionType } from "../actionTypes"

export interface IList {
    name: string,
    imgUrl?: string
}

const initialElementsList: Array<IList> = [{ name: "twoja mama" }]

interface ISet {
    type: actionType.SET,
    payload: { beg: number, end: number }
}

export type Action = ISet | { type: string }


export const ElementalListReducer = (state: Array<IList> = initialElementsList, action: Action) => {
    console.log("changing page manager reducer")
    switch (action.type) {
        case actionType.SET:
        default:
            return state
    }
}