import React from 'react'
import { useSelector, useDispatch } from 'react-redux'
import { RootState } from '../../state/store'
import { toggleNavigation } from '../../state/uiSlice'
import { Header } from './Header'
import { Navigation } from './Navigation'
import { Overlay } from './Overlay'

interface AppShellProps {
  children: React.ReactNode
}

export function AppShell({ children }: AppShellProps) {
  const dispatch = useDispatch()
  const isNavigationOpen = useSelector((state: RootState) => state.ui.isNavigationOpen)

  const handleMenuToggle = () => {
    dispatch(toggleNavigation())
  }

  const handleOverlayClick = () => {
    if (isNavigationOpen) {
      dispatch(toggleNavigation())
    }
  }

  return (
    <div className="app-shell">
      <Header onMenuToggle={handleMenuToggle} />
      <div className="main-layout">
        <Navigation isOpen={isNavigationOpen} />
        <main className="content">
          {children}
        </main>
      </div>
      <Overlay isVisible={isNavigationOpen} onClick={handleOverlayClick} />
    </div>
  )
}
