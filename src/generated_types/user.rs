// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::User;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: User = serde_json::from_str(&json).unwrap();
// }

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub bio: Option<serde_json::Value>,
    pub city: Option<serde_json::Value>,
    pub color_scheme: Option<String>,
    pub created_at: Option<String>,
    pub date_format: Option<String>,
    pub default_dashboard_range: Option<String>,
    pub display_name: Option<String>,
    pub durations_slice_by: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<serde_json::Value>,
    pub github_username: Option<serde_json::Value>,
    pub has_basic_features: Option<bool>,
    pub has_premium_features: Option<bool>,
    pub human_readable_website: Option<serde_json::Value>,
    pub id: Option<String>,
    pub invoice_id_format: Option<String>,
    pub is_email_confirmed: Option<bool>,
    pub is_email_public: Option<bool>,
    pub is_hireable: Option<bool>,
    pub is_onboarding_finished: Option<bool>,
    pub languages_used_public: Option<bool>,
    pub last_branch: Option<String>,
    pub last_heartbeat_at: Option<String>,
    pub last_plugin: Option<String>,
    pub last_plugin_name: Option<String>,
    pub last_project: Option<String>,
    pub linkedin_username: Option<serde_json::Value>,
    pub location: Option<serde_json::Value>,
    pub logged_time_public: Option<bool>,
    pub meetings_over_coding: Option<bool>,
    pub modified_at: Option<String>,
    pub needs_payment_method: Option<bool>,
    pub photo: Option<String>,
    pub photo_public: Option<bool>,
    pub plan: Option<String>,
    pub profile_url: Option<String>,
    pub profile_url_escaped: Option<String>,
    pub public_email: Option<serde_json::Value>,
    pub public_profile_time_range: Option<String>,
    pub share_all_time_badge: Option<serde_json::Value>,
    pub share_last_year_days: Option<serde_json::Value>,
    pub show_machine_name_ip: Option<bool>,
    #[serde(rename = "time_format_24hr")]
    pub time_format_24_hr: Option<serde_json::Value>,
    pub time_format_display: Option<String>,
    pub timeout: Option<i64>,
    pub timezone: Option<String>,
    pub twitter_username: Option<serde_json::Value>,
    pub username: Option<serde_json::Value>,
    pub website: Option<serde_json::Value>,
    pub weekday_start: Option<i64>,
    #[serde(rename = "wonderfuldev_username")]
    pub wonderful_dev_username: Option<serde_json::Value>,
    pub writes_only: Option<bool>,
}

