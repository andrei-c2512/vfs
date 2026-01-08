use crate::fs_base::{Error, Permissions};
use crate::serde;
use crate::traits::Serde;
use crate::util::date_time::DateTime;

#[derive(PartialEq, Eq)]
pub struct INode {
    pub name_id: u32,
    pub permissions: Permissions,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub size: usize,
}

impl INode {
    pub fn new() -> Self {
        Self {
            name_id: 0,
            permissions: 0,
            created_at: DateTime::now(),
            last_modified: DateTime::now(),
            size: 0,
        }
    }
    pub fn from(
        name_id: u32,
        perms: Permissions,
        created_at: DateTime,
        last_modified: DateTime,
        size: usize,
    ) -> Self {
        Self {
            name_id,
            permissions: perms,
            created_at,
            last_modified,
            size,
        }
    }
}

impl Default for INode {
    fn default() -> Self {
        Self::new()
    }
}
impl Serde for INode {
    fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        // CHANGE THIS PLS I MISS C++ TEMPLATES HOLY SHIT PLEASE PLAEAS PELASE
        let bytes = self.name_id.to_be_bytes();
        result.extend_from_slice(&bytes);

        let bytes = self.permissions.to_be_bytes();
        result.extend_from_slice(&bytes);

        result.extend_from_slice(&self.created_at.serialize());
        result.extend_from_slice(&self.last_modified.serialize());

        let bytes = self.size.to_be_bytes();
        result.extend_from_slice(&bytes);

        result
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        let mut n: INode = INode::new();

        n.name_id = serde::deser_u32(buffer)?;
        n.permissions = serde::deser_u16(buffer)?;
        n.created_at = DateTime::deserialize(buffer)?;
        n.last_modified = DateTime::deserialize(buffer)?;
        n.size = serde::deser_usize(buffer)?;

        Ok(n)
    }
}
