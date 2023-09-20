#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::BTreeMap;

// use async_std::stream::StreamExt;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    BooleanDescriptor, CompositeDescriptor, DescriptorSharing, IntegerDescriptor, IntegerFormat,
    PropertyDescriptor, PropertyDescriptorDetails, PropertyDescriptorMap, StringDescriptor,
};
use shared_types_descriptor::type_header::{BaseType, SemanticVersion, TypeHeader};

/// new_xxx_descriptor () functions stage new (empty) instances of Descriptors, but do NOT
/// commit them to persistent storage.
///
// new_type_header is private helper function used by the other public create functions in this
// module. It is not intended to be called externally.
fn new_type_header(
    type_name: String,
    base_type: BaseType,
    description: String,
    is_dependent: bool,
) -> Result<TypeHeader, DescriptorsError> {
    let header = TypeHeader::new(
        type_name,
        base_type,
        description,
        SemanticVersion::default(),
        is_dependent,
    );
    example_header_check(header.clone())?; // could be validation

    Ok(header)
}

fn example_header_check(header: TypeHeader) -> Result<TypeHeader, DescriptorsError> {
    if header.type_name.is_empty() {
        return Err(DescriptorsError::EmptyField("type_name".to_string()));
    }
    Ok(header)
}

/// Creates an empty holon descriptor.
///
pub fn new_holon_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> Result<HolonDescriptor, DescriptorsError> {
    let header = new_type_header(type_name, BaseType::Holon, description, is_dependent)?;

    let descriptor = HolonDescriptor::new(header, PropertyDescriptorMap::new(BTreeMap::new()));

    Ok(descriptor)
}

// new_property_descriptor is a private helper function used by the other type-specific public
// create functions in this module. It is not intended to be called externally.
fn new_property_descriptor(
    type_name: String,
    description: String,
    base_type: BaseType,
    is_dependent: bool,
    details: PropertyDescriptorDetails,
) -> Result<PropertyDescriptor, DescriptorsError> {
    // Guard that base_type in header matches details
    let header = new_type_header(
        type_name.to_string(),
        base_type,
        description.to_string(),
        is_dependent,
    )?;
    Ok(PropertyDescriptor::new(
        header,
        // Default is Dedicated
        //DescriptorSharing::default(), // NOTE: will need to change this in the future to accomodate shared or make a seperate function
        details,
    ))
}

///
/// Creates a new (empty) Composite Property Descriptor
pub fn new_composite_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    properties: PropertyDescriptorMap,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let details = PropertyDescriptorDetails::Composite(CompositeDescriptor::new(properties));

    let desc = new_property_descriptor(
        type_name,
        description,
        BaseType::Composite,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_string_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    min_length: u32,
    max_length: u32,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let details = PropertyDescriptorDetails::String(StringDescriptor::new(min_length, max_length));
    let desc = new_property_descriptor(
        type_name,
        description,
        BaseType::String,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_integer_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    format: IntegerFormat,
    min_value: i64,
    max_value: i64,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let details =
        PropertyDescriptorDetails::Integer(IntegerDescriptor::new(format, min_value, max_value));
    let desc = new_property_descriptor(
        type_name,
        description,
        BaseType::Integer,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_boolean_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    is_fuzzy: bool,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let details = PropertyDescriptorDetails::Boolean(BooleanDescriptor::new(is_fuzzy));
    let desc = new_property_descriptor(
        type_name,
        description,
        BaseType::Boolean,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn update_boolean_descriptor(
    original_descriptor: &PropertyDescriptor,
    new_description: Option<String>,
    is_fuzzy: Option<bool>,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    let mut bool_descriptor = BooleanDescriptor::default();
    match original_descriptor.details.clone() {
        PropertyDescriptorDetails::Boolean(descriptor) => {
            if let Some(fuzz) = is_fuzzy {
                bool_descriptor.is_fuzzy = fuzz
            } else {
                bool_descriptor.is_fuzzy = descriptor.is_fuzzy
            }
        }
        _ => panic!("Expected BooleanDescriptor"),
    }

    updated_descriptor.details = PropertyDescriptorDetails::Boolean(bool_descriptor);
    println!(
        "testing update boolean_descriptor: {:#?}",
        updated_descriptor
    );

    Ok(updated_descriptor)
}

pub fn update_string_descriptor(
    original_descriptor: &PropertyDescriptor,
    new_description: Option<String>,
    min_length: Option<u32>,
    max_length: Option<u32>,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    let mut string_descriptor = StringDescriptor::default();
    match original_descriptor.details.clone() {
        PropertyDescriptorDetails::String(descriptor) => {
            if let Some(min) = min_length {
                string_descriptor.min_length = min
            } else {
                string_descriptor.min_length = descriptor.min_length
            }
            if let Some(max) = max_length {
                string_descriptor.max_length = max
            } else {
                string_descriptor.max_length = descriptor.max_length
            }
        }
        _ => panic!("Expected StringDescriptor"),
    }

    updated_descriptor.details = PropertyDescriptorDetails::String(string_descriptor);
    println!(
        "testing update string_descriptor: {:#?}",
        updated_descriptor
    );

    Ok(updated_descriptor)
}

pub fn update_integer_descriptor(
    original_descriptor: &PropertyDescriptor,
    new_description: Option<String>,
    format: IntegerFormat,
    min_value: Option<i64>,
    max_value: Option<i64>,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    let mut integer_descriptor = IntegerDescriptor::new(format, 0, 0);
    match original_descriptor.details.clone() {
        PropertyDescriptorDetails::Integer(descriptor) => {
            if let Some(min) = min_value {
                integer_descriptor.min_value = min
            } else {
                integer_descriptor.min_value = descriptor.min_value
            }
            if let Some(max) = max_value {
                integer_descriptor.max_value = max
            } else {
                integer_descriptor.max_value = descriptor.max_value
            }
        }
        _ => panic!("Expected StringDescriptor"),
    }

    updated_descriptor.details = PropertyDescriptorDetails::Integer(integer_descriptor);
    // println!(
    //     "testing update string_descriptor: {:#?}",
    //     updated_descriptor
    // );

    Ok(updated_descriptor)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new_type_header() {
//         let header_success = new_type_header(
//             "example_name".to_string(),
//             BaseType::String,
//             "example_description".to_string(),
//             true,
//         )
//         .unwrap();

//         // println!("{:#?}", header_success);

//         assert_eq!(header_success.type_name, "example_name".to_string());

//         assert_eq!(header_success.base_type, BaseType::String);

//         assert!(header_success.is_dependent);

//         assert_eq!(
//             header_success.description,
//             "example_description".to_string()
//         );

//         let header_error = new_type_header(
//             "".to_string(),
//             BaseType::String,
//             "example_description".to_string(),
//             true,
//         )
//         .expect_err("Empty field, should throw error");

//         assert_eq!(
//             header_error,
//             DescriptorsError::EmptyField("type_name".to_string())
//         );

//         assert_eq!(
//             header_error.to_string(),
//             "type_name field is missing".to_string()
//         );
//     }
// }
