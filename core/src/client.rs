use std::sync::{Arc, Mutex};
use std::path::Path;
use reqwest::blocking::{Client, multipart};
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::model::{DocumentTree, ApiError};
use crate::session::RmfcSession;

pub struct RmfcClient {
    session: Arc<Mutex<RmfcSession>>,
    client: Client,
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct ConflictResponseBody {
    #[serde(rename = "docId")]
    doc_id: Option<String>,
}

impl RmfcClient {
    pub fn new(session: RmfcSession) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            session: Arc::new(Mutex::new(session)),
            client,
        }
    }

    pub fn session(&self) -> Arc<Mutex<RmfcSession>> {
        self.session.clone()
    }

    /// Force authenticating and saving the token.
    pub fn login(&self) -> Result<String, ApiError> {
        let (host, email, password) = {
            let sess = self.session.lock().unwrap();
            (sess.host.clone(), sess.email.clone(), sess.password.clone())
        };

        let url = format!("{}/ui/api/login", host);
        let payload = LoginRequest {
            email: &email,
            password: &password,
        };

        let response = self.client.post(&url)
            .json(&payload)
            .send()?;

        let status = response.status();
        if status.is_success() {
            let token = response.text()?;
            let token_trimmed = token.trim().to_string();
            
            let mut sess = self.session.lock().unwrap();
            sess.set_token(&token_trimmed);
            Ok(token_trimmed)
        } else if status == StatusCode::UNAUTHORIZED {
            Err(ApiError::Unauthorized)
        } else {
            let message = response.text().unwrap_or_default();
            Err(ApiError::HttpError { status, message })
        }
    }

    /// Internal helper to check and get a valid token.
    /// If the token doesn't exist or is expired, tries to login first.
    fn get_valid_token(&self) -> Result<String, ApiError> {
        let cached_token = {
            let sess = self.session.lock().unwrap();
            if sess.is_token_valid() {
                sess.token.clone()
            } else {
                None
            }
        };

        if let Some(tok) = cached_token {
            Ok(tok)
        } else {
            self.login()
        }
    }

    /// Execute a blocking request with automatic 401 re-login retry once.
    fn execute_with_retry<F>(&self, request_fn: F) -> Result<reqwest::blocking::Response, ApiError>
    where
        F: Fn(&Client, &str) -> Result<reqwest::blocking::Response, reqwest::Error>
    {
        let token = self.get_valid_token()?;
        let res = request_fn(&self.client, &token)?;
        if res.status() == StatusCode::UNAUTHORIZED {
            // Clear cached token
            {
                let mut sess = self.session.lock().unwrap();
                sess.clear_token();
            }
            // Re-authenticate
            let new_token = self.login()?;
            // Retry once
            let res_retry = request_fn(&self.client, &new_token)?;
            if res_retry.status() == StatusCode::UNAUTHORIZED {
                return Err(ApiError::Unauthorized);
            }
            Ok(res_retry)
        } else {
            Ok(res)
        }
    }

    /// Fetch the document tree.
    pub fn get_documents(&self) -> Result<DocumentTree, ApiError> {
        let response = self.execute_with_retry(|client, token| {
            let host = {
                let sess = self.session.lock().unwrap();
                sess.host.clone()
            };
            let url = format!("{}/ui/api/documents", host);
            client.get(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .send()
        })?;

        let status = response.status();
        if status.is_success() {
            let tree: DocumentTree = response.json()?;
            Ok(tree)
        } else if status == StatusCode::UNAUTHORIZED {
            Err(ApiError::Unauthorized)
        } else {
            let message = response.text().unwrap_or_default();
            Err(ApiError::HttpError { status, message })
        }
    }

    /// Upload a document.
    /// `parent_id` is the folder ID, or "root" for the root folder.
    /// `file_path` is the path to the PDF/EPUB file.
    pub fn upload_document<P: AsRef<Path>>(&self, parent_id: &str, file_path: P) -> Result<(), ApiError> {
        let file_path = file_path.as_ref();
        let file_name = file_path.file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file path"))?
            .to_string_lossy()
            .into_owned();

        let file_bytes = std::fs::read(file_path)?;

        let response = self.execute_with_retry(|client, token| {
            let host = {
                let sess = self.session.lock().unwrap();
                sess.host.clone()
            };
            let url = format!("{}/ui/api/documents/upload", host);

            let file_part = multipart::Part::bytes(file_bytes.clone())
                .file_name(file_name.clone());

            let form = multipart::Form::new()
                .text("parent", parent_id.to_string())
                .part("file", file_part);

            client.post(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .multipart(form)
                .send()
        })?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else if status == StatusCode::UNAUTHORIZED {
            Err(ApiError::Unauthorized)
        } else if status == StatusCode::CONFLICT {
            let body = response.json::<ConflictResponseBody>().ok();
            let doc_id = body.and_then(|b| b.doc_id);
            Err(ApiError::Conflict { doc_id })
        } else {
            let message = response.text().unwrap_or_default();
            Err(ApiError::HttpError { status, message })
        }
    }

    /// Notify the tablet to synchronize.
    pub fn sync(&self) -> Result<(), ApiError> {
        let response = self.execute_with_retry(|client, token| {
            let host = {
                let sess = self.session.lock().unwrap();
                sess.host.clone()
            };
            let url = format!("{}/ui/api/sync", host);
            client.get(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .send()
        })?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else if status == StatusCode::UNAUTHORIZED {
            Err(ApiError::Unauthorized)
        } else {
            let message = response.text().unwrap_or_default();
            Err(ApiError::HttpError { status, message })
        }
    }

    /// Create a new folder on the server.
    pub fn create_folder(&self, name: &str, parent_id: &str) -> Result<(), ApiError> {
        let response = self.execute_with_retry(|client, token| {
            let host = {
                let sess = self.session.lock().unwrap();
                sess.host.clone()
            };
            let url = format!("{}/ui/api/folders", host);
            
            let payload = serde_json::json!({
                "name": name,
                "parent": parent_id
            });

            client.post(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .json(&payload)
                .send()
        })?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else if status == StatusCode::UNAUTHORIZED {
            Err(ApiError::Unauthorized)
        } else {
            let message = response.text().unwrap_or_default();
            Err(ApiError::HttpError { status, message })
        }
    }
}
