#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use action_handler::{ActionDispatcher};
use surreal_repository::SurrealRepository;
use surrealdb::{Surreal, engine::local::File};
use tauri::{State, Manager};

mod classifier_service;
mod classifier;
mod value_objects;
mod bonsai_repository;
mod surreal_repository;
mod repository;
mod action_handler;
mod actions;


use std::{string::String, collections::HashMap, sync::Arc};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use classifier_service::{ClassifierService};

// async stateful commands must return Result
// https://github.com/tauri-apps/tauri/discussions/4317
#[tauri::command]
async fn ipc_message(message: IpcMessage, 
    context: State<'_, ApplicationContext>) -> Result<IpcMessage, ()> {
    let dispatcher = context.action_dispatchers.get(&message.domain).unwrap();
    let response = dispatcher.dispatch_action(message.domain.to_string(),message.action).await;
    Ok(IpcMessage {
        domain: message.domain,
        action: response
    })
}
#[tokio::main]
async fn main() {
    // create our application state
    let context = ApplicationContext::new().await;
    // setup and start Tauru
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(context)
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 

struct ApplicationContext {
    action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>>
}

impl ApplicationContext {
    async fn new() -> Self { 
        let surreal_db = Surreal::new::<File>("testdata/surreal/umlboard.db").await.unwrap();
        surreal_db.use_ns("umlboard_namespace").use_db("umlboard_database").await.unwrap();
        let repository = Box::new(SurrealRepository::new(Box::new(surreal_db), "classifiers"));
        let service = Arc::new(ClassifierService::new(repository));
        let mut action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>> = HashMap::new();
        action_dispatchers.insert(actions::classifier_action::CLASSIFIER_DOMAIN.to_string(), service.clone());
        action_dispatchers.insert(actions::application_action::APPLICATION_DOMAIN.to_string(), service.clone());
        Self { action_dispatchers }
    }
}






