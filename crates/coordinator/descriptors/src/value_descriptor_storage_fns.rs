use hdk::prelude::*;
use descriptors_integrity::*;
use shared_types_descriptor::value_descriptor::ValueDescriptor;

#[hdk_extern]
pub fn create_value_descriptor(
    value_descriptor: ValueDescriptor,
) -> ExternResult<Record> {
    let value_descriptor_hash = create_entry(
        &EntryTypes::ValueDescriptor(value_descriptor.clone()),
    )?;
    let record = get(value_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created ValueDescriptor"))
            ),
        )?;
    let path = Path::from("all_value_descriptors");
    create_link(
        path.path_entry_hash()?,
        value_descriptor_hash.clone(),
        LinkTypes::AllValueDescriptors,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_value_descriptor(
    original_value_descriptor_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_value_descriptor_hash.clone(),
        LinkTypes::ValueDescriptorUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_value_descriptor_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_value_descriptor_hash.clone(),
    };
    get(latest_value_descriptor_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateValueDescriptorInput {
    pub original_value_descriptor_hash: ActionHash,
    pub previous_value_descriptor_hash: ActionHash,
    pub updated_value_descriptor: ValueDescriptor,
}

#[hdk_extern]
pub fn update_value_descriptor(
    input: UpdateValueDescriptorInput,
) -> ExternResult<Record> {
    let updated_value_descriptor_hash = update_entry(
        input.previous_value_descriptor_hash.clone(),
        &input.updated_value_descriptor,
    )?;
    create_link(
        input.original_value_descriptor_hash.clone(),
        updated_value_descriptor_hash.clone(),
        LinkTypes::ValueDescriptorUpdates,
        (),
    )?;
    let record = get(updated_value_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated ValueDescriptor"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_value_descriptor(
    original_value_descriptor_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_value_descriptor_hash)
}
