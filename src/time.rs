use chrono::{DateTime, Local};

pub fn get_current_time() -> String {
    let now: DateTime<Local> = Local::now();
    // 格式化为指定的格式
    let time = now.format("%Y-%m-%d %H:%M").to_string();
    time
}
