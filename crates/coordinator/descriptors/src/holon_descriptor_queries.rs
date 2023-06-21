// use std::collections::BTreeMap;

use crate::stub_data_creator::*;

use descriptors_integrity::LinkTypes;
use hdk::prelude::*;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;

#[hdk_extern]
pub fn get_all_holontypes(_: ()) -> ExternResult<Vec<HolonDescriptor>> {
    let dummy_data_result = create_dummy_data(());
    match dummy_data_result {
        Ok(data) => return Ok(data),
        Err(error) => return Err(wasm_error!(WasmErrorInner::Guest(error.to_string()))),
    }
}
/*
#[hdk_extern]
pub fn get_entry_by_action_hash(action_hash: ActionHash) -> ExternResult<Entry> {
    get_entry_by_action(action_hash)
}
*/

// the following was generated by scaffolding tool, notice use of Record in return type

#[hdk_extern]
pub fn get_all_holon_types(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_holon_types");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllHolonTypes, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
