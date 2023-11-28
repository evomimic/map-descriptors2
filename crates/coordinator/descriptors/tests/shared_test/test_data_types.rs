use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::value_descriptor::ValueDescriptor;
// use tracing::Level;

#[derive(Clone, Debug)]
pub struct HolonDescriptorTestCase {
    pub original: HolonDescriptor,
    pub updates: Vec<HolonDescriptor>,
    // pub message_level: Level,
}

#[derive(Clone, Debug)]
pub struct ValueDescriptorTestCase {
    pub original: ValueDescriptor,
    pub updates: Vec<ValueDescriptor>,
    // pub message_level: Level,
}

#[derive(Clone, Debug)]
pub struct SharedTypesTestCase {
    pub shared_types: Vec<ValueDescriptor>,
    pub referencing_types: Vec<ValueDescriptor>, // composite descriptors, each of which references one or more of the shared_types
                                                 // pub message_level: Level,
}
