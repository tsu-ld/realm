export type DependencyType = 'Java' | 'ServerJar' | 'Playit'

export type DependencyStatus
  = | 'Valid'
    | 'Missing'
    | { Invalid: string }

export interface DependencyCheckResult {
  dependency: DependencyType
  status: DependencyStatus
}
