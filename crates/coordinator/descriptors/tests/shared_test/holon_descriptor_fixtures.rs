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

#![allow(dead_code)]


use core::panic;
use descriptors::helpers::*;
use descriptors::mutators::*;
use descriptors::property_map_builder::*;
use rstest::*;
use std::collections::btree_map::BTreeMap;

// use hdk::prelude::*;

use crate::shared_test::test_data_types::HolonDescriptorTestCase;
use crate::shared_test::fixture_helpers::{derive_type_name, derive_type_description, derive_label};
use crate::shared_test::property_descriptor_data_creators::{
    create_example_property_descriptors, create_example_updates_for_property_descriptors,
};
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::value_descriptor::{
    CompositeDescriptor, DescriptorSharing, ValueDescriptor,
    ValueDescriptorDetails, PropertyDescriptorMap, PropertyDescriptorUsage,
};
use shared_types_descriptor::type_header::BaseType;

/// This function creates a rich test dataset by creating a vector of HolonDescriptors of various
/// kinds -- from simple to complex
#[fixture]
pub fn new_holons_fixture() -> Result<Vec<HolonDescriptor>, DescriptorsError> {
    let mut test_data_set: Vec<HolonDescriptor> = Vec::new();

    // ----------------  HOLON WITH NO PROPERTIES-------------------------------
    let type_name = derive_type_name("", BaseType::Holon, "_with_no_properties");
    let holon_descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;

    test_data_set.push(holon_descriptor);

    // ----------------  HOLON WITH SINGLE BOOLEAN PROPERTY -------------------------------
    let type_name = derive_type_name("", BaseType::Holon, "_with_single_boolean_property");
    let mut holon_descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;

    let type_name = derive_type_name("simple", BaseType::Boolean, "");
    let boolean_descriptor = new_boolean_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
        false,
    )?;
    let boolean_usage = PropertyDescriptorUsage::new(
        "example boolean usage description".to_string(),
        boolean_descriptor,
        "a boolean property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut holon_descriptor.property_map,
        "a_boolean_property".to_string(),
        &boolean_usage,
    );
    test_data_set.push(holon_descriptor);

    // ----------------  HOLON WITH SCALAR PROPERTIES -------------------------------
    let type_name = derive_type_name("", BaseType::Holon, "_with_scalar_properties");
    let mut holon_descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;
    let _unused_result = create_example_property_descriptors(&mut holon_descriptor.property_map)?;

    test_data_set.push(holon_descriptor);

    // ----------------  HOLON WITH COMPOSITE PROPERTY -------------------------------
    let type_name = derive_type_name("", BaseType::Holon, "_with_composite_properties");
    let mut holon_descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;

    // Composite Property Descriptor
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());

    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;

    let type_name = derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties");
    let composite_descriptor = new_composite_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        composite_properties,
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "example composite usage description".to_string(),
        composite_descriptor,
        "a composite property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut holon_descriptor.property_map,
        "a_composite_property".to_string(),
        &composite_usage,
    );

    test_data_set.push(holon_descriptor);

    Ok(test_data_set)
}

// Builds initial HolonDescriptor with no properties
#[fixture]
pub fn add_properties() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_no_properties()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let properties: PropertyDescriptorMap =
        create_example_property_descriptors(&mut updated_descriptor.property_map)?;

    for (name, property) in properties.properties {
        upsert_property_descriptor(&mut updated_descriptor.property_map, name, &property);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };
    // println!("Original update: {:#?}", &test_case.original);
    // println!("Expected updates: {:?}", &test_case.updates);

    Ok(test_case)
}

#[fixture]
pub fn remove_properties() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_scalar()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    for (name, _property) in original_descriptor.property_map.properties.clone() {
        remove_property_descriptor(&mut updated_descriptor.property_map, name);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };
    // println!("Original update: {:#?}", &test_case.original);
    // println!("Expected updates: {:?}", &test_case.updates);

    Ok(test_case)
}

// Builds initial HolonDescritor with each type of scalar property
#[fixture]
pub fn update_each_scalar_details() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_scalar()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let update_properties =
        create_example_updates_for_property_descriptors(&mut updated_descriptor.property_map)?;

    for (name, property) in update_properties.properties.clone() {
        upsert_property_descriptor(&mut updated_descriptor.property_map, name, &property);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };

    // println!("{:#?}", test_case);
    Ok(test_case)
}

// Builds initial HolonDescriptor with a composite property
#[fixture]
pub fn add_properties_to_composite() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_composite()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let type_name = derive_type_name("new", BaseType::Boolean, "addition");
    let new_boolean_descriptor = new_boolean_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        false,
    )?;
    let boolean_usage = PropertyDescriptorUsage::new(
        "new boolean usage description".to_string(),
        new_boolean_descriptor,
        "a boolean property".to_string(),
        DescriptorSharing::default(),
    );

    let type_name = derive_type_name("new", BaseType::String, "addition");
    let new_string_descriptor = new_string_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        3,
        5000,
    )?;
    let string_usage = PropertyDescriptorUsage::new(
        "new string usage description".to_string(),
        new_string_descriptor,
        "a string property".to_string(),
        DescriptorSharing::default(),
    );

    let type_name = derive_type_name("new", BaseType::Integer, "addition");
    let new_integer_descriptor = new_integer_descriptor(
        type_name.clone(),
        "add new integer descriptor to composite property".to_string(),
        derive_label(&type_name),
        true,
        -42,
        42,
    )?;
    let integer_usage = PropertyDescriptorUsage::new(
        "new integer usage description".to_string(),
        new_integer_descriptor,
        "an integer property".to_string(),
        DescriptorSharing::default(),
    );

    let original_composite_property_descriptor = original_descriptor
        .property_map
        .properties
        .get("a_composite_property");

    if let Some(usage) = original_composite_property_descriptor {
        let mut composite_descriptor_map = get_composite_descriptor_map(&usage.descriptor.details);
        composite_descriptor_map
            .properties
            .insert("another_string_property".to_string(), string_usage);
        composite_descriptor_map
            .properties
            .insert("another_boolean_property".to_string(), boolean_usage);
        composite_descriptor_map
            .properties
            .insert("another_integer_property".to_string(), integer_usage);

        let mut new_descriptor = usage.descriptor.clone();
        new_descriptor.header.description = "reflecting added properties".to_string();

        let updated_composite_descriptor = ValueDescriptor {
            header: new_descriptor.header,
            details: ValueDescriptorDetails::Composite(CompositeDescriptor {
                property_map: composite_descriptor_map.clone(),
            }),
        };
        let composite_usage = PropertyDescriptorUsage::new(
            "new composite usage description".to_string(),
            updated_composite_descriptor,
            "a composite property".to_string(),
            DescriptorSharing::default(),
        );
        upsert_property_descriptor(
            &mut updated_descriptor.property_map,
            "a_composite_property".to_string(),
            &composite_usage,
        );
        updates.push(updated_descriptor);

        let test_case = HolonDescriptorTestCase {
            original: original_descriptor,
            updates: updates,
        };
        // println!("Original & expected update: {:#?}",test_case);
        return Ok(test_case);
    } else {
        panic!("error getting composite");
    }
}

#[fixture]
pub fn remove_properties_from_composite(
    add_properties_to_composite: Result<HolonDescriptorTestCase, DescriptorsError>,
) -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let data = add_properties_to_composite?;
    let original_descriptor = data.original;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let originalinal_composite_property_descriptor = original_descriptor
        .property_map
        .properties
        .get("a_composite_property");

    if let Some(usage) = originalinal_composite_property_descriptor {
        let mut composite_descriptor_map = get_composite_descriptor_map(&usage.descriptor.details);
        composite_descriptor_map
            .properties
            .remove("another_boolean_property");
        composite_descriptor_map
            .properties
            .remove("another_string_property");
        composite_descriptor_map
            .properties
            .remove("another_i8_property");
        composite_descriptor_map
            .properties
            .remove("another_u64_property");

        let updated_composite_descriptor = ValueDescriptor {
            header: usage.descriptor.header.clone(),
            details: ValueDescriptorDetails::Composite(CompositeDescriptor {
                property_map: composite_descriptor_map.clone(),
            }),
        };
        let composite_usage = PropertyDescriptorUsage::new(
            "new composite usage description".to_string(),
            updated_composite_descriptor,
            "a composite property".to_string(),
            DescriptorSharing::default(),
        );
        upsert_property_descriptor(
            &mut updated_descriptor.property_map,
            "a_composite_property".to_string(),
            &composite_usage,
        );
        updates.push(updated_descriptor);

        let test_case = HolonDescriptorTestCase {
            original: original_descriptor,
            updates: updates,
        };
        // println!("{:#?}", update_data);
        return Ok(test_case);
    } else {
        panic!("error getting composite");
    }
}

// Private local fns

fn build_holon_descriptor_with_no_properties() -> Result<HolonDescriptor, DescriptorsError> {
    let type_name = derive_type_name("", BaseType::Holon, "_with_no_properties");
    let descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        "A simple holon type that has no properties.".to_string(),
        derive_label(&type_name),
        false,
    )?;
    Ok(descriptor)
}

fn build_holon_descriptor_with_scalar() -> Result<HolonDescriptor, DescriptorsError> {
    let type_name = derive_type_name("", BaseType::Holon, "_with_scalar_properties");
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        "A holon type that has a single property of each scalar property type.".to_string(),
        derive_label(&type_name),
        false,
    )?;
    let _unused_result = create_example_property_descriptors(&mut descriptor.property_map);
    Ok(descriptor)
}

fn build_holon_descriptor_with_composite() -> Result<HolonDescriptor, DescriptorsError> {
    let type_name = derive_type_name("", BaseType::Holon, "_with_composite_properties");
    let mut holon_descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        "A holon type that has a single property of a composite property type.".to_string(),
        derive_label(&type_name),
        false,
    )?;
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    // Adds properties of each scalar type
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;

    let type_name = derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties");
    let composite_descriptor = new_composite_descriptor(
        type_name.clone(),
        "Simple Composite Property Type description".to_string(),
        derive_type_description(&type_name),
        true,
        composite_properties,
    )?;
    let composite_usage = PropertyDescriptorUsage::new(
        "example composite usage description".to_string(),
        composite_descriptor,
        "a composite property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        &mut holon_descriptor.property_map,
        "a_composite_property".to_string(),
        &composite_usage,
    );

    Ok(holon_descriptor)
}

fn build_property_descriptor_with_composite() -> Result<ValueDescriptor, DescriptorsError> {
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    let _unused_result = create_example_property_descriptors(&mut composite_properties)?;

    let type_name = derive_type_name("Simple_", BaseType::Composite, "_with_scalar_properties");
    let composite_descriptor = new_composite_descriptor(
        type_name.clone(),
        "Simple Composite Property Type description".to_string(),
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
