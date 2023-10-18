#![allow(dead_code)]

use descriptors::mutators::{new_composite_descriptor, new_holon_descriptor};
use descriptors::property_map_builder::upsert_property_descriptor;
use std::collections::btree_map::BTreeMap;

// use hdk::prelude::*;
use crate::shared_test::property_descriptor_data_creators::create_example_property_descriptors;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    DescriptorSharing, ValueDescriptor, PropertyDescriptorMap, PropertyDescriptorUsage,
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

pub fn derive_label(type_name: &str) -> String {
    format!("human readable label for {type_name}")
}

pub fn derive_type_description(type_name: &str) -> String {
    format!("description for {type_name}")
}

/// This function creates a rich test dataset by creating a vector of HolonDescriptors of various
/// kinds -- from simple to complex

// Private local fns

fn build_holon_descriptor_with_no_properties() -> Result<HolonDescriptor, DescriptorsError> {
    let type_name = derive_type_name("", BaseType::Holon, "_with_no_properties");
    let descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;
    Ok(descriptor)
}

fn build_holon_descriptor_with_scalar() -> Result<HolonDescriptor, DescriptorsError> {
    let type_name = derive_type_name("", BaseType::Holon, "_with_scalar_properties");
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
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
        derive_type_description(&type_name),
        derive_label(&type_name),
        false,
    )?;
    let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
    // Adds properties of each scalar type
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

    Ok(holon_descriptor)
}

fn build_property_descriptor_with_composite() -> Result<ValueDescriptor, DescriptorsError> {
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
