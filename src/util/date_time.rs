pub struct Time{
    hour : u8,
    minute : u8,
    second : u8,
}

impl Time{
    pub fn new(hour : u8, minute : u8, second : u8) -> Self{
        Self{hour : hour, minute : minute, second : second} 
    }
}

pub struct Date{
    year : u16,
    month : u8,
    day : u8,
}

impl Date{
    pub fn new(year : u16, month : u8, day : u8) -> Self{
        Self{year : year, month : month, day : day}
    }
}

pub struct DateTime{
    date : Date,
    time : Time,
}
