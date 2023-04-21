use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use strum_macros::Display;
use ts_rs::TS;

use crate::{repository::Repository, classifier::Classifier, action_handler::{ActionHandler, CLASSIFIER_DOMAIN, ActionDispatcher}};

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct EditNameDto {
    pub id: String,
    pub new_name: String
}

#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum ClassifierAction {
    RenameClassifier(EditNameDto),
    CancelClassifierRename { id: String },
    ClassifierRenamed(EditNameDto),
    ClassifierRenameCanceled(EditNameDto),
    ClassifierRenameError
}

#[derive(Serialize, Deserialize, Display)]
pub enum ApplicationAction {
    ClassiffiersLoaded
}


pub const APPLICATION_DOMAIN: &str = "application";

pub struct ClassifierService {
    repository : Box<dyn Repository<Classifier> + Send + Sync>
}
impl<'a> ClassifierService {
    
    pub fn new(
    classifier_repository: Box<dyn Repository<Classifier> + Send + Sync>) -> Self { 
        Self { repository: classifier_repository }
    }

    pub async fn load_classifiers(&self) -> Vec<Classifier> {
        let classifiers = self.repository.query_all().await;
        classifiers
    }

    pub async fn get_by_id(&self, id: &str) -> Classifier {
        self.repository.query_by_id(id).await.unwrap()
    }

    pub async fn create_new_classifier(&self, new_name: &str) -> Classifier {
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

#[async_trait]
impl ActionDispatcher for ClassifierService {
    async fn dispatch_action(&self, domain: String, action: Value) ->  Value {
        if domain == CLASSIFIER_DOMAIN {
            ActionHandler::<ClassifierAction>::convert_and_handle(self, action).await
        } else {
            ActionHandler::<ApplicationAction>::convert_and_handle(self, action).await
        }
    }
}

#[async_trait]
impl ActionHandler<ClassifierAction> for ClassifierService {
    async fn handle_action(&self, action: ClassifierAction) -> ClassifierAction {
        let response = match action {
            ClassifierAction::RenameClassifier(data) => {
                let classifier = self.update_classifier_name(&data.id, &data.new_name).await;
                ClassifierAction::ClassifierRenamed(
                    EditNameDto{ id: classifier._id, new_name: classifier.name}
                )
            },
            ClassifierAction::CancelClassifierRename{id} => {
                let classifier = self.get_by_id(&id).await;
                ClassifierAction::ClassifierRenameCanceled(
                    EditNameDto { id, new_name: String::from("Old Classname") }
                )
            },
            _ => ClassifierAction::ClassifierRenameError
        };
        return response;
    }
}

#[async_trait]
impl ActionHandler<ApplicationAction> for ClassifierService {
    async fn handle_action(&self, action: ApplicationAction) -> ApplicationAction {
        panic!();
    }
}