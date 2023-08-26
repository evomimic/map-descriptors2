use crate::shared_test::fixture_defs::derive_type_name;
use descriptors::mutators::{
    new_boolean_descriptor, new_integer_descriptor, new_string_descriptor,
    update_boolean_descriptor, update_integer_descriptor, update_string_descriptor,
};
use descriptors::property_map_builder::{insert_property_descriptor, remove_property_descriptor};
// use hdk::prelude::*;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails, PropertyDescriptorMap,
};
use shared_types_descriptor::type_header::BaseType;
use std::collections::BTreeMap;

/// This function adds a set of PropertyDescriptors of various Scalar Types to supplied PropertyMap
///
pub fn create_example_property_descriptors(
    property_descriptor_map: &mut PropertyDescriptorMap,
) -> Result<PropertyDescriptorMap, DescriptorsError> {
    // Add Boolean Descriptor
    let descriptor = new_boolean_descriptor(
        derive_type_name("simple", BaseType::Boolean, ""),
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
        derive_type_name("simple_", BaseType::String, ""),
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
        derive_type_name("simple_I8", BaseType::Integer, ""),
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
        derive_type_name("simple_I16", BaseType::Integer, ""),
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
        derive_type_name("simple_I32", BaseType::Integer, ""),
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
        derive_type_name("simple_I64", BaseType::Integer, ""),
        "Simple Integer (I64) Property Type description".to_string(),
        true,
        IntegerFormat::I64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "an_I64_property".to_string(),
        &descriptor,
    );

    // Integer U8
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U8", BaseType::Integer, ""),
        "Simple Integer (U8) Property Type description".to_string(),
        true,
        IntegerFormat::U8(),
        0,
        127,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U8_property".to_string(),
        &descriptor,
    );

    // Integer U16
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U16", BaseType::Integer, ""),
        "Simple Integer (U16) Property Type description".to_string(),
        true,
        IntegerFormat::U16(),
        0,
        32767,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U16_property".to_string(),
        &descriptor,
    );

    // Integer U32
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U32", BaseType::Integer, ""),
        "Simple Integer (U32) Property Type description".to_string(),
        true,
        IntegerFormat::U32(),
        0,
        2147483648,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U32_property".to_string(),
        &descriptor,
    );

    // Integer U64
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U64", BaseType::Integer, ""),
        "Simple Integer (U64) Property Type description".to_string(),
        true,
        IntegerFormat::U64(),
        0,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "a_U64_property".to_string(),
        &descriptor,
    );

    Ok(property_descriptor_map.clone())
}

pub fn create_example_updates_for_property_descriptors(
    property_descriptor_map: &mut PropertyDescriptorMap,
) -> Result<PropertyDescriptorMap, DescriptorsError> {
    // Update Boolean Descriptor
    let property_name = "a_boolean_property".to_string();
    let mut expected_boolean_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(descriptor) = expected_boolean_descriptor {
        let updated_boolean_descriptor = update_boolean_descriptor(
            descriptor,
            Some("change is_fuzzy to true".to_string()),
            Some(true),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_boolean_property".to_string(),
            &updated_boolean_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update String Descriptor
    let property_name = "a_string_property".to_string();
    let mut expected_string_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(string_descriptor) = expected_string_descriptor {
        let updated_string_descriptor = update_string_descriptor(
            string_descriptor,
            Some("changed min".to_string()),
            Some(3),
            None,
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_string_property".to_string(),
            &updated_string_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I8
    let property_name = "an_I8_property".to_string();
    let mut expected_i8_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_i8_descriptor {
        let updated_i8_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min".to_string()),
            IntegerFormat::I8(),
            Some(0),
            None,
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "an_I8_property".to_string(),
            &updated_i8_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I16
    let property_name = "an_I16_property".to_string();
    let mut expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_integer_descriptor {
        let updated_i16_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change max".to_string()),
            IntegerFormat::I16(),
            None,
            Some(444444),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "an_I16_property".to_string(),
            &updated_i16_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I32
    let property_name = "an_I32_property".to_string();
    let mut expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_integer_descriptor {
        let updated_i32_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min max".to_string()),
            IntegerFormat::I32(),
            Some(-123456789),
            Some(987654321),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "an_I32_property".to_string(),
            &updated_i32_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I64
    let property_name = "an_I64_property".to_string();
    let mut expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_integer_descriptor {
        let updated_i64_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min max".to_string()),
            IntegerFormat::I64(),
            Some(-3.333333e9 as i64),
            Some(7.777777e14 as i64),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "an_I64_property".to_string(),
            &updated_i64_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U8
    let property_name = "a_U8_property".to_string();
    let mut expected_u8_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_u8_descriptor {
        let updated_u8_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min".to_string()),
            IntegerFormat::U8(),
            Some(1),
            None,
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_U8_property".to_string(),
            &updated_u8_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U16
    let property_name = "a_U16_property".to_string();
    let mut expected_16_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_16_descriptor {
        let updated_16_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change max".to_string()),
            IntegerFormat::U16(),
            None,
            Some(444),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_U16_property".to_string(),
            &updated_16_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U32
    let property_name = "a_U32_property".to_string();
    let mut expected_u32_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_u32_descriptor {
        let updated_u32_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min max".to_string()),
            IntegerFormat::U32(),
            Some(12345),
            Some(67329),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_U32_property".to_string(),
            &updated_u32_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U64
    let property_name = "a_U64_property".to_string();
    let mut expected_u64_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(integer_descriptor) = expected_u64_descriptor {
        let updated_u64_descriptor = update_integer_descriptor(
            integer_descriptor,
            Some("change min max".to_string()),
            IntegerFormat::U64(),
            Some(2.1618e9 as i64),
            Some(8.5555e12 as i64),
        )?;
        insert_property_descriptor(
            property_descriptor_map,
            "a_U64_property".to_string(),
            &updated_u64_descriptor,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    Ok(property_descriptor_map.clone())
}

pub fn remove_properties(
    property_descriptor_map: &mut PropertyDescriptorMap,
) -> Result<BTreeMap<String, PropertyDescriptor>, DescriptorsError> {
    remove_property_descriptor(property_descriptor_map, "a_string_property".to_string());

    // Add String Descriptor
    let descriptor = new_string_descriptor(
        derive_type_name("simple_", BaseType::String, "update"),
        "Simple Example Update to String Property Type description".to_string(),
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
        derive_type_name("simple_I8", BaseType::Integer, "update"),
        "Simple Example Update to Integer (I8) Property Type description".to_string(),
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
        derive_type_name("simple_I16", BaseType::Integer, "update"),
        "Simple Example Update to Integer (I16) Property Type description".to_string(),
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
        derive_type_name("simple_I32", BaseType::Integer, "update"),
        "Simple Example Udpdate to Integer (I32) Property Type description".to_string(),
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
        derive_type_name("simple_I64", BaseType::Integer, "update"),
        "Simple Integer (I64) Property Type description".to_string(),
        true,
        IntegerFormat::I64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        property_descriptor_map,
        "an_I64\
        _property"
            .to_string(),
        &descriptor,
    );

    // Integer U8
    let descriptor = new_integer_descriptor(
        derive_type_name("simple_U8", BaseType::Integer, "update"),
        "Simple Example Update to  Integer (U8) Property Type description".to_string(),
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
        derive_type_name("simple_U16", BaseType::Integer, "update"),
        "Simple Example Update to Integer (U16) Property Type description".to_string(),
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
        derive_type_name("simple_U32", BaseType::Integer, "update"),
        "Simple Example Update to Integer (U32) Property Type description".to_string(),
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
        derive_type_name("simple_U64", BaseType::Integer, "update"),
        "Simple Example Update to Integer (U64) Property Type description".to_string(),
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

    Ok(property_descriptor_map.properties.clone())
}
