import useAppState from './features/app-state/hooks/useAppState'
import AppStateProvider from './features/app-state/Provider'
import ConfigScreen from './features/config/Screen'
import DashboardScreen from './features/dashboard/Screen'
import WelcomeScreen from './features/home/Screen'
import './App.css'

function AppRouter() {
  const { state } = useAppState()

  switch (state) {
    case 'INIT':
      return <WelcomeScreen />
    case 'CHECKING_DEPS':
    case 'DOWNLOADING':
    case 'CONFIGURING_PLAYIT':
    case 'SETUP_SERVER':
      return <ConfigScreen state={state} />
    case 'READY':
    case 'STARTING':
    case 'RUNNING':
    case 'STOPPING':
      return <DashboardScreen />
  }
}

function App() {
  return (
    <AppStateProvider>
      <AppRouter />
    </AppStateProvider>
  )
}

export default App
