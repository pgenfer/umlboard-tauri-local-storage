use async_trait::async_trait;
use serde_json::Value;
use crate::{repository::Repository, classifier::Classifier, action_handler::{ActionHandler, ActionDispatcher}, actions::{classifier_action::{CLASSIFIER_DOMAIN, ClassifierAction, EditNameDto}, application_action::{ApplicationAction, ClassifierDto}}};

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
                    EditNameDto { id, new_name: classifier.name }
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
        let response = match action {
            ApplicationAction::ApplicationReady => {
                // check if there is already a classifier, if not, create one
                let mut classifiers = self.load_classifiers().await;
                if classifiers.len() == 0 {
                    let new_classifier = self.create_new_classifier("new classifier").await;
                    classifiers.push(new_classifier);
                }
                // convert entities to DTOs and return them
                ApplicationAction::ClassifiersLoaded(
                    classifiers
                        .into_iter()
                        .map(|c| ClassifierDto{id: c._id, name: c.name})
                        .collect()
                )
            },
            _ => ApplicationAction::ApplicationLoadError
        };
        return response;
    }
}