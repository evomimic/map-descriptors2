use crate::shared_test::data_fixtures::{derive_type_name};
use descriptors::mutators::{new_boolean_descriptor, new_integer_descriptor, new_string_descriptor};
use descriptors::property_map_builder::{insert_property_descriptor};
// use hdk::prelude::*;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails, PropertyDescriptorMap};
use shared_types_descriptor::type_header::BaseType;

/// This function adds a set of PropertyDescriptors of various Scalar Types to supplied PropertyMap
///
pub fn create_property_descriptors(property_descriptor_map: &mut PropertyDescriptorMap)
    -> Result<(),DescriptorsError> {

    // Add Boolean Descriptor
    let descriptor = new_boolean_descriptor(
                derive_type_name("simple",BaseType::Boolean,""),
                "Simple Boolean Property Type description".to_string(),
                true,
                false,
            )?;
    insert_property_descriptor(
        property_descriptor_map,
        "a_boolean_property".to_string(),
        &descriptor,
    );

    // Add String Descriptor
    let descriptor = new_string_descriptor(
            derive_type_name("simple_",BaseType::String,""),
            "Simple String Property Type description".to_string(),
            true,
            0,
            2048,
        )?;
    insert_property_descriptor(
        property_descriptor_map,
        "a_string_property".to_string(),
        &descriptor,
    );

    // Add Integer I8
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_I8",BaseType::Integer,""),
        "Simple Integer (I8) Property Type description".to_string(),
        true,
        IntegerFormat::I8(),
        -127,
        127,
    )?;
    insert_property_descriptor(
        property_descriptor_map,
        "an_I8_property".to_string(),
        &descriptor,
    );

    // Integer I16
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_I16",BaseType::Integer,""),
        "Simple Integer (I16) Property Type description".to_string(),
        true,
        IntegerFormat::I16(),
        -32767,
        32767,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "an_I16_property".to_string(),
        &descriptor,
    );

    // Integer I32
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_I32",BaseType::Integer,""),
        "Simple Integer (I32) Property Type description".to_string(),
        true,
        IntegerFormat::I32(),
        -2147483648,
        2147483648,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "an_I32_property".to_string(),
        &descriptor,
    );

    // Integer I64
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_I64",BaseType::Integer,""),
        "Simple Integer (I64) Property Type description".to_string(),
        true,
        IntegerFormat::I64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "an_I64\
        _property".to_string(),
        &descriptor,
    );

    // Integer U8
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U8",BaseType::Integer,""),
        "Simple Integer (U8) Property Type description".to_string(),
        true,
        IntegerFormat::U8(),
        -127,
        127,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U8_property".to_string(),
        &descriptor,
    );

    // Integer U16
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U16",BaseType::Integer,""),
        "Simple Integer (U16) Property Type description".to_string(),
        true,
        IntegerFormat::U16(),
        -32767,
        32767,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U16_property".to_string(),
        &descriptor,
    );

    // Integer U32
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U32",BaseType::Integer,""),
        "Simple Integer (U32) Property Type description".to_string(),
        true,
        IntegerFormat::U32(),
        -2147483648,
        2147483648,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U32_property".to_string(),
        &descriptor,
    );

    // Integer U64
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U64",BaseType::Integer,""),
        "Simple Integer (U64) Property Type description".to_string(),
        true,
        IntegerFormat::U64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U64_property".to_string(),
        &descriptor,
    );

    Ok(())
    
}



