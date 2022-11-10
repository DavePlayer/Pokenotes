import { ElementalListReducer } from './reducers/elementalElements'
import { configureStore } from "@reduxjs/toolkit"
import { PageManagerReducer } from './reducers/pageManager'

export const store = configureStore({
    reducer: {
        elementalList: ElementalListReducer,
        pageManager: PageManagerReducer
    },
    devTools: true
})

export type storeState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
