//! 使用Polars对大规模的数据进行分析

use chrono::{Datelike, FixedOffset, Timelike, TimeZone, Utc};
use polars::prelude::*;

use crate::ReportBuilder;
use crate::wakatime::WakaTimeBackupData;

pub struct PolarsReportBuilder {
    df: DataFrame,
    year: i32,
    time_zone_offset: FixedOffset,
}

impl ReportBuilder for PolarsReportBuilder {
    fn from_raw_data(data_raw: WakaTimeBackupData, year: i32, time_zone: f64) -> Self {
        let time_zone_offset = FixedOffset::east_opt((time_zone * 3600.0) as i32).unwrap();

        let data: Vec<_> = data_raw.days.iter().flat_map(|d| &d.heartbeats)
            .filter(|h| h.time.is_some() && h.language.is_some())
            .filter(|h| {
                let time = h.time.unwrap();
                let t = Utc.timestamp_millis_opt((time * 1000.0) as i64);
                let t = t.unwrap().with_timezone(&time_zone_offset);
                t.year() == year
            })
            .collect();

        let len = data.len();
        fn get_vec<T>(len: usize) -> Vec<T> {
            let mut v = Vec::new();
            v.reserve(len);
            v
        }

        let mut time_raw_vec: Vec<f64> = get_vec(len);
        let mut month_vec: Vec<u32> = get_vec(len);
        let mut day_vec: Vec<u32> = get_vec(len);
        let mut year_day_vec: Vec<u32> = get_vec(len);
        let mut weekday_vec: Vec<String> = get_vec(len);
        let mut hour_vec: Vec<u32> = get_vec(len);
        let mut language_vec: Vec<String> = get_vec(len);
        let mut project_vec: Vec<Option<String>> = get_vec(len);
        let mut user_agent_id_vec: Vec<Option<String>> = get_vec(len);

        for heartbeat in data {
            let time = heartbeat.time.unwrap();
            let t = Utc.timestamp_millis_opt((time * 1000.0) as i64);
            let t = t.unwrap().with_timezone(&time_zone_offset);

            time_raw_vec.push(time);
            month_vec.push(t.month());
            day_vec.push(t.day());
            year_day_vec.push(t.ordinal());
            weekday_vec.push(t.weekday().to_string());
            hour_vec.push(t.hour());

            language_vec.push(heartbeat.language.clone().unwrap());
            project_vec.push(heartbeat.project.clone());
            user_agent_id_vec.push(heartbeat.user_agent_id.clone());
        }

        let df = DataFrame::new(vec![
            Series::new("time_raw", time_raw_vec),
            Series::new("month", month_vec),
            Series::new("day", day_vec),
            Series::new("year_day", year_day_vec),
            Series::new("weekday", weekday_vec),
            Series::new("hour", hour_vec),
            Series::new("language", language_vec
            ),
            Series::new("project", project_vec),
            Series::new("user_agent_id", user_agent_id_vec),
        ]).unwrap();

        Self {
            df,
            year,
            time_zone_offset,
        }
    }
}

impl PolarsReportBuilder {
    pub fn test(&self) {
        println!("Record count: {}", self.df.shape().0);
        let res = self.df.head(Some(3));
        println!("{}", res);
    }
}
