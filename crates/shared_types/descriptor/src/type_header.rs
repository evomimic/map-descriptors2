use derive_new::new;
use hdi::prelude::*;
use std::fmt;

#[hdk_entry_helper]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
pub enum BaseType {
    Holon,
    Collection,
    Composite,
    Relationship,
    Boolean,
    Integer,
    String,
    Enum,
}
impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Holon => write!(f, "Holon"),
            BaseType::Collection => write!(f, "Collection"),
            BaseType::Composite => write!(f, "Composite"),
            BaseType::Relationship => write!(f, "Relationship"),
            BaseType::Boolean => write!(f, "Boolean"),
            BaseType::Integer => write!(f, "Integer"),
            BaseType::String => write!(f, "String"),
            BaseType::Enum => write!(f, "Enum"),
        }
    }
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}
impl Default for SemanticVersion {
    fn default() -> Self {
        SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        }
    }
}
#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeHeader {
    pub type_name: String,
    pub base_type: BaseType,
    pub description: String,
    pub version: SemanticVersion,
    pub is_dependent: bool,
}
