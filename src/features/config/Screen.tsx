import type { AppState } from '../app-state/Context'
import type { DependencyType } from './deps/types'
import { useState } from 'react'
import ConfigurePlayit from './ConfigurePlayit'
import CheckDeps from './deps/CheckDeps'
import DownloadDeps from './deps/DownloadDeps'
import SetupServer from './SetupServer'

interface Props {
  state: AppState
}

export default function ConfigScreen({ state }: Props) {
  const [missingDeps, setMissingDeps] = useState<DependencyType[]>([])

  switch (state) {
    case 'CHECKING_DEPS':
      return <CheckDeps onDepsChecked={setMissingDeps} />
    case 'DOWNLOADING':
      return <DownloadDeps missingDeps={missingDeps} />
    case 'CONFIGURING_PLAYIT':
      return <ConfigurePlayit />
    case 'SETUP_SERVER':
      return <SetupServer />
    default:
      return null
  }
}
