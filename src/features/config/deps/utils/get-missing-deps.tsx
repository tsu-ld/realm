import type { DependencyCheckResult, DependencyType } from '../types'

export default function getMissingDeps(results: DependencyCheckResult[]): DependencyType[] {
  return results
    .filter(r => r.status === 'Missing')
    .map(r => r.dependency)
}
