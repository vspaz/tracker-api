use std::any::Any;
use std::iter::Map;
use serde::Serialize;


pub struct Context {

}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub integrations: Map<String, dyn Any>,
    pub user_id: String,
    pub message: String,
    pub sent_at: String,
    pub workspace_id: String,
    pub message_id: String,
    pub write_key: String,
    pub event_type: String,
    pub event: String,
    pub context: Context,
    pub properties: Map<String, dyn Any>,
    pub received_at: String,
    pub timestamp: String,
    pub traits: Map<String, dyn Any>,
}
