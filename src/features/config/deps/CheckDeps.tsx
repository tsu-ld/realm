import type { DependencyCheckResult, DependencyType } from './types'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import useAppState from '../../app-state/hooks/useAppState'
import useSmoothProgress from '../../progress/hooks/useSmoothProgress'
import ProgressScreen from '../../progress/Screen'
import getMissingDeps from './utils/get-missing-deps'

interface Props {
  onDepsChecked: (deps: DependencyType[]) => void
}

export default function CheckDeps({ onDepsChecked }: Props) {
  const { setState } = useAppState()
  const [isComplete, setIsComplete] = useState(false)
  const progress = useSmoothProgress(isComplete ? 100 : 0)

  useEffect(() => {
    invoke<DependencyCheckResult[]>('check_deps')
      .then((results) => {
        const missing = getMissingDeps(results)
        onDepsChecked(missing)
        setIsComplete(true)
        if (missing.length > 0) {
          setState('DOWNLOADING')
        }
        else {
          setState('CONFIGURING_PLAYIT')
        }
      })
      .catch((e) => {
        console.error('Failed to check deps:', e)
      })
  }, [setState, onDepsChecked])

  return <ProgressScreen title="Checking dependencies..." log="Verifying system requirements..." progress={progress} />
}
