use crate::property_descriptor::PropertyDescriptorMap;
use crate::type_header::TypeHeader;
use derive_new::new;
use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
pub struct HolonDescriptor {
    pub header: TypeHeader,
    // pub properties: PropertyDescriptorMap,
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonCollectionDescriptor {
    pub header: TypeHeader,
    pub contains_items_of_type: HolonDescriptor,
    pub min_items: u32,
    pub max_items: u32,
    pub unique_items: bool, // true means duplicate items are not allowed
    pub is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}
