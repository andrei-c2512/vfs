use std::collections::HashMap;
use crate::fs_base::Error;
use crate::traits::Serde;
use crate::serde::deser_u32;

const STRING_BUFFER_PREAMBLE : &str = "STRING_BUFFER";

#[derive(PartialEq, Eq)]
pub struct StringBuffer{
    name_map : HashMap<String, u32>,
    serialization_size : u32,
    next_index : u32,
    pub string_list : Vec<String>,
}

impl StringBuffer{
    pub fn new() -> Self{
       Self{name_map : HashMap::new(), serialization_size : 0, next_index : 0, string_list : Vec::new()} 
    }
    pub fn from(name_map0 : HashMap<String, u32>, serialization_size0 : u32, next_index0 : u32, string_list : Vec<String>) -> Self{
        Self{name_map : name_map0, serialization_size : serialization_size0, next_index : next_index0, string_list : string_list}
    }
    pub fn get(&self, s : &String) -> Result<u32, Error>{
       match self.name_map.get(s) {
            Some(num) => { Ok(*num) }
            None => {
                Err(Error::InvalidDirective(format!("Bad string: {} ", s)))
            }
       }
    }
    pub fn add(&mut self, name : &str) -> u32{
        // --- REWRITE: yes yes I know I am doing 2 conversions
        // println!("Adding '{}'", name);
        
        match self.name_map.get(name) {
            Some(id) => {
                *id
            }
            None => {
                self.string_list.push(name.to_string());
                self.name_map.insert(name.to_string(), self.next_index);
                let copy = self.next_index;
                self.next_index += 1;
                copy
            }
        }
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

    fn deser_str(buffer : &[u8], length : usize) -> Result<String, Error> {
        let bytes_opt = buffer.get(0..length);
        if bytes_opt == None {
            return Err(
                Error::InvalidStringBuffer("Did no provide the correct number of bytes for the string".to_string()));
        }
        match String::from_utf8(bytes_opt.unwrap().to_vec()){
            Ok(res) => {
                return Ok(res);
            }

            Err(err) => {
                return Err(Error::InvalidStringBuffer(err.to_string()))
            }
        };
    }
}

impl Serde for StringBuffer{
    fn serialize(&self) -> Vec<u8>{
        // --- REWRITE: too many string copies
        let mut buffer = Vec::with_capacity(self.serialization_size() + STRING_BUFFER_PREAMBLE.len());
        buffer.extend_from_slice(STRING_BUFFER_PREAMBLE.as_bytes());
        let sorted_map = self.map_as_vec(); 
       
        // write down the size
        buffer.extend_from_slice(
            &(sorted_map.len() as u32).to_be_bytes()
        );
        for item in sorted_map.iter() {
            buffer.extend_from_slice(
                &(item.len() as u32).to_be_bytes()
            );
            buffer.extend_from_slice(item.as_bytes());
        }
        
        buffer
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        let mut name_map = HashMap::<String, u32>::new();
        let serialization_size = 0 ;
        let mut next_index = 0;

        if buffer.starts_with(STRING_BUFFER_PREAMBLE.as_bytes()) == false {
            return Err(Error::InvalidPreamble("Did not find the preamble specific to the string buffer".to_string()));
        }

        *buffer = &buffer[STRING_BUFFER_PREAMBLE.len()..];
        let str_list_size = deser_u32(buffer)?;
        // println!("{}", str_list_size);

        for _ in 0..str_list_size {
            let str_size = deser_u32(buffer)?;
            let parsed_str = StringBuffer::deser_str(buffer, str_size as usize)?;
            // println!("{}", parsed_str);
            *buffer = &buffer[str_size as usize..];
            name_map.insert(parsed_str, next_index);

            next_index += 1;
        }

        // --- REWRITE: duplicate code (see the StringBuffer implementation)
        let mut string_list = Vec::new();
        string_list.resize(name_map.len(), String::new());
        // println!("{}", name_map.len());
        for (key, value ) in name_map.iter(){
            string_list[*value as usize] = key.clone();
        }
        
        Ok(
            StringBuffer::from(name_map, serialization_size, next_index, string_list)
        )
    }
}


