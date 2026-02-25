import { configureStore } from '@reduxjs/toolkit'
import noteReducer from './noteSlice'
import uiReducer from './uiSlice'

export const store = configureStore({
  reducer: {
    note: noteReducer,
    ui: uiReducer,
  },
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
