use std::error::Error;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task;
use tokio::task::JoinHandle;

pub struct Store {
    app_handle: AppHandle,
    sender: Option<Sender<super::RequestDetails>>,
    receiver_task: Option<JoinHandle<()>>,
}

impl Store {
    pub fn new(app_handle: AppHandle) -> Store {
        Store {
            app_handle,
            sender: None,
            receiver_task: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (sender, mut receiver) = mpsc::channel::<super::RequestDetails>(32);
        let app_handle = self.app_handle.clone();
        let receiver_task = task::spawn(async move {
            while let Some(message) = receiver.recv().await {
                if let Err(_) = app_handle.emit_all("update", message) {
                    println!("Failed to emit update event");
                }
            }
        });
        self.sender = Some(sender);
        self.receiver_task = Some(receiver_task);
        Ok(())
    }

    pub async fn send(&self, request_details: super::RequestDetails) {
        if let Some(sender) = self.sender.as_ref() {
            let sender = sender.clone();
            if let Err(_) = sender.send(request_details).await {
                println!("Failed to send message");
            }
        }
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        if let Some(handle) = self.receiver_task.take() {
            handle.abort();
        }
    }
}
