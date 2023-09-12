use shared_types_descriptor::property_descriptor::{
    PropertyDescriptorMap, PropertyDescriptorUsage,
};

///
/// PropertyMapBuilder and its associated methods provide a common way to insert various types
/// of PropertyDescriptors into a BTreeMap that can be leveraged by both Holon and Composite
///

pub fn upsert_property_descriptor(
    property_map: &mut PropertyDescriptorMap,
    property_name: String,
    property_usage: &PropertyDescriptorUsage,
) -> () {
    property_map
        .properties
        .insert(property_name, property_usage.clone());
}

pub fn remove_property_descriptor(
    property_map: &mut PropertyDescriptorMap,
    property_name: String,
) -> () {
    property_map.properties.remove(&property_name);
}
