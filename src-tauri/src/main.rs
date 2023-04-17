#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use classifier_action_handler::{handle_classifier_action, CLASSIFIER_DOMAIN};
use surreal_repository::SurrealRepository;
use surrealdb::{Surreal, engine::local::File};
use tauri::{State};

mod classifier_service;
mod classifier;
mod value_objects;
mod entity;
mod bonsai_repository;
mod surreal_repository;
mod repository;
mod classifier_action_handler;

use std::{string::String, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use classifier_service::{ClassifierService};

#[tauri::command]
fn ipc_message(message: IpcMessage, 
    handler_state: State<ActionHandlerState>, 
    service_state: State<ServiceState>) -> IpcMessage {
    let classifier_service = &service_state.classifier;
    let handler = handler_state.action_handlers.get(&message.domain).unwrap();
    let response = handler(message.action, classifier_service);
    IpcMessage {
        domain: message.domain,
        action: response
    }
}
#[tokio::main]
async fn main() {
    // create our application state
    let handler_state = ActionHandlerState::new().await;
    let service_state = ServiceState::new().await;
    // setup and start Tauru
    tauri::Builder::default()
        .manage(handler_state)
        .manage(service_state)
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 

struct ServiceState {
    classifier: ClassifierService
}

impl ServiceState {
    async fn new() -> Self { 
        let surreal_db = Surreal::new::<File>("testdata/surreal/umlboard.db").await.unwrap();
        surreal_db.use_ns("umlboard_namespace").use_db("umlboard_database").await.unwrap();
        let repository = Box::new(SurrealRepository::new(Box::new(surreal_db), "classifiers"));
        let service = ClassifierService::new(repository);
        Self { classifier: service } 
    }
}

struct ActionHandlerState {
    action_handlers: HashMap<String, fn(Value, &ClassifierService) -> Value>
}

impl ActionHandlerState {
    async fn new() -> Self { 
        let mut action_handlers: HashMap<String, fn(Value, &ClassifierService) -> Value> = HashMap::new();
        action_handlers.insert(CLASSIFIER_DOMAIN.to_owned(), handle_classifier_action);
        Self {action_handlers}
    }
}






