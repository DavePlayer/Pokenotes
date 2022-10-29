import { PageMangerReducer } from './reducers/pageManager'
import { configureStore } from "@reduxjs/toolkit"
import { ResultType } from '@remix-run/router/dist/utils'

const reducer = {
    PageManager: PageMangerReducer
}
export const store = configureStore({
    reducer
})

export type storeState = ReturnType<() => typeof reducer>
