import type { ReactNode } from 'react'
import type { AppState } from './Context'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useMemo, useState } from 'react'
import AppStateContext from './Context'

export default function AppStateProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<AppState>('INIT')
  const [ip, setIp] = useState<string | null>(null)

  useEffect(() => {
    let unlisten: () => void

    const setup = async () => {
      unlisten = await listen<string>('playit-ip', (event) => {
        console.log('Got Playit IP:', event.payload)
        setIp(event.payload)
      })
    }
    setup()

    return () => {
      if (unlisten) unlisten()
    }
  }, [])

  const value = useMemo(() => ({ state, setState, ip, setIp }), [state, ip])

  return <AppStateContext value={value}>{children}</AppStateContext>
}
