use hdk::prelude::*;
use shared_types_descriptor::holon_descriptor::{HolonDescriptor, HolonReference};
use shared_types_descriptor::value_descriptor::{
    CompositeDescriptor, DescriptorSharing, ValueDescriptor, ValueDescriptorDetails,
    PropertyDescriptorMap,
};

// pub fn try_entry_from_record(record: Record) -> ExternResult<Entry> {
//     record
//         .entry()
//         .as_option()
//         .ok_or(wasm_error!(WasmErrorInner::Guest(
//             "Entry missing".to_string()
//         )))
//         .cloned()
// }

// ?TODO: change input reference
pub fn get_holon_descriptor_from_record(record: Record) -> ExternResult<HolonDescriptor> {
    match record.entry() {
        RecordEntry::Present(entry) => {
            HolonDescriptor::try_from(entry.clone()).or(Err(wasm_error!(
                "Couldn't convert Record entry {:?} into data type {}",
                entry,
                std::any::type_name::<HolonDescriptor>()
            )))
        }
        _ => Err(wasm_error!("Record {:?} does not have an entry", record)),
    }
}
/*
pub fn get_holon_descriptor_from_record(record: Record) -> Result<HolonDescriptor,WasmError> {
    match record.entry() {
        record::RecordEntry::Present(entry) => {
            HolonDescriptor::try_from(entry.clone()).or(Err(wasm_error!(
                "Couldn't convert Record entry {:?} into data type {}",
                entry,
                std::any::type_name::<HolonDescriptor>()
            )))
        }
        _ => Err(wasm_error!("Record {:?} does not have an entry", record)),
    }
}
*/

pub fn get_value_descriptor_from_record(record: Record) -> ExternResult<ValueDescriptor> {
    match record.entry() {
        RecordEntry::Present(entry) => {
            ValueDescriptor::try_from(entry.clone()).or(Err(wasm_error!(
                "Couldn't convert Record entry {:?} into data type {}",
                entry,
                std::any::type_name::<ValueDescriptor>()
            )))
        }
        _ => Err(wasm_error!("Record {:?} does not have an entry", record)),
    }
}

// TEST HELPERS

// assumes map exists
pub fn get_composite_descriptor_map(details: &ValueDescriptorDetails) -> PropertyDescriptorMap {
    match details {
        ValueDescriptorDetails::Composite(map) => map.property_map.clone(),
        _ => panic!("error matching composite details"), // ?TODO: change this
    }
}

// assumes details are composite
pub fn get_composite_descriptor_from_details(
    details: &ValueDescriptorDetails,
) -> CompositeDescriptor {
    match details {
        ValueDescriptorDetails::Composite(map) => map.clone(),
        _ => panic!("error matching composite details"), // ?TODO: change this
    }
}

// assumes shared
pub fn get_holon_reference_from_sharing(sharing: &DescriptorSharing) -> HolonReference {
    match sharing {
        DescriptorSharing::Shared(holon_reference) => holon_reference.clone(),
        _ => panic!("error matching shared holon reference"), // ?TODO: change this
    }
}
