#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::action_handler::ActionHandler;
mod classifier_service;
mod action_handler;

use std::{string::String, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use classifier_service::ClassifierService;

#[tauri::command]
fn ipc_message(message: IpcMessage) -> IpcMessage {
    // Normally, we would have some kind of dictionary 
    // with our services created during startup.
    // In this example, we just create everything in place here for simplifaction
    let service = ClassifierService{};
    let mut handlers = HashMap::new();
    handlers.insert(service.domain(), &service);
    
    // this is were our actual command begins
    let message_handler = handlers.get(&*message.domain).unwrap(); 
    let response = message_handler.receive_action(message.action).unwrap();
    IpcMessage {
        domain: message_handler.domain().to_string(),
        action: response
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 