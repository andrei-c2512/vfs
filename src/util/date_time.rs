pub struct Time{
    pub hour : u8,
    pub minute : u8,
    pub second : u8,
}

impl Time{
    pub fn new() -> Self{
        Self{hour : 0, minute : 0, second : 0 } 
    }
    pub fn from(hour : u8, minute : u8, second : u8) -> Self{
        Self{hour : hour, minute : minute, second : second} 
    }
}

pub struct Date{
    pub year : u16,
    pub month : u8,
    pub day : u8,
}

impl Date{
    pub fn new() -> Self{
        Self { year : 0, month :0 , day : 0}
    }
    pub fn from(year : u16, month : u8, day : u8) -> Self{
        Self{year : year, month : month, day : day}
    }
}

pub struct DateTime{
    pub date : Date,
    pub time : Time,
}

impl DateTime{
    pub fn new() -> Self {
        Self{ date : Date::new(), time : Time::new() }
    }
    pub fn from(date : Date, time : Time) -> Self{
        Self{date : date, time : time}
    }
}
