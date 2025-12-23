import useAppState from '../app-state/hooks/useAppState'

export function WelcomeScreen() {
  const { setState } = useAppState()

  return (
    <>
      <h1 className="text-7xl">Realm</h1>
      <p>
        Host your own Minecraft server in seconds
      </p>
      <p>
        No configuration needed. Just click and play.
      </p>
      <button type="button" className="w-full" onClick={() => setState('CHECKING_DEPS')}>Get started</button>
    </>
  )
}

export default WelcomeScreen
