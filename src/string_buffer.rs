use std::collections::HashMap;
use crate::fs_base::Error;
use crate::traits::Serde;

const STRING_BUFFER_PREAMBLE : &str = "STRING_BUFFER";

pub struct StringBuffer{
    name_map : HashMap<String, u32>,
    serialization_size : u32,
    next_index : u32,
}

impl StringBuffer{
    pub fn new() -> Self{
       Self{name_map : HashMap::new(), serialization_size : 0, next_index : 0} 
    }
    pub fn from(name_map0 : HashMap<String, u32>, serialization_size0 : u32, next_index0 : u32) -> Self{
        Self{name_map : name_map0, serialization_size : serialization_size0, next_index : next_index0 }
    }
    pub fn get(&self, s : &String) -> Result<u32, Error>{
       match self.name_map.get(s) {
            Some(num) => { Ok(*num) }
            None => {
                Err(Error::InvalidDirective(format!("Bad string: {} ", s)))
            }
       }
    }
    pub fn add(&mut self, name : &String) -> u32{
        self.name_map.insert(name.clone(), self.next_index);
        let copy = self.next_index;
        self.next_index += 1;
        return copy;
    }

    fn serialization_size(&self) -> usize{
        let mut size = self.name_map.len() * size_of::<u32>();
        for (key, _) in self.name_map.iter(){
            size += key.len();
        }
        size
    }

    fn map_as_vec(&self) -> Vec<String>{
        // --- REWRITE: too many string copies
        // there are some unnecessary string copies being made here. However I can't really 
        // dive deep into ownership yet. I just want the project to work right now
        let mut result = Vec::new();
        result.resize(self.name_map.len(), String::new());
        
        for (key, value ) in self.name_map.iter(){
            result[*value as usize] = key.clone();
        }
        result
    }

    fn deser_str_len(buffer : &[u8]) -> Result<u32, Error>{
        // --- REWRITE: This piece of code lowkey doesn't look idiomatic
        let bytes_opt= buffer.get(0..4);
        if bytes_opt == None {
            return Err(
                Error::InvalidStringBuffer("Did not find enough bytes to parse string length".to_string()));
        }
        let bytes =  bytes_opt.unwrap();
        let byte_arr = [ bytes[0], bytes[1], bytes[2], bytes[3] ];

        Ok(u32::from_le_bytes(byte_arr))
    }
    fn deser_str(buffer : &[u8], length : usize) -> Result<String, Error> {
        let bytes_opt = buffer.get(0..length);
        if bytes_opt == None {
            return Err(
                Error::InvalidStringBuffer("Did no provide the correct number of bytes for the string".to_string()));
        }
        let _ = match String::from_utf8(bytes_opt.unwrap().to_vec()){
            Ok(res) => {
                Ok(res)
            }

            Err(err) => {
                Err(Error::InvalidStringBuffer(err.to_string()))
            }
        };
        Ok(String::new())
    }
}

impl Serde for StringBuffer{
    fn serialize(&self) -> Vec<u8>{
        // --- REWRITE: too many string copies
        let mut buffer = Vec::with_capacity(self.serialization_size() + STRING_BUFFER_PREAMBLE.len());
        buffer.extend_from_slice(STRING_BUFFER_PREAMBLE.as_bytes());
        let sorted_map = self.map_as_vec(); 
        
        for item in sorted_map.iter() {
            buffer.extend_from_slice(item.as_bytes());
        }
        
        buffer
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        let mut name_map = HashMap::<String, u32>::new();
        let mut serialization_size = 0 ;
        let mut next_index = 0;

        if buffer.starts_with(STRING_BUFFER_PREAMBLE.as_bytes()) == false {
            return Err(Error::InvalidPreamble("Did not find the preamble specific to the string buffer".to_string()));
        }

        while buffer.len() != 0 {
            let str_size = StringBuffer::deser_str_len(buffer)?;
            *buffer = &buffer[4..];
            let parsed_str = StringBuffer::deser_str(buffer, str_size as usize)?;
            *buffer = &buffer[str_size as usize..];
            name_map.insert(parsed_str, next_index);

            next_index += 1;
        }

        Ok(
            StringBuffer::from(name_map, serialization_size, next_index)
        )
    }
}


