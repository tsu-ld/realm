import { useEffect, useState } from 'react'

const INITIAL_PROGRESS = 0
const MAX_PROGRESS = 100
const DEFAULT_TARGET_IF_ZERO = 25
const UPDATE_INTERVAL_MS = 20
const MIN_STEP_INCREMENT = 0.2
const DIFF_MULTIPLIER = 0.1

export default function useSmoothProgress(target: number) {
  const [progress, setProgress] = useState(INITIAL_PROGRESS)

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= MAX_PROGRESS) {
          return MAX_PROGRESS
        }

        const effectiveTarget = target === 0 ? DEFAULT_TARGET_IF_ZERO : target
        const diff = effectiveTarget - prev

        if (diff <= 0) {
          return prev
        }

        const step = Math.max(MIN_STEP_INCREMENT, diff * DIFF_MULTIPLIER)
        return Math.min(effectiveTarget, prev + step)
      })
    }, UPDATE_INTERVAL_MS)

    return () => clearInterval(interval)
  }, [target])

  return progress
}
