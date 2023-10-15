use crate::holon_descriptor::HolonReference;
use crate::type_header::TypeHeader;
use derive_new::new;
use hdi::prelude::*;
use std::collections::BTreeMap;

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
pub struct PropertyDescriptor {
    pub header: TypeHeader,
    pub details: PropertyDescriptorDetails,
}

#[hdk_entry_helper]
#[derive(Default, Clone, PartialEq, Eq)]
pub enum DescriptorSharing {
    #[default]
    Dedicated,
    Shared(HolonReference),
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
pub struct PropertyDescriptorUsage {
    pub description: String,
    pub descriptor: PropertyDescriptor,
    pub label: String,
    pub sharing: DescriptorSharing,
}

/// PropertyMap contains a set of (property_name, PropertyDescriptorUsage) pairs
/// that can be used in various contexts. For example, by HolonDescriptor and CompositeDescriptor
///
#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorMap {
    pub properties: BTreeMap<String, PropertyDescriptorUsage>,
}

///
/// PropertyDescriptor enumerates the subset of TypeDescriptors whose instances cannot exist
/// independent of a parent instance. In other words, they cannot be identified or stored
/// independently of their parent instance
#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PropertyDescriptorDetails {
    Boolean(BooleanDescriptor),
    Composite(CompositeDescriptor),
    //Enum(EnumDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor),
    ValueCollection(ValueCollectionDescriptor), // can only contain collections of PropertyTypes (not Holons)
}

#[hdk_entry_helper]
#[derive(new, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BooleanDescriptor {
    pub is_fuzzy: bool, // if true, this property has FuzzyBoolean value, otherwise just true or false
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompositeDescriptor {
    pub property_map: PropertyDescriptorMap,
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerDescriptor {
    pub min_value: i64,
    pub max_value: i64,
}


#[hdk_entry_helper]
#[derive(new, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StringDescriptor {
    pub min_length: u32,
    pub max_length: u32,
    //pattern: String,
}

// This is just a first cut at ValueCollectionDescriptor
// It identifies the kinds of items the collection contains via a string

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ValueCollectionDescriptor {
    pub contains_items_of_type: String,
    // TODO: replace this with a ref
    pub min_items: u32,
    pub max_items: u32,
    pub unique_items: bool,
    // true means duplicate items are not allowed
    pub is_ordered: bool, // if items have an intrinsic order
}
