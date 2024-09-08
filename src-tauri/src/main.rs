// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use lazy_static::lazy_static;
use tokio::task;
use std::sync::{Arc, Mutex};

mod proxy_server;

lazy_static! {
    static ref PROXY: Arc<Mutex<Option<proxy_server::ProxyServer>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
fn cmd_proxy_run(ingress: String, backend: String) {
    println!("I was invoked from JS!, {}, {}", ingress, backend);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = tauri::Builder::default().setup(|app| {
        let app_handle = app.handle().clone();
        let mut proxy_guard = PROXY.lock()?;
        *proxy_guard = Some(proxy_server::ProxyServer::new(app_handle));
        Ok(())
    });
    app.invoke_handler(tauri::generate_handler![cmd_proxy_run])
        .run(tauri::generate_context!())
        .expect("Failed to run tauri application");
    Ok(())
}
