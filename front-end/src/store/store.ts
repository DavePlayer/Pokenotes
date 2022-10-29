import { ElementalListReducer } from './reducers/elementalElements'
import { configureStore } from "@reduxjs/toolkit"

const reducer = {
    elementalList: ElementalListReducer
}
export const store = configureStore({
    reducer
})

export type storeState = ReturnType<() => typeof reducer>
