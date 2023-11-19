#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::BTreeMap;

// use async_std::stream::StreamExt;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::type_header::{BaseType, SemanticVersion, TypeHeader};
use shared_types_descriptor::value_descriptor::{
    BooleanDescriptor, CompositeDescriptor, DescriptorSharing, IntegerDescriptor,
    PropertyDescriptorMap, StringDescriptor, ValueDescriptor, ValueDescriptorDetails,
};

/// new_xxx_descriptor () functions stage new (empty) instances of Descriptors, but do NOT
/// commit them to persistent storage.
///
// new_type_header is private helper function used by the other public create functions in this
// module. It is not intended to be called externally.
fn new_type_header(
    type_name: String,
    base_type: BaseType,
    description: String,
    label: String,
    is_dependent: bool,
) -> Result<TypeHeader, DescriptorsError> {
    let header = TypeHeader::new(
        type_name,
        base_type,
        description,
        label,
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
    label: String,
    is_dependent: bool,
) -> Result<HolonDescriptor, DescriptorsError> {
    let header = new_type_header(type_name, BaseType::Holon, description, label, is_dependent)?;

    let descriptor = HolonDescriptor::new(header, PropertyDescriptorMap::new(BTreeMap::new()));

    Ok(descriptor)
}

// new_property_descriptor is a private helper function used by the other type-specific public
// create functions in this module. It is not intended to be called externally.
fn new_property_descriptor(
    type_name: String,
    description: String,
    label: String,
    base_type: BaseType,
    is_dependent: bool,
    details: ValueDescriptorDetails,
) -> Result<ValueDescriptor, DescriptorsError> {
    // Guard that base_type in header matches details
    let header = new_type_header(
        type_name.to_string(),
        base_type,
        description.to_string(),
        label,
        is_dependent,
    )?;
    Ok(ValueDescriptor::new(
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
    label: String,
    is_dependent: bool,
    properties: PropertyDescriptorMap,
) -> Result<ValueDescriptor, DescriptorsError> {
    let details = ValueDescriptorDetails::Composite(CompositeDescriptor::new(properties));

    let desc = new_property_descriptor(
        type_name,
        description,
        label,
        BaseType::Composite,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_string_descriptor(
    type_name: String,
    description: String,
    label: String,
    is_dependent: bool,
    min_length: u32,
    max_length: u32,
) -> Result<ValueDescriptor, DescriptorsError> {
    let details = ValueDescriptorDetails::String(StringDescriptor::new(min_length, max_length));
    let desc = new_property_descriptor(
        type_name,
        description,
        label,
        BaseType::String,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_integer_descriptor(
    type_name: String,
    description: String,
    label: String,
    is_dependent: bool,
    min_value: i64,
    max_value: i64,
) -> Result<ValueDescriptor, DescriptorsError> {
    let details = ValueDescriptorDetails::Integer(IntegerDescriptor::new(min_value, max_value));
    let desc = new_property_descriptor(
        type_name,
        description,
        label,
        BaseType::Integer,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn new_boolean_descriptor(
    type_name: String,
    description: String,
    label: String,
    is_dependent: bool,
    is_fuzzy: bool,
) -> Result<ValueDescriptor, DescriptorsError> {
    let details = ValueDescriptorDetails::Boolean(BooleanDescriptor::new(is_fuzzy));
    let desc = new_property_descriptor(
        type_name,
        description,
        label,
        BaseType::Boolean,
        is_dependent,
        details,
    )?;
    Ok(desc)
}

pub fn update_boolean_descriptor(
    original_descriptor: &ValueDescriptor,
    new_description: Option<String>,
    new_label: Option<String>,
    is_fuzzy: Option<bool>,
) -> Result<ValueDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    if let Some(label) = new_label {
        updated_descriptor.header.label = label;
    }

    let mut bool_descriptor = BooleanDescriptor::default();
    match original_descriptor.details.clone() {
        ValueDescriptorDetails::Boolean(descriptor) => {
            if let Some(fuzz) = is_fuzzy {
                bool_descriptor.is_fuzzy = fuzz
            } else {
                bool_descriptor.is_fuzzy = descriptor.is_fuzzy
            }
        }
        _ => panic!("Expected BooleanDescriptor"),
    }

    updated_descriptor.details = ValueDescriptorDetails::Boolean(bool_descriptor);

    Ok(updated_descriptor)
}

pub fn update_string_descriptor(
    original_descriptor: &ValueDescriptor,
    new_description: Option<String>,
    new_label: Option<String>,
    min_length: Option<u32>,
    max_length: Option<u32>,
) -> Result<ValueDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    if let Some(label) = new_label {
        updated_descriptor.header.label = label;
    }
    let mut string_descriptor = StringDescriptor::default();
    match original_descriptor.details.clone() {
        ValueDescriptorDetails::String(descriptor) => {
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

    updated_descriptor.details = ValueDescriptorDetails::String(string_descriptor);

    Ok(updated_descriptor)
}

pub fn update_integer_descriptor(
    original_descriptor: &ValueDescriptor,
    new_description: Option<String>,
    new_label: Option<String>,
    min_value: Option<i64>,
    max_value: Option<i64>,
) -> Result<ValueDescriptor, DescriptorsError> {
    let mut updated_descriptor = original_descriptor.clone();
    if let Some(description) = new_description {
        updated_descriptor.header.description = description;
    }
    if let Some(label) = new_label {
        updated_descriptor.header.label = label;
    }
    let mut integer_descriptor = IntegerDescriptor::new(0, 0);
    match original_descriptor.details.clone() {
        ValueDescriptorDetails::Integer(descriptor) => {
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

    updated_descriptor.details = ValueDescriptorDetails::Integer(integer_descriptor);

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
