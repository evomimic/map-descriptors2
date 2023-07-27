#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::BTreeMap;

use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    BooleanDescriptor, CompositeDescriptor, IntegerDescriptor, IntegerFormat, PropertyDescriptor,
    PropertyDescriptorDetails, PropertyDescriptorMap, StringDescriptor,
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
    let header = new_type_header(type_name.to_string(), base_type, description.to_string(), is_dependent)?;
    Ok(PropertyDescriptor::new(header))
    //Ok(PropertyDescriptor::new(header,details))
}
///
/// Creates a new (empty) Composite Property Descriptor
pub fn new_composite_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> Result<PropertyDescriptor, DescriptorsError> {
    let details = PropertyDescriptorDetails::Composite(CompositeDescriptor::new(
         PropertyDescriptorMap::new(Default::default()),
    ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_type_header() {
        let header_success = new_type_header(
            "example_name".to_string(),
            BaseType::String,
            "example_description".to_string(),
            true,
        )
        .unwrap();

        println!("{:#?}", header_success);

        assert_eq!(header_success.type_name, "example_name".to_string());

        assert_eq!(header_success.base_type, BaseType::String);

        assert!(header_success.is_dependent);

        assert_eq!(
            header_success.description,
            "example_description".to_string()
        );

        let header_error = new_type_header(
            "".to_string(),
            BaseType::String,
            "example_description".to_string(),
            true,
        )
        .expect_err("Empty field, should throw error");

        assert_eq!(
            header_error,
            DescriptorsError::EmptyField("type_name".to_string())
        );

        assert_eq!(
            header_error.to_string(),
            "type_name field is missing".to_string()
        );
    }
}
