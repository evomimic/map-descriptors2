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

use crate::shared_test::test_data_types::{SharedTypesTestCase, ValueDescriptorTestCase};
use descriptors::helpers::{get_composite_descriptor_from_details, get_composite_descriptor_map};
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_integer_descriptor, new_string_descriptor,
};
use descriptors::property_map_builder::upsert_property_descriptor;
use rstest::*;
use std::collections::btree_map::BTreeMap;
use tracing::Level;

// use hdk::prelude::*;
use crate::shared_test::fixture_helpers::{
    derive_label, derive_type_description, derive_type_name,
};
use crate::shared_test::property_descriptor_data_creators::{
    create_example_property_descriptors, create_example_updates_for_property_descriptors,
};
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonReference;
use shared_types_descriptor::type_header::BaseType;
use shared_types_descriptor::value_descriptor::{
    DescriptorSharing, PropertyDescriptorMap, PropertyDescriptorUsage, ValueDescriptor,
    ValueDescriptorDetails,
};

#[fixture]
pub fn new_dedicated_value_descriptors_fixture() -> Result<Vec<ValueDescriptor>, DescriptorsError> {
    let mut test_data_set: Vec<ValueDescriptor> = Vec::new();

    // ----------------  VALUE  DESCRIPTOR WITH STRING VALUE TYPE -------------------------------
    let type_name = derive_type_name("simple", BaseType::String, "example");
    let string_descriptor = new_string_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        0,
        100,
    )?;
    test_data_set.push(string_descriptor);

    // ----------------  VALUE  DESCRIPTOR WITH INTEGER VALUE TYPE -------------------------------
    let type_name = derive_type_name("simple", BaseType::Integer, "example");
    let integer_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        -3.168e9 as i64,
        4.44e9 as i64,
    )?;
    test_data_set.push(integer_descriptor);

    // ----------------  VALUE  DESCRIPTOR WITH BOOLEAN VALUE TYPE -------------------------------
    let type_name = derive_type_name("simple", BaseType::Boolean, "example");
    let boolean_descriptor = new_boolean_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        false,
    )?;
    test_data_set.push(boolean_descriptor);

    // ----------------  VALUE  DESCRIPTOR WITH COMPOSITE VALUE TYPE -------------------------------
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;

    let type_name = derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties");
    let composite_descriptor = new_composite_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        composite_properties.clone(),
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "example composite usage description".to_string(),
        composite_descriptor.clone(),
        "a composite property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut composite_properties,
        "a_composite_property".to_string(),
        &composite_usage,
    );
    test_data_set.push(composite_descriptor);

    Ok(test_data_set)
}

#[fixture]
pub fn new_shared_value_descriptors_fixture() -> Result<SharedTypesTestCase, DescriptorsError> {
    let message_threshold = Level::INFO;
    // This fixture creates a vector of scalar types (shared_types)
    // Then creates a vector of composite types that reference those shared types

    let mut shared_types: Vec<ValueDescriptor> = Vec::new();

    let type_name = derive_type_name("Shared", BaseType::String, "example");
    let string_descriptor = new_string_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
        0,
        100,
    )?;
    shared_types.push(string_descriptor);

    let type_name = derive_type_name("Shared_", BaseType::Integer, "example");
    let integer_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
        -3.168e9 as i64,
        4.44e9 as i64,
    )?;
    shared_types.push(integer_descriptor);

    let type_name = derive_type_name("Shared", BaseType::Boolean, "example");
    let boolean_descriptor = new_boolean_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
        false,
    )?;
    shared_types.push(boolean_descriptor);

    let mut referencing_types: Vec<ValueDescriptor> = Vec::new(); // composites

    let mut composite1_properties = PropertyDescriptorMap::new(BTreeMap::new());

    // Now that we have that composite, iterate through the shared types and define one property
    // of each type
    let mut i = 1;
    for shared_type in &shared_types {
        let usage = PropertyDescriptorUsage::new(
            "testing referenced usage of from composite".to_string(),
            shared_type.clone(),
            "a shared property".to_string(),
            DescriptorSharing::Shared(HolonReference {
                id: None,
                name: Some(shared_type.header.type_name.clone()),
            }),
        );
        upsert_property_descriptor(
            &mut composite1_properties,
            format!("test_composite_property_{}", i),
            &usage,
        );
        i += 1;
    }
    let type_name = derive_type_name("TestComposite1_", BaseType::Composite, "");
    let composite_descriptor1 = new_composite_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
        composite1_properties.clone(),
    )?;
    referencing_types.push(composite_descriptor1.clone());

    let test_case = SharedTypesTestCase {
        shared_types,
        referencing_types,
        message_level: message_threshold,
    };

    Ok(test_case)
}

#[fixture]
pub fn update_value_descriptor_composite() -> Result<ValueDescriptorTestCase, DescriptorsError> {
    let message_threshold = Level::DEBUG;
    let original_descriptor = build_value_descriptor_with_composite()?;
    let mut updates = Vec::new();

    let mut composite_descriptor =
        get_composite_descriptor_from_details(&original_descriptor.details);
    let mut descriptor_map = get_composite_descriptor_map(&original_descriptor.details);

    let update_properties = create_example_updates_for_property_descriptors(&mut descriptor_map)?;

    for (name, property) in update_properties.properties.clone() {
        upsert_property_descriptor(&mut descriptor_map, name, &property);
    }

    composite_descriptor.property_map = descriptor_map;

    let updated_descriptor = ValueDescriptor {
        header: original_descriptor.header.clone(),
        details: ValueDescriptorDetails::Composite(composite_descriptor),
    };

    updates.push(updated_descriptor.clone());

    let test_case = ValueDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
        message_level: message_threshold,
    };
    // println!("Original & expected update: {:#?}", test_case);
    Ok(test_case)
}

fn build_value_descriptor_with_composite() -> Result<ValueDescriptor, DescriptorsError> {
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;

    let type_name = derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties");
    let composite_descriptor = new_composite_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        composite_properties.clone(),
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "new composite usage description".to_string(),
        composite_descriptor.clone(),
        "a composite property".to_string(),
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
