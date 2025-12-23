import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useEffect } from 'react'
import useAppState from '../app-state/hooks/useAppState'
import ProgressScreen from '../progress/Screen'

const EVENT_LOG = 'server-log'
const LOG_DONE = 'Done'
const CMD_START_SERVER = 'start_server'

export default function SetupServer() {
  console.log('Rendering SetupServer')
  const { setState } = useAppState()

  useEffect(() => {
    let unlisten: () => void

    const setup = async () => {
      console.log('Starting server setup...')
      try {
        unlisten = await listen<string>(EVENT_LOG, (event) => {
          console.log('Server Log:', event.payload)
          if (event.payload.includes(LOG_DONE)) {
            setState('RUNNING')
          }
        })

        await invoke(CMD_START_SERVER)
        invoke('start_playit').catch(e => console.error('Failed to start playit:', e))
      }
      catch (error) {
        console.error('Failed to setup server:', error)
      }
    }

    setup()

    return () => {
      if (unlisten)
        unlisten()
    }
  }, [setState])

  return (
    <ProgressScreen
      title="Preparing your server..."
      log="Starting Minecraft server..."
      progress={100}
    />
  )
}
