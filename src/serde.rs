use crate::errors::Error;

pub trait Serde{
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(buffer : &[u8]) -> Result<Self, Error>
        where
            Self : Sized;
}


