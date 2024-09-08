// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use std::error::Error;
use std::sync::{Arc, Mutex};

mod proxy_server;

lazy_static! {
    static ref PROXY: Arc<Mutex<Option<proxy_server::ProxyServer>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
fn cmd_proxy_is_running() -> Result<bool, String> {
    let guard = PROXY.lock().map_err(|err| err.to_string())?;
    if let Some(proxy) = guard.as_ref() {
        Ok(proxy.is_running())
    } else {
        Ok(false)
    }
}

#[tauri::command]
fn cmd_proxy_run(ingress: String, backend: String) -> Result<(), String> {
    let mut guard = PROXY.lock().map_err(|err| err.to_string())?;
    if let Some(proxy) = guard.as_mut() {
        proxy.run(ingress.clone(), backend)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = tauri::Builder::default().setup(|app| {
        let app_handle = app.handle().clone();
        let mut proxy_guard = PROXY.lock()?;
        *proxy_guard = Some(proxy_server::ProxyServer::new(app_handle));
        Ok(())
    });
    app.invoke_handler(tauri::generate_handler![cmd_proxy_is_running])
        .invoke_handler(tauri::generate_handler![cmd_proxy_run])
        .run(tauri::generate_context!())
        .expect("Failed to run tauri application");
    Ok(())
}
