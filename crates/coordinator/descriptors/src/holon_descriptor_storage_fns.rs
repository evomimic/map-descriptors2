use hdk::prelude::*;
use descriptors_integrity::*;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;

#[hdk_extern]
pub fn create_holon_descriptor(
    holon_descriptor: HolonDescriptor,
) -> ExternResult<Record> {
    let holon_descriptor_hash = create_entry(
        &EntryTypes::HolonDescriptor(holon_descriptor.clone()),
    )?;
    let record = get(holon_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created HolonDescriptor"))
            ),
        )?;
    let path = Path::from("all_holon_types");
    create_link(
        path.path_entry_hash()?,
        holon_descriptor_hash.clone(),
        LinkTypes::AllHolonTypes,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_holon_descriptor(
    original_holon_descriptor_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_holon_descriptor_hash.clone(),
        LinkTypes::HolonDescriptorUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_holon_descriptor_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_holon_descriptor_hash.clone(),
    };
    get(latest_holon_descriptor_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHolonDescriptorInput {
    pub original_holon_descriptor_hash: ActionHash,
    pub previous_holon_descriptor_hash: ActionHash,
    pub updated_holon_descriptor: HolonDescriptor,
}
#[hdk_extern]
pub fn update_holon_descriptor(
    input: UpdateHolonDescriptorInput,
) -> ExternResult<Record> {
    let updated_holon_descriptor_hash = update_entry(
        input.previous_holon_descriptor_hash.clone(),
        &input.updated_holon_descriptor,
    )?;
    create_link(
        input.original_holon_descriptor_hash.clone(),
        updated_holon_descriptor_hash.clone(),
        LinkTypes::HolonDescriptorUpdates,
        (),
    )?;
    let record = get(updated_holon_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated HolonDescriptor"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_holon_descriptor(
    original_holon_descriptor_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_holon_descriptor_hash)
}
