use hdk::prelude::*;
use descriptors_integrity::*;
use shared_types_descriptor::property_descriptor::ValueDescriptor;

#[hdk_extern]
pub fn create_property_descriptor(
    property_descriptor: ValueDescriptor,
) -> ExternResult<Record> {
    let property_descriptor_hash = create_entry(
        &EntryTypes::PropertyDescriptor(property_descriptor.clone()),
    )?;
    let record = get(property_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created PropertyDescriptor"))
            ),
        )?;
    let path = Path::from("all_property_descriptors");
    create_link(
        path.path_entry_hash()?,
        property_descriptor_hash.clone(),
        LinkTypes::AllPropertyDescriptors,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_property_descriptor(
    original_property_descriptor_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_property_descriptor_hash.clone(),
        LinkTypes::PropertyDescriptorUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_property_descriptor_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_property_descriptor_hash.clone(),
    };
    get(latest_property_descriptor_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertyDescriptorInput {
    pub original_property_descriptor_hash: ActionHash,
    pub previous_property_descriptor_hash: ActionHash,
    pub updated_property_descriptor: ValueDescriptor,
}

#[hdk_extern]
pub fn update_property_descriptor(
    input: UpdatePropertyDescriptorInput,
) -> ExternResult<Record> {
    let updated_property_descriptor_hash = update_entry(
        input.previous_property_descriptor_hash.clone(),
        &input.updated_property_descriptor,
    )?;
    create_link(
        input.original_property_descriptor_hash.clone(),
        updated_property_descriptor_hash.clone(),
        LinkTypes::PropertyDescriptorUpdates,
        (),
    )?;
    let record = get(updated_property_descriptor_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated PropertyDescriptor"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_property_descriptor(
    original_property_descriptor_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_property_descriptor_hash)
}
