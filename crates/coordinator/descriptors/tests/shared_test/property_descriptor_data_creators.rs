use descriptors::mutators::{
    new_boolean_descriptor, new_integer_descriptor, new_string_descriptor,
    update_boolean_descriptor, update_integer_descriptor, update_string_descriptor,
};
use descriptors::property_map_builder::upsert_property_descriptor;
// use hdk::prelude::*;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::property_descriptor::{DescriptorSharing, IntegerFormat, PropertyDescriptorMap, PropertyDescriptorUsage};
use shared_types_descriptor::type_header::BaseType;
use crate::shared_test::fixture_helpers::{derive_label, derive_type_description, derive_type_name};

/// This function adds a set of PropertyDescriptors of various Scalar Types to supplied PropertyMap
///
pub fn create_example_property_descriptors(
    property_descriptor_map: &mut PropertyDescriptorMap,
) -> Result<PropertyDescriptorMap, DescriptorsError> {
    // Add Boolean Descriptor
    let type_name = derive_type_name("simple", BaseType::Boolean, "");
    let boolean_descriptor = new_boolean_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        false,
    )?;
    let boolean_usage = PropertyDescriptorUsage::new(
        "example boolean usage description".to_string(),
        boolean_descriptor,
        "a boolean property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_boolean_property".to_string(),
        &boolean_usage,
    );

    // Add String Descriptor
    let type_name = derive_type_name("simple_", BaseType::String, "");
    let string_descriptor = new_string_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        0,
        2048,
    )?;
    let string_usage = PropertyDescriptorUsage::new(
        "example string usage description".to_string(),
        string_descriptor,
        "a string property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_string_property".to_string(),
        &string_usage,
    );

    // Add Integer I8
    let type_name = derive_type_name("simple_I8", BaseType::Integer, "");
    let i8_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::I8(),
        -127,
        127,
    )?;
    let i8_usage = PropertyDescriptorUsage::new(
        "example integer usage description".to_string(),
        i8_descriptor,
        "an i8 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "an_I8_property".to_string(),
        &i8_usage,
    );

    // Integer I16
    let type_name = derive_type_name("simple_I16", BaseType::Integer, "");
    let i16_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::I16(),
        -32767,
        32767,
    )?;
    let i16_usage = PropertyDescriptorUsage::new(
        "example integer usage description".to_string(),
        i16_descriptor,
        "am i16 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "an_I16_property".to_string(),
        &i16_usage,
    );

    // Integer I32
    let type_name = derive_type_name("simple_I32", BaseType::Integer, "");
    let i32_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::I32(),
        -2147483648,
        2147483648,
    )?;
    let i32_usage = PropertyDescriptorUsage::new(
        "example i32 integer property description".to_string(),
        i32_descriptor,
        "an i32 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "an_I32_property".to_string(),
        &i32_usage,
    );

    // Integer I64
    let type_name = derive_type_name("simple_I64", BaseType::Integer, "");
    let i64_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::I64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;
    let i64_usage = PropertyDescriptorUsage::new(
        "example i64 integer property description".to_string(),
        i64_descriptor,
        "an i64 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "an_I64_property".to_string(),
        &i64_usage,
    );

    // Integer U8
    let type_name = derive_type_name("simple_U8", BaseType::Integer, "");
    let u8_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::U8(),
        0,
        127,
    )?;
    let u8_usage = PropertyDescriptorUsage::new(
        "example u8 integer property description".to_string(),
        u8_descriptor,
        "a u8 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_U8_property".to_string(),
        &u8_usage,
    );

    // Integer U16
    let type_name = derive_type_name("simple_U16", BaseType::Integer, "");
    let u16_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::U16(),
        0,
        32767,
    )?;
    let u16_usage = PropertyDescriptorUsage::new(
        "example u16 integer property description".to_string(),
        u16_descriptor,
        "a u16 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_U16_property".to_string(),
        &u16_usage,
    );

    // Integer U32
    let type_name = derive_type_name("simple_U32", BaseType::Integer, "");
    let u32_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::U32(),
        0,
        2147483648,
    )?;
    let u32_usage = PropertyDescriptorUsage::new(
        "example u32 integer property description".to_string(),
        u32_descriptor,
        "a u32 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_U32_property".to_string(),
        &u32_usage,
    );

    // Integer U64
    let type_name = derive_type_name("simple_U64", BaseType::Integer, "");
    let u64_descriptor = new_integer_descriptor(
        type_name.clone(),
        derive_type_description(&type_name),
        derive_label(&type_name),
        true,
        IntegerFormat::U64(),
        0,
        9.223372036855e18 as i64,
    )?;
    let u64_usage = PropertyDescriptorUsage::new(
        "example u64 integer property description".to_string(),
        u64_descriptor,
        "a u64 integer property".to_string(),
        DescriptorSharing::default(),
    );
    upsert_property_descriptor(
        property_descriptor_map,
        "a_U64_property".to_string(),
        &u64_usage,
    );

    Ok(property_descriptor_map.clone())
}

pub fn create_example_updates_for_property_descriptors(
    property_descriptor_map: &mut PropertyDescriptorMap,
) -> Result<PropertyDescriptorMap, DescriptorsError> {
    // Update Boolean Descriptor
    let property_name = "a_boolean_property".to_string();
    let expected_boolean_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(boolean_usage) = expected_boolean_descriptor {
        let mut updated_boolean_usage = boolean_usage.clone();
        updated_boolean_usage.descriptor = update_boolean_descriptor(
            &boolean_usage.descriptor,
            Some("change is_fuzzy to true".to_string()),
            Some("a new label".to_string()),
            Some(true),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_boolean_property".to_string(),
            &updated_boolean_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update String Descriptor
    let property_name = "a_string_property".to_string();
    let expected_string_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(string_usage) = expected_string_descriptor {
        let mut updated_string_usage = string_usage.clone();
        updated_string_usage.descriptor = update_string_descriptor(
            &string_usage.descriptor,
            Some("changed min".to_string()),
            Some("a new label".to_string()),
            Some(3),
            None,
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_string_property".to_string(),
            &updated_string_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I8
    let property_name = "an_I8_property".to_string();
    let expected_i8_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(i8_usage) = expected_i8_descriptor {
        let mut updated_i8_usage = i8_usage.clone();
        updated_i8_usage.descriptor = update_integer_descriptor(
            &i8_usage.descriptor,
            Some("change min".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::I8(),
            Some(0),
            None,
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "an_I8_property".to_string(),
            &updated_i8_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I16
    let property_name = "an_I16_property".to_string();
    let expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(i16_usage) = expected_integer_descriptor {
        let mut updated_i16_usage = i16_usage.clone();
        updated_i16_usage.descriptor = update_integer_descriptor(
            &i16_usage.descriptor,
            Some("change max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::I16(),
            None,
            Some(444444),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "an_I16_property".to_string(),
            &updated_i16_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I32
    let property_name = "an_I32_property".to_string();
    let expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(i32_usage) = expected_integer_descriptor {
        let mut updated_i32_usage = i32_usage.clone();
        updated_i32_usage.descriptor = update_integer_descriptor(
            &i32_usage.descriptor,
            Some("change min max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::I32(),
            Some(-123456789),
            Some(987654321),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "an_I32_property".to_string(),
            &updated_i32_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer I64
    let property_name = "an_I64_property".to_string();
    let expected_integer_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(i64_usage) = expected_integer_descriptor {
        let mut updated_i64_usage = i64_usage.clone();
        updated_i64_usage.descriptor = update_integer_descriptor(
            &i64_usage.descriptor,
            Some("change min max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::I64(),
            Some(-3.333333e9 as i64),
            Some(7.777777e14 as i64),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "an_I64_property".to_string(),
            &updated_i64_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U8
    let property_name = "a_U8_property".to_string();
    let expected_u8_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(u8_usage) = expected_u8_descriptor {
        let mut updated_u8_usage = u8_usage.clone();
        updated_u8_usage.descriptor = update_integer_descriptor(
            &u8_usage.descriptor,
            Some("change min".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::U8(),
            Some(1),
            None,
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_U8_property".to_string(),
            &updated_u8_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U16
    let property_name = "a_U16_property".to_string();
    let expected_u16_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(u16_usage) = expected_u16_descriptor {
        let mut updated_u16_usage = u16_usage.clone();
        updated_u16_usage.descriptor = update_integer_descriptor(
            &u16_usage.descriptor,
            Some("change max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::U16(),
            None,
            Some(444),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_U16_property".to_string(),
            &updated_u16_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U32
    let property_name = "a_U32_property".to_string();
    let expected_u32_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(u32_usage) = expected_u32_descriptor {
        let mut updated_u32_usage = u32_usage.clone();
        updated_u32_usage.descriptor = update_integer_descriptor(
            &u32_usage.descriptor,
            Some("change min max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::U32(),
            Some(12345),
            Some(67329),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_U32_property".to_string(),
            &updated_u32_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    // Update Integer U64
    let property_name = "a_U64_property".to_string();
    let expected_u64_descriptor = property_descriptor_map.properties.get(&property_name);
    if let Some(u64_usage) = expected_u64_descriptor {
        let mut updated_u64_usage = u64_usage.clone();
        updated_u64_usage.descriptor = update_integer_descriptor(
            &u64_usage.descriptor,
            Some("change min max".to_string()),
            Some("a new label".to_string()),
            IntegerFormat::U64(),
            Some(2.1618e9 as i64),
            Some(8.5555e12 as i64),
        )?;
        upsert_property_descriptor(
            property_descriptor_map,
            "a_U64_property".to_string(),
            &updated_u64_usage,
        );
    } else {
        panic!("Expected {:?}, not found", property_name);
    }

    Ok(property_descriptor_map.clone())
}
