use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NormalReport {
    /// 年份
    pub year: i32,

    /// 总时间
    pub total_time: f64,

    /// 按语言分类的总时间统计
    pub time_by_language: Vec<(String, f64)>,

    /// 按项目分类的总时间统计（前30个）
    pub time_by_project: Vec<(String, f64)>,

    /// 按月份分类的总时间统计
    pub time_by_month: Vec<(i32, f64)>,

    /// 按月份内的日期分类的总时间统计
    pub time_by_month_day: Vec<(i32, f64)>,

    /// 按星期内的日期分类的总时间统计
    pub time_by_weekday: Vec<(String, f64)>,

    /// 按年内某天的总时间统计（前15个）
    pub time_by_day: Vec<(i32, f64)>,

    /// 按每天的时段统计
    pub time_by_hours: Vec<(i32, f64)>,

    /// 深夜加班（key是年内日期，从1开始）
    pub late_night_time: Vec<(i32, f64)>,
}
