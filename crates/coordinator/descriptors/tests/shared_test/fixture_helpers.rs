use crate::shared_test::test_data_types::{
    HolonDescriptorTestCase, PropertyDescriptorTestCase, SharedTypesTestCase,
};
use core::panic;
use std::collections::btree_map::BTreeMap;
use descriptors::helpers::{get_composite_descriptor_from_details, get_composite_descriptor_map};
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_holon_descriptor, new_integer_descriptor,
    new_string_descriptor,
};
use descriptors::property_map_builder::{remove_property_descriptor, upsert_property_descriptor};
use rstest::*;

// use hdk::prelude::*;
use crate::shared_test::property_descriptor_data_creators::{
    create_example_property_descriptors, create_example_updates_for_property_descriptors,
};
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::{HolonDescriptor, HolonReference};
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, DescriptorSharing, IntegerFormat, PropertyDescriptor,
    PropertyDescriptorDetails, PropertyDescriptorMap, PropertyDescriptorUsage,
};
use shared_types_descriptor::type_header::BaseType;

pub fn derive_type_name(prefix: &str, base_type: BaseType, suffix: &str) -> String {
    let base_type_string = base_type.to_string() + "_Type";
    let result = if prefix.is_empty() {
        if suffix.is_empty() {
            base_type_string
        } else {
            format!("{base_type_string}_{suffix}")
        }
    } else {
        // prefix is NOT empty
        if suffix.is_empty() {
            format!("{prefix}_{base_type_string}")
        } else {
            format!("{prefix}_{base_type_string}_{suffix}")
        }
    };
    result.to_string()
}


// fn derive_type_description(type_name: String)-> String {
//    format!("{type_name},_description")
//}

/// This function creates a rich test dataset by creating a vector of HolonDescriptors of various
/// kinds -- from simple to complex


// Private local fns

fn build_holon_descriptor_with_no_properties() -> Result<HolonDescriptor, DescriptorsError> {
    let descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_no_properties"),
        "A simple holon type that has no properties.".to_string(),
        false,
    )?;
    Ok(descriptor)
}

fn build_holon_descriptor_with_scalar() -> Result<HolonDescriptor, DescriptorsError> {
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_scalar_properties"),
        "A holon type that has a single property of each scalar property type.".to_string(),
        false,
    )?;
    let _unused_result = create_example_property_descriptors(&mut descriptor.properties);
    Ok(descriptor)
}

fn build_holon_descriptor_with_composite() -> Result<HolonDescriptor, DescriptorsError> {
    let mut holon_descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_composite_properties"),
        "A holon type that has a single property of a composite property type.".to_string(),
        false,
    )?;
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    // Adds properties of each scalar type
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;
    let composite_descriptor = new_composite_descriptor(
        derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties"),
        "Simple Composite Property Type description".to_string(),
        true,
        composite_properties,
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "example composite usage description".to_string(),
        composite_descriptor,
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut holon_descriptor.properties,
        "a_composite_property".to_string(),
        &composite_usage,
    );

    Ok(holon_descriptor)
}

fn build_property_descriptor_with_composite() -> Result<PropertyDescriptor, DescriptorsError> {
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;
    let composite_descriptor = new_composite_descriptor(
        derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties"),
        "Simple Composite Property Type description".to_string(),
        true,
        composite_properties.clone(),
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "new composite usage description".to_string(),
        composite_descriptor.clone(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut composite_properties,
        "a_composite_property".to_string(),
        &composite_usage,
    );

    Ok(composite_descriptor)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_dummy_data() {
//         let data = create_dummy_data(()).unwrap();

//         println!("{:#?}", data);
//     }
// }

