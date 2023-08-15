use shared_types_descriptor::property_descriptor::{PropertyDescriptor, PropertyDescriptorMap};

///
/// PropertyMapBuilder and its associated methods provide a common way to insert various types
/// of PropertyDescriptors into a BTreeMap that can be leveraged by both Holon and Composite
///

pub fn insert_property_descriptor(
    property_map: &mut PropertyDescriptorMap,
    property_name: String,
    property_descriptor: &PropertyDescriptor,
) -> () {
    property_map
        .properties
        .insert(property_name, property_descriptor.clone());
}

pub fn remove_property_descriptor(
    property_map: &mut PropertyDescriptorMap,
    property_name: String,
) -> () {
    property_map.properties.remove(&property_name);
}
