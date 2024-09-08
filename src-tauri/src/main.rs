// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use tokio::task;

mod proxy_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = tauri::Builder::default().setup(|app| {
        let app_handle = app.handle().clone();
        task::spawn(async move {
            let _ = proxy_server::ProxyServer::new("127.0.0.1:9090", app_handle)
                .run()
                .await;
        });
        Ok(())
    });
    app.run(tauri::generate_context!())
        .expect("Failed to run tauri application");
    Ok(())
}
