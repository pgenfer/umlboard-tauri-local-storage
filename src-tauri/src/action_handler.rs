use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub trait ActionHandler {
    type TActionType: DeserializeOwned + Serialize + std::fmt::Display;

    fn domain(&self) -> &str;
    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error>;    
    fn receive_action(&self, json_action: Value) -> Result<Value, serde_json::Error> {
        // convert json to action
        let incoming: Self::TActionType = serde_json::from_value(json_action)?;
        // call action specific handler
        let response = self.handle_action(incoming)?;
        // convert response to json
        let response_json = serde_json::to_value(response)?;
        Ok(response_json)
    }
}