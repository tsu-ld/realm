import ProgressBar from '@/shared/ui/ProgressBar'

interface Props {
  title?: string
  progress: number
  description?: string
  log?: string
}

export function ProgressScreen({ title = 'Processing', progress, description, log = 'Please wait while we set things up' }: Props) {
  return (
    <>
      <h1 className="text-3xl font-bold">{title}</h1>
      <p className="text-foreground-muted">{description}</p>

      <ProgressBar progress={progress} />

      <p className="text-foreground-muted text-sm">
        {log}
      </p>
    </>
  )
}

export default ProgressScreen
