use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RmfcSession {
    pub host: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub last_auth: Option<Instant>,
    pub allow_http: bool,
}

impl RmfcSession {
    pub fn new(host: &str, email: &str, password: &str, allow_http: bool) -> Result<Self, String> {
        let host_trimmed = host.trim().trim_end_matches('/');
        
        if !host_trimmed.starts_with("https://") && !host_trimmed.starts_with("http://") {
            return Err("URL must start with http:// or https://".to_string());
        }

        if host_trimmed.starts_with("http://") && !allow_http {
            return Err("HTTP is disabled by default for security. Enable allow_http if you are connecting to a local LAN server.".to_string());
        }

        Ok(Self {
            host: host_trimmed.to_string(),
            email: email.trim().to_string(),
            password: password.to_string(),
            token: None,
            last_auth: None,
            allow_http,
        })
    }

    pub fn is_token_valid(&self) -> bool {
        match (self.token.as_ref(), self.last_auth) {
            (Some(_), Some(last_auth_time)) => {
                // Token valid for 23 hours
                last_auth_time.elapsed() < Duration::from_secs(23 * 3600)
            }
            _ => false,
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
        self.last_auth = Some(Instant::now());
    }

    pub fn clear_token(&mut self) {
        self.token = None;
        self.last_auth = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_new_https() {
        let session = RmfcSession::new("https://example.com/", "test@example.com", "password", false);
        assert!(session.is_ok());
        let session = session.unwrap();
        assert_eq!(session.host, "https://example.com");
        assert_eq!(session.email, "test@example.com");
        assert_eq!(session.password, "password");
        assert!(!session.allow_http);
    }

    #[test]
    fn test_session_new_http_not_allowed() {
        let session = RmfcSession::new("http://example.com", "test@example.com", "password", false);
        assert!(session.is_err());
    }

    #[test]
    fn test_session_new_http_allowed() {
        let session = RmfcSession::new("http://example.com", "test@example.com", "password", true);
        assert!(session.is_ok());
        let session = session.unwrap();
        assert_eq!(session.host, "http://example.com");
        assert!(session.allow_http);
    }

    #[test]
    fn test_session_new_invalid_protocol() {
        let session = RmfcSession::new("ftp://example.com", "test@example.com", "password", true);
        assert!(session.is_err());
    }

    #[test]
    fn test_session_token_lifecycle() {
        let mut session = RmfcSession::new("https://example.com", "test@example.com", "password", false).unwrap();
        assert!(!session.is_token_valid());
        assert_eq!(session.token, None);

        session.set_token("my_token");
        assert!(session.is_token_valid());
        assert_eq!(session.token, Some("my_token".to_string()));

        session.clear_token();
        assert!(!session.is_token_valid());
        assert_eq!(session.token, None);
    }
}
