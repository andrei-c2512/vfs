use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Time{
    pub hour : u8,
    pub minute : u8,
    pub second : u8,
}

impl Time{
    pub fn new() -> Self{
        Self{hour : 0, minute : 0, second : 0 } 
    }
    pub fn now() -> Self{
        let now = SystemTime::now();

        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let total_seconds = duration_since_epoch.as_secs();

        // Compute hours, minutes, seconds in UTC
        let seconds_in_day = total_seconds % 86400; // 24*60*60
        let hour = seconds_in_day / 3600;
        let minute = (seconds_in_day % 3600) / 60;
        let second = seconds_in_day % 60;
        Self {
            hour : hour as u8,
            minute : minute as u8,
            second : second as u8
        }
    }
    pub fn from(hour : u8, minute : u8, second : u8) -> Self{
        Self{hour, minute, second} 
    }
}

impl Default for Time{
    fn default() -> Self{
        Self::new()
    }
}
impl fmt::Display for Time{
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    } 
}

#[derive(PartialEq, Eq)]
pub struct Date{
    pub year : u16,
    pub month : u8,
    pub day : u8,
}

impl Date{
    pub fn new() -> Self{
        Self { year : 0, month :0 , day : 0}
    }
    pub fn now() -> Self {
        let output = Command::new("date")
            .arg("+%Y-%m-%d")
            .output().unwrap();

        let date_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        let data_vec : Vec<&str> = date_str.split('-').collect();
        Self {
            year : data_vec[0].parse().unwrap(),
            month : data_vec[1].parse().unwrap(),
            day :data_vec[2].parse().unwrap(),
        }
    }
    pub fn from(year : u16, month : u8, day : u8) -> Self{
        Self{year, month, day}
    }
}

impl Default for Date{
    fn default() -> Self{
        Self::new()
    }
}


impl fmt::Display for Date{
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.day, self.month, self.year)
    } 
}

#[derive(PartialEq, Eq)]
pub struct DateTime{
    pub date : Date,
    pub time : Time,
}

impl DateTime{
    pub fn new() -> Self {
        Self{ date : Date::new(), time : Time::new() }
    }
    pub fn now() -> Self{
        Self{ date : Date::now(), time : Time::now()} 
    }
    pub fn from(date : Date, time : Time) -> Self{
        Self{date, time}
    }
}

impl Default for DateTime{
    fn default() -> Self{
        Self::new()
    }
}

impl fmt::Display for DateTime{
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    } 
}
