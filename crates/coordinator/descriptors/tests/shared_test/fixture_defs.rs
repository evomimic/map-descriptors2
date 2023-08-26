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

use crate::shared_test::HolonDescriptorTestCase;
use core::panic;
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_holon_descriptor, new_integer_descriptor,
    new_string_descriptor, update_boolean_descriptor, update_string_descriptor,
};
use descriptors::property_map_builder::{insert_property_descriptor, remove_property_descriptor};
use hdk::prelude::properties;
use rstest::*;
use std::collections::BTreeMap;
// use hdk::prelude::*;
use crate::shared_test::property_descriptor_data_creators::{
    create_example_property_descriptors, create_example_updates_for_property_descriptors,
};
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails,
    PropertyDescriptorMap,
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
    create_example_property_descriptors(&mut descriptor.properties);

    test_data_set.push(descriptor);

    // ----------------  HOLON WITH COMPOSITE PROPERTY -------------------------------
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_composite_properties"),
        "A holon type that has a single property of a composite property type.".to_string(),
        false,
    )?;

    // Composite Property Descriptor
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());

    create_example_property_descriptors(&mut composite_properties);

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

// Builds initial HolonDescriptor with no properties
#[fixture]
pub fn add_properties() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_no_properties()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let properties: PropertyDescriptorMap =
        create_example_property_descriptors(&mut updated_descriptor.properties)?;

    for (name, property) in properties.properties {
        insert_property_descriptor(&mut updated_descriptor.properties, name, &property);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };
    // println!("Original update: {:#?}", &test_case.original);
    // println!("Expected updates: {:?}", &test_case.updates);

    Ok((test_case))
}

#[fixture]
pub fn remove_properties() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_scalar()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    for (name, property) in original_descriptor.properties.properties.clone() {
        remove_property_descriptor(&mut updated_descriptor.properties, name);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };
    // println!("Original update: {:#?}", &test_case.original);
    // println!("Expected updates: {:?}", &test_case.updates);

    Ok((test_case))
}

// Builds initial HolonDescritor with each type of scalar property
#[fixture]
pub fn update_each_scalar_details() -> Result<HolonDescriptorTestCase, DescriptorsError> {
    let original_descriptor = build_holon_descriptor_with_scalar()?;
    let mut updated_descriptor = original_descriptor.clone();
    let mut updates = Vec::new();

    let update_properties =
        create_example_updates_for_property_descriptors(&mut updated_descriptor.properties)?;

    for (name, property) in update_properties.properties.clone() {
        insert_property_descriptor(&mut updated_descriptor.properties, name, &property);
        updates.push(updated_descriptor.clone());
    }

    let test_case = HolonDescriptorTestCase {
        original: original_descriptor,
        updates: updates,
    };

    // println!("{:#?}", test_case);
    Ok((test_case))
}

// // Builds initial HolonDescriptor with a composite property
// #[fixture]
// pub fn add_string_property_to_composite() -> Result<HolonDescriptorTestCase, DescriptorsError> {
//     let original_descriptor = build_holon_descriptor_with_composite()?;
//     let mut updated_descriptor = original_descriptor.clone();

//     let string_descriptor_property: PropertyDescriptor = new_string_descriptor(
//         "example_string_property_inside_composite".to_string(),
//         "adding string property to a composite".to_string(),
//         true,
//         0,
//         42,
//     )?;

//     let originalinal_composite_property_descriptor = original_descriptor
//         .properties
//         .properties
//         .get("a_composite_property");

//     if let Some(descriptor) = originalinal_composite_property_descriptor {
//         let mut composite_descriptor_map = get_composite_descriptor_map(&descriptor.details);
//         composite_descriptor_map.properties.insert(
//             "another_string_property".to_string(),
//             string_descriptor_property,
//         );

//         let updated_composite_descriptor = PropertyDescriptor {
//             header: descriptor.header.clone(),
//             details: PropertyDescriptorDetails::Composite(CompositeDescriptor {
//                 properties: composite_descriptor_map.clone(),
//             }),
//         };

//         insert_property_descriptor(
//             &mut updated_descriptor.properties,
//             "a_composite_property".to_string(),
//             &updated_composite_descriptor,
//         );

//         let update_data = HolonDescriptorTestCase {
//             original: original_descriptor,
//             updates: updated_descriptor,
//         };
//         // println!("Originalinal & expected update: {:#?}", update_data);
//         return Ok((update_data));
//     } else {
//         panic!("error getting composite");
//     }
// }

// #[fixture]
// pub fn remove_string_property_from_composite(
//     add_string_property_to_composite: Result<HolonDescriptorTestCase, DescriptorsError>,
// ) -> Result<HolonDescriptorTestCase, DescriptorsError> {
//     let mut data = add_string_property_to_composite?;
//     let original_descriptor = data.original;
//     let mut updated_descriptor = data.updates;

//     let originalinal_composite_property_descriptor = original_descriptor
//         .properties
//         .properties
//         .get("a_composite_property");

//     if let Some(descriptor) = originalinal_composite_property_descriptor {
//         let mut composite_descriptor_map = get_composite_descriptor_map(&descriptor.details);
//         composite_descriptor_map
//             .properties
//             .remove("another_string_property");

//         let updated_composite_descriptor = PropertyDescriptor {
//             header: descriptor.header.clone(),
//             details: PropertyDescriptorDetails::Composite(CompositeDescriptor {
//                 properties: composite_descriptor_map.clone(),
//             }),
//         };

//         insert_property_descriptor(
//             &mut updated_descriptor.properties,
//             "a_composite_property".to_string(),
//             &updated_composite_descriptor,
//         );

//         let update_data = HolonDescriptorTestCase {
//             originalinal: originalinal_descriptor,
//             updates: updated_descriptor,
//         };
//         // println!("{:#?}", update_data);
//         return Ok((update_data));
//     } else {
//         panic!("error getting composite");
//     }
// }

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
    create_example_property_descriptors(&mut descriptor.properties);
    Ok(descriptor)
}

fn build_holon_descriptor_with_composite() -> Result<HolonDescriptor, DescriptorsError> {
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("", BaseType::Holon, "_with_composite_properties"),
        "A holon type that has a single property of a composite property type.".to_string(),
        false,
    )?;
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    create_example_property_descriptors(&mut composite_properties);
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

    Ok(descriptor)
}

fn get_composite_descriptor_map(details: &PropertyDescriptorDetails) -> PropertyDescriptorMap {
    match details {
        PropertyDescriptorDetails::Composite(map) => map.properties.clone(),
        _ => panic!("error matching composite details"),
    }
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
