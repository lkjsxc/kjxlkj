import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface UiState {
  isNavigationOpen: boolean
  showCommandPalette: boolean
  showBacklinks: boolean
  showOutline: boolean
}

const initialState: UiState = {
  isNavigationOpen: false,
  showCommandPalette: false,
  showBacklinks: true,
  showOutline: false,
}

const uiSlice = createSlice({
  name: 'ui',
  initialState,
  reducers: {
    toggleNavigation(state) {
      state.isNavigationOpen = !state.isNavigationOpen
    },
    setNavigationOpen(state, action: PayloadAction<boolean>) {
      state.isNavigationOpen = action.payload
    },
    toggleCommandPalette(state) {
      state.showCommandPalette = !state.showCommandPalette
    },
    toggleBacklinks(state) {
      state.showBacklinks = !state.showBacklinks
    },
    toggleOutline(state) {
      state.showOutline = !state.showOutline
    },
  },
})

export const {
  toggleNavigation,
  setNavigationOpen,
  toggleCommandPalette,
  toggleBacklinks,
  toggleOutline,
} = uiSlice.actions

export default uiSlice.reducer
