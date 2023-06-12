use hdk::prelude::*;
use descriptors_integrity::*;
#[hdk_extern]
pub fn create_type_header(type_header: TypeHeader) -> ExternResult<Record> {
    let type_header_hash = create_entry(&EntryTypes::TypeHeader(type_header.clone()))?;
    let record = get(type_header_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created TypeHeader"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_type_header(type_header_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(type_header_hash, GetOptions::default())
}
