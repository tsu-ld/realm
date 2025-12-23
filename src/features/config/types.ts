export type ConfigurePlayitResult
  = | { AlreadyConfigured: { tunnel_address: string } }
    | { NeedsAuth: { claim_url: string } }
    | { Error: string }
