
use hdk::prelude::*;

use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{BooleanDescriptor, CompositeDescriptor, IntegerDescriptor, IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails, PropertyDescriptorMap, StringDescriptor};
use shared_types_descriptor::type_header::{BaseType, SemanticVersion, TypeHeader};


/// new_xxx_descriptor () functions stage new (empty) instances of Descriptors, but do NOT
/// commit them to persistent storage

pub fn new_type_header(
    type_name: String,
    base_type: BaseType,
    description: String,
    is_dependent: bool,
) -> ExternResult<TypeHeader> {
    let header = TypeHeader::new(
        type_name,
        base_type,
        description,
        SemanticVersion::default(),
        is_dependent,
    );
    Ok(header)
}


pub fn new_holon_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<HolonDescriptor> {
    let header = new_type_header(type_name, BaseType::Holon, description, is_dependent)?;

    let descriptor = HolonDescriptor::new(header, PropertyDescriptorMap::new(Default::default()));

    Ok(descriptor)
}

fn new_property_descriptor(
    type_name: String,
    description: String,
    base_type: BaseType,
    is_dependent: bool,
    details: PropertyDescriptorDetails,
) -> ExternResult<PropertyDescriptor> {
    // Guard that base_type in header matches details
    let header = new_type_header(type_name, base_type, description, is_dependent)?;
    Ok(PropertyDescriptor::new(header, details))
}

pub fn new_composite_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<PropertyDescriptor> {
    let details = PropertyDescriptorDetails::Composite(CompositeDescriptor::new(PropertyDescriptorMap::new(Default::default())));
    let desc = new_property_descriptor(type_name,description,BaseType::Composite,is_dependent, details)?;
    Ok(desc)
}

pub fn new_string_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    min_length: u32,
    max_length: u32,
) -> ExternResult<PropertyDescriptor> {
    let details = PropertyDescriptorDetails::String(StringDescriptor::new(min_length, max_length,));
    let desc = new_property_descriptor(type_name,description,BaseType::String,is_dependent, details)?;
    Ok(desc)
}

pub fn new_integer_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    format: IntegerFormat,
    min_value: i128,
    max_value: i128,
) -> ExternResult<PropertyDescriptor> {
    let details = PropertyDescriptorDetails::Integer(IntegerDescriptor::new(format, min_value, max_value,));
    let desc = new_property_descriptor(type_name,description,BaseType::Integer,is_dependent, details)?;
    Ok(desc)
}
pub fn new_boolean_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    is_fuzzy: bool,
) -> ExternResult<PropertyDescriptor> {
    let details = PropertyDescriptorDetails::Boolean(BooleanDescriptor::new(is_fuzzy,));
    let desc = new_property_descriptor(type_name,description,BaseType::Boolean,is_dependent, details)?;
    Ok(desc)
}
