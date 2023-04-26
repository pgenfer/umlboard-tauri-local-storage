use serde::{Serialize, Deserialize};
use ts_rs::TS;
use strum_macros::Display;

pub const APPLICATION_DOMAIN: &str = "application";

/// data type for transfering Classifier entities
/// between core and webview process
#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/classifier-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct ClassifierDto {
    pub id: String,
    pub name: String
}

/// Actions for controlling the program flow
#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum ApplicationAction {
    // fired when the web view process is finished with loading
    ApplicationReady,
    // used by core process to return available classifiers 
    // during startup
    ClassifiersLoaded(Vec<ClassifierDto>),
    // error state, not used yet
    ApplicationLoadError
}