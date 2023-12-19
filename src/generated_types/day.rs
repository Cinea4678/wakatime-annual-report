// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Day;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Day = serde_json::from_str(&json).unwrap();
// }

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub date: String,
    pub heartbeats: Vec<Heartbeat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heartbeat {
    pub branch: Option<serde_json::Value>,
    pub category: Option<String>,
    pub created_at: String,
    pub cursorpos: Option<i64>,
    pub dependencies: Option<Vec<Option<serde_json::Value>>>,
    pub entity: Option<String>,
    pub id: Option<String>,
    pub is_write: Option<bool>,
    pub language: Option<String>,
    pub lineno: Option<i64>,
    pub lines: Option<i64>,
    pub machine_name_id: Option<String>,
    pub project: Option<String>,
    pub project_root_count: Option<serde_json::Value>,
    pub time: Option<f64>,
    #[serde(rename = "type")]
    pub heartbeat_type: Option<String>,
    pub user_agent_id: Option<String>,
    pub user_id: Option<String>,
}
