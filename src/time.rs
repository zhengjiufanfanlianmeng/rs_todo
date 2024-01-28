use std::time::SystemTime;
use chrono::{DateTime, Local};

pub fn get_time() -> Result<String, Box<dyn std::error::Error>> {
    // 获取当前的系统时间
    // 获取当前的系统时间
    let system_now = SystemTime::now();

    // 将系统时间转换为DateTime<Local>类型
    let datetime: DateTime<Local> = system_now.into();

    // 格式化显示时间
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // 打印出格式化后的本地时间
    // println!("{}", timestamp_str);

    Ok(timestamp_str)
}