use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub is_folder: bool,
    #[serde(default)]
    pub children: Vec<Entry>,
    pub last_modified: String,
    #[serde(rename = "type")]
    pub doc_type: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentTree {
    pub entries: Vec<Entry>,
    pub trash: Vec<Entry>,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Unauthorized: invalid credentials or expired token (status: 401)")]
    Unauthorized,

    #[error("Document already exists (status: 409)")]
    Conflict { doc_id: Option<String> },

    #[error("Server returned error status {status}: {message}")]
    HttpError {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
