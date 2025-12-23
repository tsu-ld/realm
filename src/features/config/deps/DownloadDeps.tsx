import type { DependencyType } from './types'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'
import useSmoothProgress from '@/features/progress/hooks/useSmoothProgress'
import useAppState from '../../app-state/hooks/useAppState'
import ProgressScreen from '../../progress/Screen'

interface Props {
  missingDeps: DependencyType[]
}

const TASK_INFO: Record<string, { title: string, log: string }> = {
  Java: { title: 'Downloading Java', log: 'Getting the Java runtime...' },
  ServerJar: { title: 'Downloading Server', log: 'Getting the server file from Paper...' },
  Playit: { title: 'Downloading Connection Tool', log: 'Getting the tool to let friends join...' },
}

export default function DownloadDeps({ missingDeps }: Props) {
  const { setState } = useAppState()
  const [downloadedCount, setDownloadedCount] = useState(0)

  useEffect(() => {
    if (missingDeps.length === 0) {
      setState('CONFIGURING_PLAYIT')
      return
    }

    let unlisten: (() => void) | undefined

    const setupListener = async () => {
      unlisten = await listen<DependencyType>('dep-downloaded', () => {
        setDownloadedCount(prev => prev + 1)
      })
    }
    setupListener()

    invoke('download_deps', { deps: missingDeps })
      .then(() => { setState('CONFIGURING_PLAYIT') })
      .catch((e) => {
        console.error('Failed to download deps:', e)
      })

    return () => {
      if (unlisten)
        unlisten()
    }
  }, [setState, missingDeps])

  const targetProgress = missingDeps.length > 0 ? (downloadedCount / missingDeps.length) * 100 : 0
  const progress = useSmoothProgress(targetProgress)

  const currentDep = missingDeps[Math.min(downloadedCount, missingDeps.length - 1)]

  const { title, log } = TASK_INFO[currentDep]

  return (
    <ProgressScreen
      title={title}
      description="We are setting up your server environment"
      log={log}
      progress={progress}
    />
  )
}
