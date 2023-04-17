use serde_json::Value;
use crate::classifier_service::{ClassifierService, ClassifierAction, EditNameDto};

pub const CLASSIFIER_DOMAIN: &str = "classifier";

pub fn handle_classifier_action(action: Value, service: &ClassifierService) -> Value {
    let incoming: ClassifierAction = serde_json::from_value(action).unwrap();
    // call action specific handler
    let response = match incoming {
        ClassifierAction::RenameClassifier(data) => {
            // service.update_classifier_name(data., new_name)
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
    // convert response to json
    let response_json = serde_json::to_value(response).unwrap();
    response_json
}