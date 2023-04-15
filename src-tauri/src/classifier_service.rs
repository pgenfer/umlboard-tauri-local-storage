use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use strum_macros::Display;
use ts_rs::TS;

use crate::{action_handler::{ActionHandler, ActionReceiver, convert_and_handle_action}, repository::Repository, classifier::{Classifier}};

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct EditNameDto {
    new_name: String
}

#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum ClassifierAction {
    RenameClassifier(EditNameDto),
    CancelClassifierRename,
    ClassifierRenamed(EditNameDto),
    ClassifierRenameCanceled(EditNameDto),
    ClassifierRenameError
}

#[derive(Serialize, Deserialize, Display)]
pub enum ApplicationAction {
    ClassiffiersLoaded
}

// we need one file with constants, like
// this will also be generated for redux to use a slice name
pub const CLASSIFIER_DOMAIN: &str = "classifier";

pub struct ClassifierService {
    repository : Box<dyn Repository<Classifier> + Send + Sync>
}
impl<'a> ClassifierService {
    
    pub fn new(classifier_repository: Box<dyn Repository<Classifier> + Send + Sync>) -> Self { 
        Self { repository: classifier_repository } 
    }

    pub async fn load_classifiers(&self) -> Vec<Classifier> {
        let classifiers = self.repository.query_all().await;
        classifiers
    }

    pub async fn create_new_classifier(&self, new_name: &str) -> Classifier { // TODO: use repository error
        let id = uuid::Uuid::new_v4().to_string();
        let new_classifier = self.repository.insert(Classifier{
            _id: id.clone(), 
            name: new_name.to_string(), 
            is_interface: false, 
            ..Default::default()
        }, &id).await;
        new_classifier
    }
    
    pub async fn update_classifier_name(&self, id: &str, new_name: &str) -> Classifier {
        let mut classifier = self.repository.query_by_id(id).await.unwrap();
        classifier.name = new_name.to_string();
        // a bit ugly, but we need to copy the id because "edit" owns the containing struct
        let id = classifier._id.clone();
        let updated = self.repository.edit(&id, classifier).await;
        updated
    }

    
}

impl ActionHandler<ClassifierAction> for ClassifierService {
    fn handle_action(&self, action: ClassifierAction) -> Result<ClassifierAction, serde_json::Error> {
        let response = match action {
            ClassifierAction::RenameClassifier(data) => {
                // self.update_classifier_name(&data.new_name);
                ClassifierAction::ClassifierRenamed(data)
            },
            ClassifierAction::CancelClassifierRename =>
                ClassifierAction::ClassifierRenameCanceled(
                    EditNameDto { new_name: String::from("Old Classname") }
                )
            ,
            _ => ClassifierAction::ClassifierRenameError
        };
        Ok(response)
    }
}

impl ActionHandler<ApplicationAction> for ClassifierService {
    fn handle_action(&self, action: ApplicationAction) -> Result<ApplicationAction, serde_json::Error> {
        let response = match action {
            _ => ApplicationAction::ClassiffiersLoaded
        };
        Ok(response)
    }
}

impl ActionReceiver for ClassifierService {
    fn receive_action(&self, json_action: serde_json::Value) -> Result<serde_json::Value, serde_json::Error> {
        convert_and_handle_action::<ClassifierAction>(json_action, self)
    }
    fn domain(&self) -> &str { CLASSIFIER_DOMAIN}
}

// TODO: we need a way to register more than one action handler per service
// for example, the classifier service must be able to handle classifier actions and application actions
// We could implement the trait for every action and let the service register the action
/*

example:
in service:
    // we could use a closure here maybe?
    // but the closure needs to be typed? not good...
    handlers.register("classifier", |a| convert_and_handle_action::<ClassifierAction>(a, self))
 */

 pub fn test() {
    let mut action_handlers = HashMap::<String, Box<fn(Value) -> Value> >::new();
    
 }