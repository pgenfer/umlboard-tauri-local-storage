use serde::{Serialize, Deserialize};
use strum_macros::Display;
use ts_rs::TS;

use crate::action_handler::ActionHandler;

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

pub struct ClassifierService {}
impl ClassifierService {
    pub fn update_classifier_name(&self, new_name: &str) -> () {/* TODO: implement */}
}
impl ActionHandler for ClassifierService {
    type TActionType = ClassifierAction;

    fn domain(&self) -> &str { CLASSIFIER_DOMAIN}
    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {
        let response = match action {
            ClassifierAction::RenameClassifier(data) => {
                self.update_classifier_name(&data.new_name);
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