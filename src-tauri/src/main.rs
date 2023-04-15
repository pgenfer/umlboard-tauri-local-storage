#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use action_handler::{ActionReceiver, ActionHandler, convert_and_handle_action};
//use bonsai_repository::BonsaiRepository;
use surreal_repository::SurrealRepository;
//use bonsaidb::{local::config::{StorageConfiguration, Builder}, core::permissions::Action};
use classifier::Classifier;
use surrealdb::{Surreal, engine::local::File};
use tauri::{State};

mod classifier_service;
mod action_handler;
mod classifier;
mod value_objects;
mod entity;
mod bonsai_repository;
mod surreal_repository;
mod repository;

use std::{string::String, collections::HashMap};
// use bonsaidb::{local::{Database, config::{StorageConfiguration, Builder}, Storage}, core::connection::StorageConnection};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use classifier_service::{ClassifierService, ClassifierAction, ApplicationAction};

#[tauri::command]
fn ipc_message(message: IpcMessage, state: State<ApplicationState>, ) -> IpcMessage {
    let message_handler = state.action_handlers.get(&*message.domain).unwrap(); 
    let response = message_handler.receive_action(message.action).unwrap();
    IpcMessage {
        domain: message_handler.domain().to_string(),
        action: response
    }
}
#[tokio::main]
async fn main() {
    // create our application state
    let state = ApplicationState::new().await;
    // setup and start Tauru
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 

struct ApplicationState {
    classifier_service: Box<ClassifierService>,
    action_handlers: HashMap<String, &Box<dyn ActionReceiver + Send + Sync>>
}

impl ApplicationState {
    async fn new() -> ApplicationState { 
        // setup surreal db, repository, service
        let surreal_db = Surreal::new::<File>("testdata/surreal/umlboard.db").await.unwrap();
        surreal_db.use_ns("umlboard_namespace").use_db("umlboard_database").await.unwrap();
        let classifier_repository = Box::new(SurrealRepository::<Classifier>::new(Box::new(surreal_db), "classifiers"));
        // let classifier_service = Box::new(ClassifierService::new(classifier_repository));
        // let mut action_handlers = HashMap::<String, &Box<dyn ActionReceiver + Send + Sync>>::new();
        // store everything in state
        // action_handlers.insert(classifier_service.domain().to_string(), &*classifier_service);
        let mut state = ApplicationState {
            classifier_service: Box::new(ClassifierService::new(classifier_repository)),
            action_handlers: HashMap::new()
        };
        let service = state.classifier_service;
        state.action_handlers.insert(service.domain().to_string(), &(state.classifier_service));


        let mut handlers = HashMap::new();

        // handlers.insert("classifiers", Box::new(|a| convert_and_handle_action::<ClassifierAction>(a, &*classifier_service)));
        handlers.insert("classifiers", |a, s| convert_and_handle_action::<ApplicationAction>(a, s));
        // handlers.insert("classifiers", |a: ApplicationAction| classifier_service.handle_action(a));
        let action = handlers.get("classifieres").unwrap();
        
        // action(ClassifierAction::CancelClassifierRename, classifier_service);
        


        state
    }
}

