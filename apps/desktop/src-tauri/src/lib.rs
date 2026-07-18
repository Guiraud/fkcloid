use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use keyring::Entry;
use tauri::{Manager, Emitter};
use tauri::menu::{Menu, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use std::sync::{Mutex, OnceLock};

use fkcloud_core::{RmfcClient, RmfcSession, DocumentTree};

static PENDING_UPLOAD: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_pending_upload_mutex() -> &'static Mutex<Option<String>> {
    PENDING_UPLOAD.get_or_init(|| Mutex::new(None))
}

static ICON_DEFAULT: &[u8] = include_bytes!("../icons/tray-default.png");
static ICON_ACTIVE: &[u8] = include_bytes!("../icons/tray-active.png");

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub username: String,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub contextmenu: bool,
}

fn get_config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;
    Some(PathBuf::from(home).join(".fkcloud-desktop-config.json"))
}

fn get_password(username: &str) -> Option<String> {
    let entry = Entry::new("fkcloud-share", &format!("password:{}", username)).ok()?;
    entry.get_password().ok()
}

fn set_password(username: &str, password: &str) -> Result<(), String> {
    let entry = Entry::new("fkcloud-share", &format!("password:{}", username))
        .map_err(|e| e.to_string())?;
    entry.set_password(password).map_err(|e| e.to_string())
}

fn delete_password(username: &str) -> Result<(), String> {
    let entry = Entry::new("fkcloud-share", &format!("password:{}", username))
        .map_err(|e| e.to_string())?;
    let _ = entry.delete_password();
    Ok(())
}

fn load_client_from_storage() -> Result<RmfcClient, String> {
    let config_path = get_config_path()
        .ok_or_else(|| "Could not determine config path".to_string())?;
    
    if !config_path.exists() {
        return Err("No active session. Please login first.".to_string());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let password = get_password(&config.username)
        .ok_or_else(|| "Credentials not found in keyring. Please login again.".to_string())?;

    let session = RmfcSession::new(&config.host, &config.username, &password, true)
        .map_err(|e| format!("Failed to create session: {}", e))?;

    Ok(RmfcClient::new(session))
}

#[tauri::command]
fn get_config() -> Result<Option<AppConfig>, String> {
    let config_path = get_config_path()
        .ok_or_else(|| "Could not determine config path".to_string())?;
    
    if !config_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    Ok(Some(config))
}

#[tauri::command]
fn check_auth() -> Result<bool, String> {
    match load_client_from_storage() {
        Ok(client) => {
            match client.get_documents() {
                Ok(_) => Ok(true),
                Err(e) => {
                    eprintln!("Auth check failed: {}", e);
                    Ok(false)
                }
            }
        }
        Err(_) => Ok(false),
    }
}

#[tauri::command]
fn login(host: String, username: String, password: String) -> Result<String, String> {
    let session = RmfcSession::new(&host, &username, &password, true)
        .map_err(|e| format!("Invalid session configuration: {}", e))?;
    let client = RmfcClient::new(session);

    let token = client.login().map_err(|e| e.to_string())?;

    let config_path = get_config_path()
        .ok_or_else(|| "Could not determine config path".to_string())?;
    
    let config = AppConfig {
        host: host.clone(),
        username: username.clone(),
        autostart: false,
        contextmenu: false,
    };
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| e.to_string())?;
    
    std::fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    set_password(&username, &password)?;

    Ok(token)
}

#[tauri::command]
fn logout() -> Result<(), String> {
    let config_path = get_config_path()
        .ok_or_else(|| "Could not determine config path".to_string())?;
    
    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).ok();
        if let Some(c) = content {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&c) {
                let _ = delete_password(&config.username);
            }
        }
        let _ = std::fs::remove_file(config_path);
    }
    Ok(())
}

#[tauri::command]
fn get_documents() -> Result<DocumentTree, String> {
    let client = load_client_from_storage()?;
    client.get_documents().map_err(|e| e.to_string())
}

#[tauri::command]
fn upload_document(parent_id: String, file_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&file_path);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_markdown = extension == "md" || extension == "markdown";

    let actual_upload_path = if is_markdown {
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document");
        
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
            
        let temp_dir = std::env::temp_dir().join(format!("fkcloud-{}", ts));
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Impossible de créer le dossier temporaire : {}", e))?;
        let temp_epub_path = temp_dir.join(format!("{}.epub", stem));

        let output = std::process::Command::new("pandoc")
            .arg(&file_path)
            .arg("-o")
            .arg(&temp_epub_path)
            .output();

        match output {
            Ok(out) => {
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let _ = std::fs::remove_dir_all(&temp_dir);
                    return Err(format!("Erreur de conversion pandoc : {}", stderr));
                }
            }
            Err(e) => {
                let _ = std::fs::remove_dir_all(&temp_dir);
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Err("La commande 'pandoc' est requise pour convertir les fichiers Markdown en EPUB. Veuillez l'installer (ex: 'brew install pandoc').".to_string());
                } else {
                    return Err(format!("Erreur lors du lancement de pandoc : {}", e));
                }
            }
        }
        temp_epub_path
    } else {
        path.to_path_buf()
    };

    let client = load_client_from_storage()?;
    let upload_result = client.upload_document(&parent_id, &actual_upload_path).map_err(|e| e.to_string());

    if is_markdown {
        if let Some(parent) = actual_upload_path.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }

    upload_result
}

#[tauri::command]
fn sync_tablet() -> Result<(), String> {
    let client = load_client_from_storage()?;
    client.sync().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_folder(parent_id: String, name: String) -> Result<(), String> {
    let client = load_client_from_storage()?;
    client.create_folder(&name, &parent_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn trigger_file_dialog(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("Documents reMarkable", &["pdf", "epub", "md", "markdown"])
            .pick_file() 
        {
            let path_str = file_path.to_string_lossy().to_string();
            let _ = app.emit("upload-file-selected", path_str);
        }
    });
}

use std::net::ToSocketAddrs;

fn is_tablet_active() -> bool {
    if let Ok(addr) = "10.11.99.1:22".parse::<std::net::SocketAddr>() {
        if std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(300)).is_ok() {
            return true;
        }
    }
    if let Ok(addrs) = "remarkable.local:22".to_socket_addrs() {
        for addr in addrs {
            if std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(300)).is_ok() {
                return true;
            }
        }
    }
    false
}

fn set_autostart(enable: bool) -> Result<(), String> {
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|_| "HOME not defined".to_string())?;
        let plist_dir = std::path::PathBuf::from(home).join("Library/LaunchAgents");
        let plist_path = plist_dir.join("net.example.fkcloud.plist");
        
        if enable {
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>net.example.fkcloud</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
                current_exe.to_string_lossy()
            );
            std::fs::create_dir_all(&plist_dir).map_err(|e| e.to_string())?;
            std::fs::write(&plist_path, plist_content).map_err(|e| e.to_string())?;
        } else {
            if plist_path.exists() {
                std::fs::remove_file(&plist_path).map_err(|e| e.to_string())?;
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        if enable {
            let exe_str = format!("\"{}\"", current_exe.to_string_lossy());
            let _ = std::process::Command::new("reg")
                .args(&["add", r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run", "/v", "FkCloud Share", "/t", "REG_SZ", "/d", &exe_str, "/f"])
                .status();
        } else {
            let _ = std::process::Command::new("reg")
                .args(&["delete", r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run", "/v", "FkCloud Share", "/f"])
                .status();
        }
    }
    Ok(())
}

fn set_context_menu(enable: bool) -> Result<(), String> {
    let _current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|_| "HOME not defined".to_string())?;
        let services_dir = std::path::PathBuf::from(home).join("Library/Services");
        let workflow_dir = services_dir.join("Envoyer vers Paper pure.workflow");
        let contents_dir = workflow_dir.join("Contents");
        let resources_dir = contents_dir.join("Resources");
        
        if enable {
            std::fs::create_dir_all(&resources_dir).map_err(|e| e.to_string())?;
            
            let plist_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key>
	<string>Envoyer vers Paper pure</string>
	<key>CFBundleDevelopmentRegion</key>
	<string>fr_FR</string>
	<key>CFBundleIdentifier</key>
	<string>net.example.fkcloud.remarkable-service</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>NSServices</key>
	<array>
		<dict>
			<key>NSBackgroundColorName</key>
			<string>background</string>
			<key>NSIconName</key>
			<string>NSActionTemplate</string>
			<key>NSMenuItem</key>
			<dict>
				<key>default</key>
				<string>Envoyer vers Paper pure</string>
			</dict>
			<key>NSMessage</key>
			<string>runWorkflowAsService</string>
			<key>NSRequiredContext</key>
			<dict>
				<key>NSApplicationIdentifier</key>
				<string>com.apple.finder</string>
			</dict>
			<key>NSSendFileTypes</key>
			<array>
				<string>public.item</string>
			</array>
		</dict>
	</array>
</dict>
</plist>"#;
            std::fs::write(contents_dir.join("Info.plist"), plist_content).map_err(|e| e.to_string())?;

            let wflow_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>AMApplicationBuild</key>
	<string>523</string>
	<key>AMApplicationVersion</key>
	<string>2.10</string>
	<key>AMDocumentVersion</key>
	<string>2</string>
	<key>actions</key>
	<array>
		<dict>
			<key>action</key>
			<dict>
				<key>AMAccepts</key>
				<dict>
					<key>Container</key>
					<string>List</string>
					<key>Optional</key>
					<true/>
					<key>Types</key>
					<array>
						<string>com.apple.cocoa.path</string>
					</array>
				</dict>
				<key>AMActionVersion</key>
				<string>2.0.3</string>
				<key>AMApplication</key>
				<array>
					<string>Automator</string>
				</array>
				<key>AMParameterProperties</key>
				<dict>
					<key>COMMAND_STRING</key>
					<dict/>
					<key>CheckedForUserDefaultShell</key>
					<dict/>
					<key>inputMethod</key>
					<dict/>
					<key>shell</key>
					<dict/>
					<key>source</key>
					<dict/>
				</dict>
				<key>AMProvides</key>
				<dict>
					<key>Container</key>
					<string>List</string>
					<key>Types</key>
					<array>
						<string>com.apple.cocoa.string</string>
					</array>
				</dict>
				<key>ActionBundlePath</key>
				<string>/System/Library/Automator/Run Shell Script.action</string>
				<key>ActionName</key>
				<string>Run Shell Script</string>
				<key>ActionParameters</key>
				<dict>
					<key>COMMAND_STRING</key>
					<string>for f in "$@"; do
  "{}" --upload "$f" &amp;
done</string>
					<key>CheckedForUserDefaultShell</key>
					<true/>
					<key>inputMethod</key>
					<integer>1</integer>
					<key>shell</key>
					<string>/bin/zsh</string>
					<key>source</key>
					<string></string>
				</dict>
				<key>BundleIdentifier</key>
				<string>com.apple.RunShellScript</string>
				<key>CFBundleVersion</key>
				<string>2.0.3</string>
				<key>CanShowSelectedItemsWhenRun</key>
				<false/>
				<key>CanShowWhenRun</key>
				<true/>
				<key>Category</key>
				<array>
					<string>AMCategoryUtilities</string>
				</array>
				<key>Class Name</key>
				<string>RunShellScriptAction</string>
				<key>InputUUID</key>
				<string>5595FB5D-2745-4646-B4CB-B1651E2B82D1</string>
				<key>Keywords</key>
				<array>
					<string>Shell</string>
					<string>Script</string>
					<string>Command</string>
					<string>Run</string>
					<string>Unix</string>
				</array>
				<key>OutputUUID</key>
				<string>88B12B58-08BA-4AE3-A72D-6A616C00381B</string>
				<key>UUID</key>
				<string>EB596654-6547-4218-AC9C-694771DD4977</string>
				<key>UnlocalizedApplications</key>
				<array>
					<string>Automator</string>
				</array>
				<key>arguments</key>
				<dict>
					<key>0</key>
					<dict>
						<key>default value</key>
						<integer>0</integer>
						<key>name</key>
						<string>inputMethod</string>
						<key>required</key>
						<string>0</string>
						<key>type</key>
						<string>0</string>
						<key>uuid</key>
						<string>0</string>
					</dict>
					<key>1</key>
					<dict>
						<key>default value</key>
						<string></string>
						<key>name</key>
						<string>source</string>
						<key>required</key>
						<string>0</string>
						<key>type</key>
						<string>0</string>
						<key>uuid</key>
						<string>1</string>
					</dict>
					<key>2</key>
					<dict>
						<key>default value</key>
						<false/>
						<key>name</key>
						<string>CheckedForUserDefaultShell</string>
						<key>required</key>
						<string>0</string>
						<key>type</key>
						<string>0</string>
						<key>uuid</key>
						<string>2</string>
					</dict>
					<key>3</key>
					<dict>
						<key>default value</key>
						<string></string>
						<key>name</key>
						<string>COMMAND_STRING</string>
						<key>required</key>
						<string>0</string>
						<key>type</key>
						<string>0</string>
						<key>uuid</key>
						<string>3</string>
					</dict>
					<key>4</key>
					<dict>
						<key>default value</key>
						<string>/bin/sh</string>
						<key>name</key>
						<string>shell</string>
						<key>required</key>
						<string>0</string>
						<key>type</key>
						<string>0</string>
						<key>uuid</key>
						<string>4</string>
					</dict>
				</dict>
				<key>isViewVisible</key>
				<true/>
				<key>location</key>
				<string>309.000000:253.000000</string>
				<key>nibPath</key>
				<string>/System/Library/Automator/Run Shell Script.action/Contents/Resources/en.lproj/main.nib</string>
			</dict>
			<key>isViewVisible</key>
			<true/>
		</dict>
	</array>
	<key>connectors</key>
	<dict/>
	<key>workflowMetaData</key>
	<dict>
		<key>applicationBundleIDsByPath</key>
		<dict/>
		<key>applicationPaths</key>
		<array/>
		<key>inputTypeIdentifier</key>
		<string>com.apple.Automator.fileSystemObject</string>
		<key>outputTypeIdentifier</key>
		<string>com.apple.Automator.nothing</string>
		<key>presentationMode</key>
		<integer>15</integer>
		<key>processInput</key>
		<integer>0</integer>
		<key>serviceApplicationBundleID</key>
		<string>com.apple.finder</string>
		<key>serviceInputTypeIdentifier</key>
		<string>com.apple.Automator.fileSystemObject</string>
		<key>serviceOutputTypeIdentifier</key>
		<string>com.apple.Automator.nothing</string>
		<key>serviceProcessesInput</key>
		<integer>0</integer>
		<key>useAutomaticInputType</key>
		<integer>0</integer>
		<key>workflowTypeIdentifier</key>
		<string>com.apple.Automator.servicesMenu</string>
	</dict>
</dict>
</plist>"#,
                _current_exe.to_string_lossy()
            );
            std::fs::write(resources_dir.join("document.wflow"), wflow_content).map_err(|e| e.to_string())?;
        } else {
            if workflow_dir.exists() {
                let _ = std::fs::remove_dir_all(&workflow_dir);
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        if enable {
            let cmd_str = format!("\"{}\" --upload \"%1\"", _current_exe.to_string_lossy());
            let _ = std::process::Command::new("reg")
                .args(&["add", r"HKCU\Software\Classes\*\shell\Envoyer vers Paper pure\command", "/ve", "/d", &cmd_str, "/f"])
                .status();
        } else {
            let _ = std::process::Command::new("reg")
                .args(&["delete", r"HKCU\Software\Classes\*\shell\Envoyer vers Paper pure", "/f"])
                .status();
        }
    }
    Ok(())
}

#[tauri::command]
fn get_pending_upload() -> Option<String> {
    let mut pending = get_pending_upload_mutex().lock().unwrap();
    pending.take()
}

#[tauri::command]
fn save_settings(autostart: bool, contextmenu: bool) -> Result<(), String> {
    let config_path = get_config_path()
        .ok_or_else(|| "Could not determine config path".to_string())?;
    
    let mut config = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_str::<AppConfig>(&content).map_err(|e| e.to_string())?
    } else {
        return Err("Configuration not initialized".to_string());
    };
    
    config.autostart = autostart;
    config.contextmenu = contextmenu;
    
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, json).map_err(|e| e.to_string())?;
    
    let _ = set_autostart(autostart);
    let _ = set_context_menu(contextmenu);
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Check command line arguments for --upload
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 2 && args[1] == "--upload" {
                let mut pending = get_pending_upload_mutex().lock().unwrap();
                *pending = Some(args[2].clone());
            }

            let show_i = MenuItemBuilder::new("Afficher la fenêtre")
                .id("show")
                .build(app)?;
            let upload_i = MenuItemBuilder::new("Envoyer un fichier...")
                .id("upload_file")
                .build(app)?;
            let quit_i = MenuItemBuilder::new("Quitter")
                .id("quit")
                .build(app)?;
            let menu = Menu::with_items(app, &[&show_i, &upload_i, &quit_i])?;

            let img_default = tauri::image::Image::from_bytes(ICON_DEFAULT).unwrap();

            let _tray = TrayIconBuilder::with_id("main")
                .menu(&menu)
                .tooltip("FkCloud Share")
                .icon(img_default)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Background thread to poll tablet and update tray icon glow
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let img_default = tauri::image::Image::from_bytes(ICON_DEFAULT).unwrap();
                let img_active = tauri::image::Image::from_bytes(ICON_ACTIVE).unwrap();
                let mut last_state = false;
                
                // Init status
                let active = is_tablet_active();
                if active {
                    if let Some(tray) = app_handle.tray_by_id("main") {
                        let _ = tray.set_icon(Some(img_active.clone()));
                        last_state = true;
                    }
                }

                loop {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    let active = is_tablet_active();
                    if active != last_state {
                        if let Some(tray) = app_handle.tray_by_id("main") {
                            let icon = if active { img_active.clone() } else { img_default.clone() };
                            let _ = tray.set_icon(Some(icon));
                            last_state = active;
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .on_menu_event(|app, event| {
            if event.id() == "show" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            } else if event.id() == "upload_file" {
                let app_handle = app.clone();
                std::thread::spawn(move || {
                    if let Some(file_path) = rfd::FileDialog::new()
                        .add_filter("Documents reMarkable", &["pdf", "epub", "md", "markdown"])
                        .pick_file() 
                    {
                        let path_str = file_path.to_string_lossy().to_string();
                        let _ = app_handle.emit("upload-file-selected", path_str);
                    }
                });
            } else if event.id() == "quit" {
                app.exit(0);
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            check_auth,
            login,
            logout,
            get_documents,
            upload_document,
            sync_tablet,
            create_folder,
            trigger_file_dialog,
            save_settings,
            get_pending_upload
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
