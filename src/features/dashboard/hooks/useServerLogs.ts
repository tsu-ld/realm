import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

export default function useServerLogs() {
  const [logs, setLogs] = useState<string[]>([])

  useEffect(() => {
    let unlisten: () => void

    const setup = async () => {
      unlisten = await listen<string>('server-log', (event) => {
        setLogs(prev => [...prev, event.payload])
      })
    }

    setup()

    return () => {
      if (unlisten)
        unlisten()
    }
  }, [])

  return logs
}
