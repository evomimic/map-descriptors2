// Test Dataset Creator
//
// This file is used to create dummy data in support of various testing scenarios
// Some observations:
// - To verify a "get" we need to first "create" the record
// - To verify a "create", we need follow the create with a "get"
// - So... we don't really need independent tests of both. So omit "Get" tests.
//
// Creating many descriptors depends on creating a single descriptor,
// so we don't need a separate test for single creates
//
// The logic for CUD tests is identical, what varies is the test data.
// BUT... if the test data set has all different variations in it, we may only need 1 test data set

use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_holon_descriptor,
};
use rstest::*;
use std::collections::BTreeMap;

pub use descriptors::property_map_builder::insert_property_descriptor;
// use hdk::prelude::*;
use crate::shared_test::property_descriptor_data_creators::create_property_descriptors;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    IntegerFormat, PropertyDescriptorDetails, PropertyDescriptorMap,
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

#[fixture]
pub fn rs_dummy_data() -> Result<Vec<HolonDescriptor>, DescriptorsError> {
    let mut test_data_set: Vec<HolonDescriptor> = Vec::new();

    // ----------------  HOLON WITH NO PROPERTIES-------------------------------
    let descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_no_properties"),
        "A simple holon type that has no properties.".to_string(),
        false,
    )?;

    test_data_set.push(descriptor);

    // ----------------  HOLON WITH SINGLE BOOLEAN PROPERTY -------------------------------

    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_single_boolean_property"),
        "A simple holon type that has a single boolean property".to_string(),
        false,
    )?;

    let bool_descriptor = new_boolean_descriptor(
        derive_type_name("simple", BaseType::Boolean, ""),
        "Simple Boolean Property Type description".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "a_boolean_property".to_string(),
        &bool_descriptor,
    );
    test_data_set.push(descriptor);

    // ----------------  HOLON WITH SCALAR PROPERTIES -------------------------------
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_scalar_properties"),
        "A holon type that has a single property of each scalar property type.".to_string(),
        false,
    )?;
    create_property_descriptors(&mut descriptor.properties);

    test_data_set.push(descriptor);

    // ----------------  HOLON WITH COMPOSITE PROPERTY -------------------------------
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_composite_properties"),
        "A holon type that has a single property of a composite property type.".to_string(),
        false,
    )?;

    // Composite Property Descriptor
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());

    create_property_descriptors(&mut composite_properties);

    let comp_descriptor = new_composite_descriptor(
        derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties"),
        "Simple Composite Property Type description".to_string(),
        true,
        composite_properties,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "a_composite_property".to_string(),
        &comp_descriptor,
    );

    test_data_set.push(descriptor);

    Ok(test_data_set)
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
