#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod account_manager;
mod database;
mod proxy_manager;

use account_manager::AccountManager;
use database::{Account, Database};
use proxy_manager::ProxyManager;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Mutex<Database>,
    account_manager: Mutex<AccountManager>,
    proxy_manager: Mutex<ProxyManager>,
}

// Tauri Commands

#[tauri::command]
fn get_accounts(state: State<AppState>) -> Result<Vec<Account>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_accounts()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_account(account_json: String, state: State<AppState>) -> Result<String, String> {
    state
        .db
        .lock()
        .unwrap()
        .add_account(&account_json)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_account(email: String, state: State<AppState>) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .delete_account(&email)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_active_account(email: String, state: State<AppState>) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .set_active_account(&email)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_active_account(state: State<AppState>) -> Result<Option<String>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_active_account()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_active_account(state: State<AppState>) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .clear_active_account()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn refresh_token(email: String, state: State<AppState>) -> Result<String, String> {
    state
        .account_manager
        .lock()
        .unwrap()
        .refresh_token(&email)
        .map(|_| format!("Token refreshed for {}", email))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_limit_info(email: String, state: State<AppState>) -> Result<String, String> {
    state
        .account_manager
        .lock()
        .unwrap()
        .get_limit_info(&email)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_account_health(
    email: String,
    health_status: String,
    state: State<AppState>,
) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .update_account_health(&email, &health_status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_account_limit_info(
    email: String,
    limit_info: String,
    state: State<AppState>,
) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .update_account_limit_info(&email, &limit_info)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn start_proxy(state: State<AppState>) -> Result<(), String> {
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or("Failed to get app directory")?;
    let script_path = app_dir.join("warp_proxy_script.py");

    let mut proxy_manager = state.proxy_manager.lock().unwrap();
    proxy_manager
        .start_mitmproxy(script_path)
        .map_err(|e| e.to_string())?;
    
    proxy_manager
        .enable_system_proxy()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn stop_proxy(state: State<AppState>) -> Result<(), String> {
    let mut proxy_manager = state.proxy_manager.lock().unwrap();
    proxy_manager
        .disable_system_proxy()
        .map_err(|e| e.to_string())?;
    
    proxy_manager
        .stop_mitmproxy()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn is_proxy_running(state: State<AppState>) -> Result<bool, String> {
    Ok(state.proxy_manager.lock().unwrap().is_running())
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let app_dir = tauri::api::path::app_data_dir(&app.config())
                .expect("Failed to get app directory");
            
            std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");
            
            let db_path = app_dir.join("accounts.db");
            let db = Database::new(db_path).expect("Failed to initialize database");
            let account_manager = AccountManager::new(db.clone());
            let proxy_manager = ProxyManager::new();

            // Copy proxy script to app directory if not exists
            let proxy_script = app_dir.join("warp_proxy_script.py");
            if !proxy_script.exists() {
                if let Ok(current_dir) = std::env::current_dir() {
                    let source = current_dir.join("warp_proxy_script.py");
                    if source.exists() {
                        let _ = std::fs::copy(&source, &proxy_script);
                    }
                }
            }

            app.manage(AppState {
                db: Mutex::new(db),
                account_manager: Mutex::new(account_manager),
                proxy_manager: Mutex::new(proxy_manager),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            delete_account,
            set_active_account,
            get_active_account,
            clear_active_account,
            refresh_token,
            get_limit_info,
            update_account_health,
            update_account_limit_info,
            start_proxy,
            stop_proxy,
            is_proxy_running,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
