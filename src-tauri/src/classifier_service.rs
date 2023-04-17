use serde::{Serialize, Deserialize};
use strum_macros::Display;
use ts_rs::TS;

use crate::{repository::Repository, classifier::Classifier};

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct EditNameDto {
    pub new_name: String
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