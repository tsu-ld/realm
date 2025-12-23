import { invoke } from '@tauri-apps/api/core'
import Console from '@/shared/ui/Console'
import useAppState from '../app-state/hooks/useAppState'
import useServerLogs from './hooks/useServerLogs'

const STATUS_CONFIG: Record<string, { label: string, color: string }> = {
  READY: { label: 'Ready', color: 'bg-success' },
  STARTING: { label: 'Starting...', color: 'bg-warning' },
  RUNNING: { label: 'Running', color: 'bg-success' },
  STOPPING: { label: 'Stopping...', color: 'bg-error' },
}

export function DashboardScreen() {
  const { state, setState, ip } = useAppState()
  const logs = useServerLogs()
  const status = STATUS_CONFIG[state] || STATUS_CONFIG.READY

  const isRunning = state === 'RUNNING'
  const isTransitioning = state === 'STARTING' || state === 'STOPPING'

  function handleStart() {
    setState('STARTING')
    invoke('start_server')
      .then(() => setState('RUNNING'))
      .catch((e) => {
        console.error(e)
        setState('READY')
      })
  }

  function handleStop() {
    setState('STOPPING')
    invoke('stop_server')
      .then(() => setState('READY'))
      .catch((e) => {
        console.error(e)
        setState('RUNNING')
      })
  }

  return (
    <>
      <header className="flex items-center justify-between w-full">
        <h1 className="text-2xl font-bold text-foreground">Server dashboard</h1>
        <div className="flex items-center gap-4">

          <div className="flex items-center gap-2">
            <div className={`w-4 h-4  ${status.color}`} />
            <p className="-mb-3">
              {status.label}
            </p>
          </div>
        </div>
      </header>

      <Console logs={logs} />
      <p>
        {' '}
        Server IP:
        {' '}
        <span className="text-success font-bold select-all">
          {ip ?? '...'}
        </span>
      </p>
      <div className="flex gap-4 justify-center">
        {!isRunning
          ? (
              <button
                type="button"
                onClick={handleStart}
                disabled={isTransitioning}
              >
                Start server
              </button>
            )
          : (
              <button
                type="button"
                onClick={handleStop}
                disabled={isTransitioning}
              >
                Stop server
              </button>
            )}
      </div>
    </>
  )
}

export default DashboardScreen
