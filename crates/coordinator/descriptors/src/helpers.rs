use hdk::prelude::*;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;

// pub fn try_entry_from_record(record: Record) -> ExternResult<Entry> {
//     record
//         .entry()
//         .as_option()
//         .ok_or(wasm_error!(WasmErrorInner::Guest(
//             "Entry missing".to_string()
//         )))
//         .cloned()
// }

pub fn get_descriptor_from_record(record: Record) -> ExternResult<HolonDescriptor> {
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
