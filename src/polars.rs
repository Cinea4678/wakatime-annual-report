//! 使用Polars对大规模的数据进行分析

use std::fs::File;
use std::time;
use chrono::{Datelike, FixedOffset, Timelike, TimeZone, Utc};
use polars::prelude::*;
use polars::series::ops::NullBehavior;
use crate::report::NormalReport;

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
            .filter(|h| h.time.is_some())
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
        let mut language_vec: Vec<Option<String>> = get_vec(len);
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

            language_vec.push(heartbeat.language.clone());
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
            Series::new("language", language_vec,
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

    fn get_normal_report(&self, timeout: f64) -> NormalReport {
        let lazy = self.df.head(Some(10)).clone().lazy();


        todo!()
    }
}

impl PolarsReportBuilder {
    pub fn test(&self) {
        println!("Record count: {}", self.df.shape().0);
        let lf = self.df.clone().lazy();
        get_total_time(&lf, 900.0);
    }
}

/// 本函数用于将心跳数据转换为编程总时间
///
/// 算法参考官方FAQ：<a>https://wakatime.com/faq#timeout</a>
fn get_total_time(lf: &LazyFrame, timeout: f64) {
    let d = lf.clone()
        .select([
            col("time_raw"),
            col("time_raw").diff(1, NullBehavior::Ignore).alias("diff")
        ])
        .filter(
            col("diff").lt(timeout)
        )
        .select([
            col("diff").sum().alias("sum_diff")
        ])
        .collect().unwrap();

    println!("{}", d);

    if let AnyValue::Float64(sum) = d.get(0).unwrap()[0] {
        let duration = time::Duration::from_millis((sum * 1000.0) as u64);
        let duration = chrono::Duration::from_std(duration).unwrap();
        println!("Total duration: {} hours {} minutes {} seconds", duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60);
    }

    ()
}
