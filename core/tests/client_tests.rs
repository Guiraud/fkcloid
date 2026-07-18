use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use fkcloud_core::{RmfcSession, RmfcClient};

fn start_mock_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 2048];
            let n = stream.read(&mut buffer).unwrap();
            let req = String::from_utf8_lossy(&buffer[..n]);
            
            if req.contains("POST /ui/api/login") {
                let body = "mock_jwt";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            } else if req.contains("GET /ui/api/documents") {
                if req.contains("mock_jwt") {
                    let body = r#"{"Entries":[{"id":"doc123","name":"Test PDF","isFolder":false,"lastModified":"2026-07-17T17:00:00Z","type":"pdf","size":123}],"Trash":[]}"#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    let response = "HTTP/1.1 401 Unauthorized\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            } else if req.contains("POST /ui/api/documents/upload") {
                if req.contains("mock_jwt") {
                    let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    let response = "HTTP/1.1 401 Unauthorized\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        }
    });
    
    format!("http://127.0.0.1:{}", port)
}

fn start_relogin_mock_server() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let documents_call_count = Arc::new(AtomicUsize::new(0));
    let count_clone = documents_call_count.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 2048];
            let n = stream.read(&mut buffer).unwrap();
            let req = String::from_utf8_lossy(&buffer[..n]);

            if req.contains("POST /ui/api/login") {
                let body = "new_mock_jwt";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            } else if req.contains("GET /ui/api/documents") {
                let call_num = count_clone.fetch_add(1, Ordering::SeqCst);
                if call_num == 0 {
                    let response = "HTTP/1.1 401 Unauthorized\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    if req.contains("new_mock_jwt") {
                        let body = r#"{"Entries":[],"Trash":[]}"#;
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        );
                        stream.write_all(response.as_bytes()).unwrap();
                    } else {
                        let response = "HTTP/1.1 401 Unauthorized\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
            }
        }
    });

    (format!("http://127.0.0.1:{}", port), documents_call_count)
}

#[test]
fn test_client_flow() {
    let mock_host = start_mock_server();
    let session = RmfcSession::new(&mock_host, "test@example.com", "my_password", true).unwrap();
    let client = RmfcClient::new(session);

    // 1. Test Login
    let token = client.login().unwrap();
    assert_eq!(token, "mock_jwt");

    // 2. Test Get Documents
    let tree = client.get_documents().unwrap();
    assert_eq!(tree.entries.len(), 1);
    assert_eq!(tree.entries[0].id, "doc123");
    assert_eq!(tree.entries[0].name, "Test PDF");
    assert!(!tree.entries[0].is_folder);

    // 3. Test Upload Document
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_upload.pdf");
    std::fs::write(&test_file, b"dummy pdf content").unwrap();

    let upload_res = client.upload_document("root", &test_file);
    assert!(upload_res.is_ok());

    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_client_auto_relogin() {
    let (mock_host, call_count) = start_relogin_mock_server();
    let mut session = RmfcSession::new(&mock_host, "test@example.com", "my_password", true).unwrap();
    session.set_token("old_expired_token");
    let client = RmfcClient::new(session);

    let tree = client.get_documents().unwrap();
    assert_eq!(tree.entries.len(), 0);
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}
