pub mod generated_types;
mod polars;
mod wakatime;
pub mod report;

use std::fs::File;
use std::io::BufReader;
use anyhow::Result;
use crate::polars::PolarsReportBuilder;
use crate::report::NormalReport;

/// 实现了这个trait的实现都可以被用来生成我们的报告
trait ReportBuilder {

    /// 导入原始的json数据
    ///
    /// time_zone: 格式为小时的偏移数（例如，GMT +8 = 8，GMT +8.5 = 8.5）
    fn from_raw_data(data: wakatime::WakaTimeBackupData, year: i32, time_zone: f64) -> Self;

    /// 生成报表
    fn get_normal_report(&self, timeout: f64) -> NormalReport;
}

/// 从给定的路径导入JSON
pub fn read_json_from_file(path: &str) -> Result<wakatime::WakaTimeBackupData> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

/// 制作报告
pub fn get_analyses_report_data(data: wakatime::WakaTimeBackupData, year: i32, time_zone: f64) -> Result<()> {

    let user_data = data.user.clone(); // 保留一份用户数据备用

    let report_builder = PolarsReportBuilder::from_raw_data(data, year, time_zone);

    report_builder.test();

    Ok(())
}

