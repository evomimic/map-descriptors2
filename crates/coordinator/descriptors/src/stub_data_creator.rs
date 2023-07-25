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

use crate::mutators::{
    new_boolean_descriptor, new_holon_descriptor, new_integer_descriptor, new_string_descriptor,
};
use crate::property_map_builder::insert_property_descriptor;
// use hdk::prelude::*;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::IntegerFormat;
use shared_types_descriptor::type_header::BaseType;


fn derive_type_name(prefix: &str, base_type: BaseType, suffix: &str)-> String {
    let base_type_string = base_type.to_string()+"_Type";
    let result = if prefix.is_empty() {
        if suffix.is_empty() {
            base_type_string
        } else {
            format!("{base_type_string}_{suffix}")
        }

    } else { // prefix is NOT empty
        if suffix.is_empty() {
            format!("{prefix}_{base_type_string}")
        } else {
            format!("{prefix}_{base_type_string}_{suffix}")

        }
    };
    result.to_string()
}



fn derive_type_description(type_name: String)-> String {
    format!("{type_name},_description")
}
/// This function creates a rich test dataset by creating a vector of HolonDescriptors of various
/// kinds -- from simple to complex
/// 
pub fn create_dummy_data(_: ()) -> Result<Vec<HolonDescriptor>, DescriptorsError> {

    let mut test_data_set: Vec<HolonDescriptor> = Vec::new();


    // ----------------  HOLON WITH NO PROPERTIES-------------------------------
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("",BaseType::Holon,"_with_no_properties"),
        "A simple holon type that has no properties.".to_string(),
        false,
    )?;
    test_data_set.push(descriptor);

    // ----------------  HOLON WITH SINGLE BOOLEAN PROPERTY -------------------------------

    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("",BaseType::Holon,"_with_single_boolean_property"),
        "A simple holon type that has a single boolean property".to_string(),
        false,
    )?;

    let bool_descriptor = new_boolean_descriptor(
        derive_type_name("simple",BaseType::Boolean,""),
        "Simple Boolean Property Type description".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "a_boolean_property".to_string(),
        bool_descriptor,
    );
    test_data_set.push(descriptor);

    // ----------------  HOLON WITH SCALAR PROPERTIES -------------------------------
    let mut descriptor: HolonDescriptor = new_holon_descriptor(
        derive_type_name("",BaseType::Holon,"_with_scalar_properties"),
        "A holon type that has a single property of each scalar property type.".to_string(),
        false,
    )?;

    let bool_descriptor = new_boolean_descriptor(
        derive_type_name("simple",BaseType::Boolean,""),
        "Simple Boolean Property Type description".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "a_boolean_property".to_string(),
        bool_descriptor,
    );

    // String
    let string_descriptor = new_string_descriptor(
        derive_type_name("simple_",BaseType::String,""),
        "Simple String Property Type description".to_string(),
        true,
        0,
        2048,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "a_string_property".to_string(),
        string_descriptor,
    );
    // Integer I8
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_I8",BaseType::Integer,""),
        "Simple Integer (I8) Property Type description".to_string(),
        true,
        IntegerFormat::I8(),
        -127,
        127,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_I8_property".to_string(),
        int_descriptor,
    );
    // Integer I16
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_I16",BaseType::Integer,""),
        "Simple Integer (I16) Property Type description".to_string(),
        true,
        IntegerFormat::I16(),
        -32767,
        32767,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_I16_property".to_string(),
        int_descriptor,
    );

    // Integer I32
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_I32",BaseType::Integer,""),
        "Simple Integer (I32) Property Type description".to_string(),
        true,
        IntegerFormat::I32(),
        -2147483648,
        2147483648,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_I32_property".to_string(),
        int_descriptor,
    );

    // Integer I64
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_I64",BaseType::Integer,""),
        "Simple Integer (I64) Property Type description".to_string(),
        true,
        IntegerFormat::I64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_I64\
        _property".to_string(),
        int_descriptor,
    );
    
    // Integer U8
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_U8",BaseType::Integer,""),
        "Simple Integer (U8) Property Type description".to_string(),
        true,
        IntegerFormat::U8(),
        -127,
        127,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_U8_property".to_string(),
        int_descriptor,
    );
    // Integer U16
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_U16",BaseType::Integer,""),
        "Simple Integer (U16) Property Type description".to_string(),
        true,
        IntegerFormat::U16(),
        -32767,
        32767,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_U16_property".to_string(),
        int_descriptor,
    );

    // Integer U32
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_U32",BaseType::Integer,""),
        "Simple Integer (U32) Property Type description".to_string(),
        true,
        IntegerFormat::U32(),
        -2147483648,
        2147483648,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_U32_property".to_string(),
        int_descriptor,
    );

    // Integer U64
    let int_descriptor = new_integer_descriptor(
        derive_type_name("simple_U64",BaseType::Integer,""),
        "Simple Integer (U64) Property Type description".to_string(),
        true,
        IntegerFormat::U64(),
        -9.223372036855e18 as i64,
        9.223372036855e18 as i64,
    )?;

    insert_property_descriptor(
        &mut descriptor.properties,
        "an_U64_property".to_string(),
        int_descriptor,
    );


    test_data_set.push(descriptor);




/*
    let int_descriptor1 = new_integer_descriptor(
        "an_integer_property_type".to_string(),
        "desc for integer_property_type descriptor1, ".to_string(),
        true,
        IntegerFormat::I32(),
        -10,
        100,
    )?;

    let string_descriptor1 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        1,
        10,
    )?;



    insert_property_descriptor(
        &mut descriptor1.properties,
        "prop1a_int".to_string(),
        int_descriptor1,
    );

    insert_property_descriptor(
        &mut descriptor1.properties,
        "prop2a_string".to_string(),
        string_descriptor1,
    );



    let mut descriptor2: HolonDescriptor = new_holon_descriptor(
        "holon_type_name2".to_string(),
        "holon_type_description".to_string(),
        false,
    )?;

    let int_descriptor2 = new_integer_descriptor(
        "an_integer_property_type".to_string(),
        "desc for integer_property_type descriptor2, ".to_string(),
        true,
        IntegerFormat::I16(),
        -20,
        200,
    )?;

    let string_descriptor2 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        2,
        20,
    )?;

    let bool_descriptor2 = new_boolean_descriptor(
        "a_boolean_property_type".to_string(),
        "desc for boolean_property_type descriptor1, ".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop1b_int".to_string(),
        int_descriptor2,
    );

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop2b_string".to_string(),
        string_descriptor2,
    );

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop3b_bool".to_string(),
        bool_descriptor2,
    );

    let mut descriptor3: HolonDescriptor = new_holon_descriptor(
        "holon_type_name3".to_string(),
        "holon_type_description3".to_string(),
        false,
    )?;

    let int_descriptor3 = new_integer_descriptor(
        "an_integer_property_type".to_string(),
        "desc for integer_property_type descriptor3, ".to_string(),
        true,
        IntegerFormat::I32(),
        -30,
        300,
    )?;

    let string_descriptor3 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        3,
        30,
    )?;

    let bool_descriptor3 = new_boolean_descriptor(
        "a_boolean_property_type".to_string(),
        "desc for boolean_property_type descriptor1, ".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop1c_int".to_string(),
        int_descriptor3,
    );

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop2c_string".to_string(),
        string_descriptor3,
    );

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop3c_bool".to_string(),
        bool_descriptor3,
    );
*/
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
