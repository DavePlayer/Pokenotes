import { ElementalListReducer } from './reducers/elementalElements'
import { configureStore } from "@reduxjs/toolkit"
import { PageManagerReducer } from './reducers/pageManager'

const reducer = {
    elementalList: ElementalListReducer,
    pageManager: PageManagerReducer
}
export const store = configureStore({
    reducer
})

export type storeState = ReturnType<() => typeof reducer>
