use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub build: String,
    pub namespace: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub app_info: AppInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub integrations: HashMap<String, String>,
    pub user_id: String,
    pub message: String,
    pub sent_at: String,
    pub workspace_id: String,
    pub message_id: String,
    pub write_key: String,
    pub event_type: String,
    pub event: String,
    pub context: Context,
    pub properties: HashMap<String, String>,
    pub received_at: String,
    pub timestamp: String,
    pub traits: HashMap<String, String>,
}
