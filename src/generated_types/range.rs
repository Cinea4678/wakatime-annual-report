// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Range;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Range = serde_json::from_str(&json).unwrap();
// }

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub end: i64,
    pub start: i64,
}
