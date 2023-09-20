use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;


#[derive(Clone, Debug)]
pub struct HolonDescriptorTestCase {
    pub original: HolonDescriptor,
    pub updates: Vec<HolonDescriptor>,
}

#[derive(Clone, Debug)]
pub struct PropertyDescriptorTestCase {
    pub original: PropertyDescriptor,
    pub updates: Vec<PropertyDescriptor>,
}

pub struct SharedTypesTestCase {
    pub shared_types: Vec<PropertyDescriptor>,
    pub referencing_types: Vec<PropertyDescriptor>, // composite descriptors, each of which references one or more of the shared_types
}
