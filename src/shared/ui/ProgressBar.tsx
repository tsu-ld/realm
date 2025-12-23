interface ProgressBarProps {
  progress: number
}

export function ProgressBar({ progress }: ProgressBarProps) {
  const clampedProgress = Math.min(100, Math.max(0, progress))

  return (
    <div className="w-full space-y-2">
      <div className="h-6 overflow-hidden z-1 bezel">
        <div
          className="h-full relative"
          style={{ width: `${clampedProgress}%` }}
        >
          <div className="absolute inset-0 bg-success -z-1" />
        </div>
      </div>
      <p className="text-sm text-foreground-muted mt-1 text-right">
        {clampedProgress.toFixed(0)}
        %
      </p>
    </div>
  )
}

export default ProgressBar
