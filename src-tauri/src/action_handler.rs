use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// general interface used to receive actions.
/// Structs should implement this trait and call 
/// convert_and_handle_action(...)
pub trait ActionReceiver {
    fn domain(&self) -> &str;
    fn receive_action(&self, json_action: Value) -> Result<Value, serde_json::Error>; 
}

/// generic action handler for specific action types, must be implemented by application services
pub trait ActionHandler<TActionType> 
    where TActionType: DeserializeOwned + Serialize + std::fmt::Display {
    fn handle_action(&self, action: TActionType) -> Result<TActionType, serde_json::Error>; 
}

/// helper function to convert and handle action
pub fn convert_and_handle_action<T>(json_action: Value, handler: &dyn ActionHandler<T>) -> Result<Value, serde_json::Error> 
where T: DeserializeOwned + Serialize + std::fmt::Display {
    // convert json to action
    let incoming: T = serde_json::from_value(json_action)?;
    // call action specific handler
    let response = handler.handle_action(incoming)?;
    // convert response to json
    let response_json = serde_json::to_value(response)?;
    Ok(response_json)
}