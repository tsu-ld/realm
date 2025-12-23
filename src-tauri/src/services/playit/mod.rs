mod process;
mod output_parser;
mod browser;

pub use browser::open_claim_url;
pub use process::{run_playit, start, stop};
pub use output_parser::PlayitOutput;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurePlayitResult {
    AlreadyConfigured { tunnel_address: String },
    NeedsAuth { claim_url: String },
    Error(String),
}

use std::sync::Mutex;
use std::process::Child;

pub struct PlayitState {
    pub child: Mutex<Option<Child>>,
}

impl Default for PlayitState {
    fn default() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }
}
