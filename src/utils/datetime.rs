use chrono::{Datelike, Local, NaiveDateTime};

pub fn get_year_now() -> i32{
    let now = Local::now();

    return now.year();
}

// pub fn get_date_time_now_string() -> String{

//     let now = Local::now();

//     return format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
// }

pub fn get_date_time_now() -> NaiveDateTime {
    return Local::now().naive_local();
}