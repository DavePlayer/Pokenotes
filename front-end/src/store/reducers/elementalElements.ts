import { IsEmptyObj } from "@reduxjs/toolkit/dist/tsHelpers"
import { elementalsActions as actionType } from "../actionTypes"

export interface IList {
    name: string,
    imgUrl?: string
}

const initialElementsList: Array<IList> = [{ name: "twoja mama" }]

interface ISet {
    type: actionType.SET,
    payload: Array<IList>
}

export type Action = ISet


export const ElementalListReducer = (state: Array<IList> = initialElementsList, action: Action) => {
    console.log("changing page manager reducer")
    switch (action.type) {
        case actionType.SET:
            state = action.payload
            return state
        default:
            return state
    }
}