use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DependencyType {
    Java,
    ServerJar,
    Playit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DependencyStatus {
    Valid,
    Missing,
    Invalid(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DependencyCheckResult {
    pub dependency: DependencyType,
    pub status: DependencyStatus,
}

#[derive(Error, Debug)]
pub enum DependencyError {
    #[error("Failed to download {0}: {1}")]
    DownloadFailed(String, String),
    #[error("Failed to extract {0}: {1}")]
    ExtractionFailed(String, String),
    #[error("Validation failed for {0}: {1}")]
    ValidationFailed(String, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}
