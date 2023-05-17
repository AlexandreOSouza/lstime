use chrono::prelude::*;

fn main() {
    let local_time = Local::now();
    let time_zone_offset = 7;
    let tz = FixedOffset::west(time_zone_offset * 3600);
    let time_with_time_zone = local_time.with_timezone(&tz).format("%H:%M:%S");
    let my_local_time = local_time.format("%H:%M:%S");
    println!("BRT: {} | PST: {}", my_local_time, time_with_time_zone);
}

