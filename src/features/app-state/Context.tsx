import { createContext } from 'react'

export type AppState
  = | 'INIT'
    | 'CHECKING_DEPS'
    | 'CONFIGURING_PLAYIT'
    | 'DOWNLOADING'
    | 'CONFIGURING_PLAYIT'
    | 'SETUP_SERVER'
    | 'READY'
    | 'STARTING'
    | 'RUNNING'
    | 'STOPPING'

export interface AppStateContextType {
  state: AppState
  setState: (state: AppState) => void
  ip: string | null
  setIp: (ip: string | null) => void
}

const AppStateContext = createContext<AppStateContextType>({
  state: 'INIT',
  setState: () => { },
  ip: null,
  setIp: () => { },
})

export default AppStateContext
