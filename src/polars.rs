//! 使用Polars对大规模的数据进行分析

use std::ops::Add;

use chrono::{Datelike, FixedOffset, Timelike, TimeZone, Utc};
use log::{error, warn};
use polars::prelude::*;
use polars::series::ops::NullBehavior;

use crate::report::NormalReport;
use crate::ReportBuilder;
use crate::wakatime::WakaTimeBackupData;

/// 本trait用于将心跳数据转换为编程总时间
///
/// 算法参考官方FAQ：<a>https://wakatime.com/faq#timeout</a>
trait TimeSum {
    fn time_sum(&self, timeout: f64) -> f64;
}

impl TimeSum for DataFrame {
    fn time_sum(&self, timeout: f64) -> f64 {
        if self.get_column_index("time_raw").is_none() {
            error!("function time_sum(&df) requires DataFrame has column 'time_raw'.");
            panic!()
        }
        if self.shape().0 <= 1 {
            return 0.0;
        }
        if timeout <= 0.0 {
            warn!("a timeout which is equal or below to 0 will cause no result.")
        }

        let d = self.clone().lazy()
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

        if let AnyValue::Float64(sum) = d.get(0).unwrap()[0] {
            sum
        } else {
            error!("Error: the calculating result is abnormal.\n{}", d);
            panic!()
        }
    }
}

impl TimeSum for Series {
    fn time_sum(&self, timeout: f64) -> f64 {
        let df = DataFrame::new(vec![self.clone().with_name("time_raw")]).unwrap();
        df.time_sum(timeout)
    }
}

/// 本trait用于在Frame中加入一列人类可读的时间
trait HumanTimeDecorator {
    fn add_human_time(self, column_name: &str) -> Self;
}

impl HumanTimeDecorator for LazyFrame {
    fn add_human_time(self, column_name: &str) -> Self {
        let new_column_name = String::from(column_name).add("_human");
        self
            .with_column(
                col(column_name).map(|s| {
                    let new_s: Vec<_> = s.iter().map(|v| {
                        match v {
                            AnyValue::Float64(v) => {
                                let duration = chrono::Duration::milliseconds((v * 1000.0) as i64);
                                format!("{} hr. {} min. {} sec.", duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60)
                            }
                            _ => String::from("<Error>")
                        }
                    }).collect();

                    Ok(Some(Series::new("", new_s)))
                }, GetOutput::from_type(DataType::Utf8)).alias(new_column_name.as_str())
            )
    }
}

/// 本trait用于将两列的DataFrame（通常是Key - Value的形式），保留顺序转换成元组列表
trait AsTupleVec {
    fn as_tuple_vec<F, T, R>(&self, get_key: F) -> Vec<(T, R)>
        where F: Fn(AnyValue) -> T, R: polars::export::num::NumCast;
}

impl AsTupleVec for DataFrame {
    fn as_tuple_vec<F, T, R>(&self, get_key: F) -> Vec<(T, R)>
        where F: Fn(AnyValue) -> T, R: polars::export::num::NumCast
    {
        if self.shape().1 != 2 {
            error!("function as_tuple_vec() should be used in a DataFrame which has exactly 2 columns.");
            panic!()
        }

        let mut vec: Vec<(T, R)> = Vec::new();

        let len = self.shape().0;
        (0..len).for_each(|i| {
            let v = self.get(i).unwrap();

            vec.push((get_key(v[0].clone()), v[1].clone().try_extract().unwrap()))
        });


        vec
    }
}

pub struct PolarsReportBuilder {
    df: DataFrame,
    year: i32,
    timeout: f64,
}

impl ReportBuilder for PolarsReportBuilder {
    fn from_raw_data(data_raw: WakaTimeBackupData, year: i32, time_zone: f64, time_out: f64) -> Self {
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
        let mut hour_min_vec: Vec<u32> = get_vec(len);
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
            hour_min_vec.push(t.hour() * 100 + t.minute());

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
            Series::new("hour_min", hour_min_vec),
            Series::new("language", language_vec,
            ),
            Series::new("project", project_vec),
            Series::new("user_agent_id", user_agent_id_vec),
        ]).unwrap();

        Self {
            df,
            year,
            timeout: time_out,
        }
    }

    fn get_normal_report(&self) -> NormalReport {
        let year = self.year;

        let total_time = self.get_total_time();

        let time_by_language_df = self.get_time_by_language();
        let time_by_language: Vec<(_, f64)> = time_by_language_df.as_tuple_vec(|v| {
            String::from(v.get_str().unwrap())
        });

        let time_by_project_df = self.get_time_by_project();
        let time_by_project: Vec<(_, f64)> = time_by_project_df.as_tuple_vec(|v| {
            String::from(v.get_str().unwrap())
        });

        let time_by_month_df = self.get_time_by_month();
        let time_by_month: Vec<(_, f64)> = time_by_month_df.as_tuple_vec(|v| {
            v.try_extract::<i32>().unwrap()
        });

        let time_by_month_day_df = self.get_time_by_month_day();
        let time_by_month_day: Vec<(_, f64)> = time_by_month_day_df.as_tuple_vec(|v| {
            v.try_extract::<i32>().unwrap()
        });

        let time_by_weekday_df = self.get_time_by_weekday();
        let time_by_weekday: Vec<(_, f64)> = time_by_weekday_df.as_tuple_vec(|v| {
            String::from(v.get_str().unwrap())
        });

        let time_by_day_df = self.get_time_by_day();
        let time_by_day: Vec<(_, f64)> = time_by_day_df.as_tuple_vec(|v| {
            v.try_extract::<i32>().unwrap()
        });

        let time_by_hours_df = self.get_time_by_hours();
        let time_by_hours: Vec<(_, f64)> = time_by_hours_df.as_tuple_vec(|v| {
            v.try_extract::<i32>().unwrap()
        });

        let late_night_time_df = self.get_late_nights();
        let late_night_time: Vec<(_, f64)> = late_night_time_df.as_tuple_vec(|v| {
            v.try_extract::<i32>().unwrap()
        });

        NormalReport {
            year,
            total_time,
            time_by_language,
            time_by_project,
            time_by_month,
            time_by_month_day,
            time_by_weekday,
            time_by_day,
            time_by_hours,
            late_night_time,
        }
    }
}

impl PolarsReportBuilder {
    pub fn get_total_time(&self) -> f64 {
        return self.df.time_sum(self.timeout);
    }

    pub fn get_time_by_language(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("language").is_not_null()
            )
            .group_by(["language"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .limit(15)
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_project(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("project").is_not_null()
            )
            .group_by(["project"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .limit(15)
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_month(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("month").is_not_null()
            )
            .group_by(["month"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_month_day(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("day").is_not_null()
            )
            .group_by(["day"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_weekday(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("weekday").is_not_null()
            )
            .group_by(["weekday"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_day(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("year_day").is_not_null()
            )
            .group_by(["year_day"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .limit(5)
            .collect()
            .unwrap();

        out
    }

    pub fn get_time_by_hours(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("hour_min").is_not_null()
            )
            .with_column(
                col("hour_min")
                    .map(
                        |series|
                            Ok(Some(
                                series.u32()?
                                    .into_iter()
                                    .map(|opt_v| match opt_v {
                                        Some(v) if v / 100 <= 12 => Some(v / 100),
                                        _ => None
                                    })
                                    .collect::<UInt32Chunked>()
                                    .into_series()
                            )),
                        GetOutput::from_type(DataType::UInt32),
                    )
                    .alias("hour")
            )
            .filter(
                col("hour").is_not_null()
            )
            .group_by(["hour"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .collect()
            .unwrap();

        out
    }

    pub fn get_late_nights(&self) -> DataFrame {
        let lf = self.df.clone().lazy();

        let time_out = self.timeout;

        let out = lf
            .filter(
                col("hour_min").is_not_null()
            )
            .filter(
                col("hour_min").lt(530)     // 5:30之前算熬夜
            )
            .group_by(["year_day"])
            .agg([
                col("time_raw").apply(move |s| {
                    let res = s.time_sum(time_out);
                    Ok(Some(Series::new("", &[res])))
                }, GetOutput::float_type()).alias("time_sum").first()
            ])
            // .add_human_time("time_sum")
            .sort("time_sum", SortOptions {
                descending: true,
                ..Default::default()
            })
            .filter(
                col("time_sum").gt(0)
            )
            .collect()
            .unwrap();

        out
    }
}
