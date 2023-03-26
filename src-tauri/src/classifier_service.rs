use std::error::Error;

use serde::{Serialize, Deserialize};
use strum_macros::Display;
use ts_rs::TS;

use crate::{action_handler::ActionHandler, repository::Repository, classifier::{Classifier, ClassifierEntity}};

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

// we need one file with constants, like
// this will also be generated for redux to use a slice name
pub const CLASSIFIER_DOMAIN: &str = "classifier";

pub struct ClassifierService<'a> {
    repository : &'a dyn Repository<Classifier>
}
impl<'a> ClassifierService<'a> {
    
    pub fn new(classifier_repository: &'a dyn Repository<Classifier>) -> Self { 
        Self { repository: classifier_repository } 
    }

    pub fn load_classifiers(&self) -> Vec<ClassifierEntity> {
        self.repository.query_all()
    }

    pub fn create_new_classifier(&self, new_name: &str) -> ClassifierEntity { // TODO: use repository error
        self.repository.insert(Classifier{name: new_name.to_string(), isInterface: false, ..Default::default()})
    }
    
    pub fn update_classifier_name(&self, id: &str, new_name: &str) -> ClassifierEntity {
        let mut classifier = self.repository.query_by_id(id).unwrap();
        classifier.content.name = new_name.to_string();
        let updated = self.repository.edit(&classifier.id, classifier.content).unwrap();
        updated
    }

    
}
impl ActionHandler for ClassifierService<'_> {
    type TActionType = ClassifierAction;

    fn domain(&self) -> &str { CLASSIFIER_DOMAIN}
    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {
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