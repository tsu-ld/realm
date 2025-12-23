import { useEffect, useRef } from 'react'

interface ConsoleProps {
  logs?: string[]
}

const DEFAULT_LOGS = [
  '[Server] Waiting for server to start...',
  '[Info] Console output will appear here',
]

export function Console({ logs = DEFAULT_LOGS }: ConsoleProps) {
  const bottomRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [logs])

  return (
    <div className="w-full h-64 bg-stone-800 bezel overflow-hidden flex flex-col">
      <div className="p-3 overflow-y-auto text-white">
        {logs.map((log, index) => (
          <p key={index} className="text-sm text-white/80">
            <span className="select-none text-white/50 mr-2">&gt;</span>
            {log}
          </p>
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  )
}

export default Console
