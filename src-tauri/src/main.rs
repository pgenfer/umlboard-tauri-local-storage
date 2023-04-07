#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use action_handler::ActionHandler;
use bonsai_repository::BonsaiRepository;
use bonsaidb::local::config::{StorageConfiguration, Builder};
use classifier::Classifier;
// use data_model::{save_classifier_polo, get_classifiers};
use repository::Repository;
use surrealdb::kvs::Datastore;

mod classifier_service;
mod action_handler;
mod classifier;
mod value_objects;
mod entity;
mod bonsai_repository;
mod repository;

use std::{string::String, collections::HashMap, path::{Path, PathBuf}};
// use bonsaidb::{local::{Database, config::{StorageConfiguration, Builder}, Storage}, core::connection::StorageConnection};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use classifier_service::ClassifierService;

use crate::{value_objects::Point};

//#[tauri::command]
//fn ipc_message(message: IpcMessage) -> IpcMessage {

    // Normally, we would have some kind of dictionary 
    // with our services created during startup.
    // In this example, we just create everything in place here for simplifaction
    // let service = ClassifierService{};
    // let mut handlers = HashMap::new();
    // handlers.insert(service.domain(), &service);
    
    // // this is were our actual command begins
    // let message_handler = handlers.get(&*message.domain).unwrap(); 
    // let response = message_handler.receive_action(message.action).unwrap();
    // IpcMessage {
    //     domain: message_handler.domain().to_string(),
    //     action: response
    // }
//}
#[tokio::main]
async fn main() {

    // DB test:
    // 1. read all elements from the DB
    // 2. if there is no element, create a new one

    let bonsai_db = bonsaidb::local::AsyncDatabase::open::<Classifier>(
        StorageConfiguration::new("testdata/bonsai/umlboard.bonsaidb")).await.unwrap();
    // let bonsai_db = bonsaidb::local::Database::open::<Classifier>(
    //     StorageConfiguration::new("testdata/bonsai/umlboard.bonsaidb")).unwrap();

    // let surrealDb = Datastore::new("testdata/surreal/umlboard.db");

    let classifier_repository = BonsaiRepository::<Classifier>::new(&bonsai_db);
    let classifier_service = ClassifierService::new(&classifier_repository);
    let mut classifiers = classifier_service.load_classifiers().await;
    if classifiers.len() < 2 {
        classifier_service.create_new_classifier("new class").await;
        classifiers = classifier_service.load_classifiers().await;
    }
    let id = &classifiers[0]._id;
    classifier_service.update_classifier_name(&id, "changed name44").await;
    classifiers = classifier_service.load_classifiers().await;

    print!("{:?}", classifiers);
    


    // poloDB test
    // let db = Database::open_file("umlboard.polo").unwrap();
    // save_classifier_polo(&db);
    //get_classifiers(&db);

    // summary about bonsaidb: 
    // Implementation may work, however, having a complete folder and not a single file is a bit inconvenient
    // also, having a binary format is a bit complicated, and changing this to json may be a bit cumbersome.
    // maybe we have to compress the folder to a new file by using https://crates.io/crates/zip
    // let storage =
    //      Storage::open(StorageConfiguration::new("my-db.bonsaidb").with_schema::<Classifier>().unwrap()).unwrap();
    // storage.create_database::<Classifier>("default", true).unwrap();
    // let db = storage.database::<Classifier>("default").unwrap();

    
        
    
    // let classifier = data_model::save_classifier(&db).unwrap();

    // let mut path = PathBuf::new();
    // path.push("test.txt");
    // storage.backup(&path).unwrap();
    
    // print!("{}", classifier.header.id);
    // print!("{}", classifier.contents.name);
    // let changed_classifier = data_model::change_name(1, "new name".to_string(), &db).unwrap();
    //print!("{}", changed_classifier.contents.name);



    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 