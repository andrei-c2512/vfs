use crate::util::date_time::{Date,Time,DateTime};
use crate::traits::Serde;
use crate::fs_base::Error;



// stores the deserialized stuff in the return value. 
// Rust generics can go kill themselves. Worst fucking thing ever. C++ 20 concepts are better
// SOLVE THE FOLLOWING WITH A MACRO OR SUM
// --- REWRITE: too ugly
pub fn deser_u8(buffer : &mut &[u8]) -> Result<u8, Error>{ 
    match &buffer[0..size_of::<u8>()].try_into() {
        Ok(val) => { 
            *buffer = &buffer[size_of::<u8>()..];
            Ok(u8::from_be_bytes(*val))
        } ,
        Err(err) => { 
            //println!("Flopped u8");
            Err(Error::BadDeser(err.to_string()))
        } 
    }
}

pub fn deser_u16(buffer : &mut &[u8]) -> Result<u16, Error>{ 
    match &buffer[0..size_of::<u16>()].try_into() {
        Ok(val) => { 
            *buffer = &buffer[size_of::<u16>()..];
            Ok(u16::from_be_bytes(*val))
        } ,
        Err(err) => { 
            //println!("Flopped u16");
            Err(Error::BadDeser(err.to_string())) 
        } 
    }
}

pub fn deser_u32(buffer : &mut &[u8]) -> Result<u32, Error>{ 
    //println!("Deserialising {:?}", buffer);
    match &buffer[0..size_of::<u32>()].try_into() {
        Ok(val) => { 
            *buffer = &buffer[size_of::<u32>()..];
            Ok(u32::from_be_bytes(*val))
        } ,
        Err(err) => {
            //println!("Flopped u32");
            Err(Error::BadDeser(err.to_string()))
        } 
    }
}

pub fn deser_usize(buffer : &mut &[u8]) -> Result<usize, Error>{ 
    match &buffer[0..size_of::<usize>()].try_into() {
        Ok(val) => { 
            *buffer = &buffer[size_of::<usize>()..];
            Ok(usize::from_be_bytes(*val))
        } ,
        Err(err) => {
            //println!("Flopped usize");
            Err(Error::BadDeser(err.to_string())) 
        } 
    }
}

/* END PLS MODIFY THIS IT S SO SO UGLY */


pub fn ser_vec_u32(vec : &[u32]) -> Vec<u8> {
    // +1 because the front is gonna be the length of the list
    let mut res = Vec::with_capacity(size_of::<u32>() * (vec.len() + 1));
    res.extend_from_slice(&(vec.len() as u32).to_be_bytes()); 
    for nr in vec.iter() {
        res.extend_from_slice(&nr.to_be_bytes()); 
    }
    res
}

pub fn deser_vec_u32(buffer : &mut &[u8]) -> Result<Vec<u32>, Error> {
    let capacity = deser_u32(buffer)?;
    //println!("Deserialising a list of {} u32's", capacity);
    let mut res = Vec::with_capacity(capacity as usize);

    for _ in 0..capacity {
        res.push(
            deser_u32(buffer)?
        );
    }
    Ok(res)
}

impl Serde for Date{
    fn serialize(&self) -> Vec<u8>{
        let mut result = Vec::new();
        // dog code 
        result.extend_from_slice(&self.day.to_be_bytes());
        result.extend_from_slice(&self.month.to_be_bytes());
        result.extend_from_slice(&self.year.to_be_bytes());

        result
    }

    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        let mut date = Date::new();

        date.day = deser_u8(buffer)?; 
        date.month = deser_u8(buffer)?;
        let year_bytes : &[u8;2] = &buffer[0..size_of::<u16>()].try_into().unwrap();
        date.year = u16::from_be_bytes(*year_bytes);
        *buffer = &buffer[size_of::<u16>()..];

        Ok(date)
    }
}

// pretty similar to the last one. My C++ ahh is drooling to somehow template this
impl Serde for Time { 
    fn serialize(&self) -> Vec<u8>{
        let mut result = Vec::new();
        // dog code 
        result.extend_from_slice(&self.hour.to_be_bytes());
        result.extend_from_slice(&self.minute.to_be_bytes());
        result.extend_from_slice(&self.second.to_be_bytes());

        result
    }

    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        let mut time= Time::new();

        time.hour = deser_u8(buffer)?;
        time.minute = deser_u8(buffer)?;
        time.second = deser_u8(buffer)?;

        Ok(time)
    }
}

impl Serde for DateTime{
    fn serialize(&self) -> Vec<u8>{
        let mut result = Vec::new();
        result.extend_from_slice(
            &self.date.serialize()
        );
        result.extend_from_slice(
            &self.time.serialize()
        );

        result
    }

    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        let date = Date::deserialize(buffer)?;
        let time = Time::deserialize(buffer)?;

        Ok(DateTime::from(date, time))
    }
}
