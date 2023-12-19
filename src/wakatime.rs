use serde_derive::{Deserialize, Serialize};
use chrono::{Datelike, TimeZone, Utc};

use crate::generated_types::{day, range, user};

#[derive(Serialize, Deserialize)]
pub struct WakaTimeBackupData {
    pub user: user::User,
    pub range: range::Range,
    pub days: Vec<day::Day>,
}

impl WakaTimeBackupData {

    /// 获取可被分析的年份
    pub fn get_available_years(self: &Self) -> Vec<i32> {
        let start = Utc.timestamp_opt(self.range.start,0).unwrap();
        let end = Utc.timestamp_opt(self.range.end,0).unwrap();

        (start.year() ..= end.year()).collect()
    }

}
