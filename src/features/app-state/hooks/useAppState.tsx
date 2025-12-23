import type { AppStateContextType } from '../Context'
import { use } from 'react'
import AppStateContext from '../Context'

export default function useAppState(): AppStateContextType {
  const context = use(AppStateContext)

  return context
}
