use serde::{Serialize, Deserialize};
use ts_rs::TS;
use strum_macros::Display;

pub const APPLICATION_DOMAIN: &str = "application";

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/classifier-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct ClassifierDto {
    pub id: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum ApplicationAction {
    ApplicationReady,
    ClassifiersLoaded(Vec<ClassifierDto>),
    ApplicationLoadError
}