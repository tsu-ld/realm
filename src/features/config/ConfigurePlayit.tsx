import type { ConfigurePlayitResult } from './types'
import { invoke } from '@tauri-apps/api/core'
import { useState } from 'react'
import useAppState from '../app-state/hooks/useAppState'

type Status = 'idle' | 'connecting' | 'error'

export default function ConfigurePlayit() {
  const { setState } = useAppState()
  const [status, setStatus] = useState<Status>('idle')
  const [error, setError] = useState<string | null>(null)

  async function handleConnect() {
    setStatus('connecting')
    setError(null)

    try {
      const result = await invoke<ConfigurePlayitResult>('configure_playit')

      if ('AlreadyConfigured' in result) {
        setState('SETUP_SERVER')
      }
      else if ('NeedsAuth' in result) {
        setState('SETUP_SERVER')
      }
      else if ('Error' in result) {
        setError(result.Error)
        setStatus('error')
      }
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
      setStatus('error')
    }
  }

  function handleSkip() {
    setState('SETUP_SERVER')
  }

  return (
    <>
      <h1 className="text-3xl font-bold">Connect with playit</h1>

      <p className="text-foreground-muted">
        To allow your friends to connect from the internet, we use playit.gg.
        It is a secure service that avoids router configuration.
      </p>

      <div className="flex flex-col gap-2">
        {error && <p className="text-error">{error}</p>}
        <button
          onClick={handleConnect}
          disabled={status === 'connecting'}
        >
          {status === 'connecting' ? 'Connecting...' : 'Connect with playit'}
        </button>

        <button
          onClick={handleSkip}
          disabled={status === 'connecting'}
        >
          Skip for now, I already have ports forwarded
        </button>
      </div>
    </>
  )
}
