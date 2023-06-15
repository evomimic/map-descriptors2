pub mod holon_descriptor_validators;
pub mod property_descriptor_validators;

use shared_types_descriptor::holon_descriptor::{HolonDescriptor};
use shared_types_descriptor::property_descriptor::{PropertyDescriptor};
use crate::holon_descriptor_validators::{
    validate_create_link_holon_descriptor_updates,
    validate_update_holon_descriptor,
    validate_delete_holon_descriptor,
    validate_create_link_all_holon_types,
    validate_delete_link_all_holon_types,
    validate_delete_link_holon_descriptor_updates,
    validate_create_holon_descriptor,
};

use hdi::prelude::*;
use crate::property_descriptor_validators::{
    validate_create_link_property_descriptor_updates,
    validate_create_link_all_property_types,
    validate_delete_link_all_property_types,
    validate_create_property_descriptor,
    validate_delete_link_property_descriptor_updates,
    validate_delete_property_descriptor,
    validate_update_property_descriptor};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    HolonDescriptor(HolonDescriptor),
    PropertyDescriptor(PropertyDescriptor),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    HolonDescriptorUpdates,
    AllHolonTypes,
    PropertyDescriptorUpdates,
    AllPropertyDescriptors,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::HolonDescriptor(holon_descriptor) => {
                            validate_create_holon_descriptor(
                                EntryCreationAction::Create(action),
                                holon_descriptor,
                            )
                        }
                        EntryTypes::PropertyDescriptor(property_descriptor) => {
                            validate_create_property_descriptor(
                                EntryCreationAction::Create(action),
                                property_descriptor,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::HolonDescriptor(holon_descriptor) => {
                            validate_create_holon_descriptor(
                                EntryCreationAction::Update(action),
                                holon_descriptor,
                            )
                        }
                        EntryTypes::PropertyDescriptor(property_descriptor) => {
                            validate_create_property_descriptor(
                                EntryCreationAction::Update(action),
                                property_descriptor,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::PropertyDescriptor(property_descriptor),
                            EntryTypes::PropertyDescriptor(original_property_descriptor),
                        ) => {
                            validate_update_property_descriptor(
                                action,
                                property_descriptor,
                                original_action,
                                original_property_descriptor,
                            )
                        }
                        (
                            EntryTypes::HolonDescriptor(holon_descriptor),
                            EntryTypes::HolonDescriptor(original_holon_descriptor),
                        ) => {
                            validate_update_holon_descriptor(
                                action,
                                holon_descriptor,
                                original_action,
                                original_holon_descriptor,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::HolonDescriptor(holon_descriptor) => {
                            validate_delete_holon_descriptor(
                                action,
                                original_action,
                                holon_descriptor,
                            )
                        }
                        EntryTypes::PropertyDescriptor(property_descriptor) => {
                            validate_delete_property_descriptor(
                                action,
                                original_action,
                                property_descriptor,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::HolonDescriptorUpdates => {
                    validate_create_link_holon_descriptor_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHolonTypes => {
                    validate_create_link_all_holon_types(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PropertyDescriptorUpdates => {
                    validate_create_link_property_descriptor_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPropertyDescriptors => {
                    validate_create_link_all_property_types(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::HolonDescriptorUpdates => {
                    validate_delete_link_holon_descriptor_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHolonTypes => {
                    validate_delete_link_all_holon_types(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PropertyDescriptorUpdates => {
                    validate_delete_link_property_descriptor_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPropertyDescriptors => {
                    validate_delete_link_all_property_types(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::HolonDescriptor(holon_descriptor) => {
                            validate_create_holon_descriptor(
                                EntryCreationAction::Create(action),
                                holon_descriptor,
                            )
                        }
                        EntryTypes::PropertyDescriptor(property_descriptor) => {
                            validate_create_property_descriptor(
                                EntryCreationAction::Create(action),
                                property_descriptor,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::HolonDescriptor(holon_descriptor) => {
                            let result = validate_create_holon_descriptor(
                                EntryCreationAction::Update(action.clone()),
                                holon_descriptor.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_holon_descriptor: Option<HolonDescriptor> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_holon_descriptor = match original_holon_descriptor {
                                    Some(holon_descriptor) => holon_descriptor,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_holon_descriptor(
                                    action,
                                    holon_descriptor,
                                    original_action,
                                    original_holon_descriptor,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::PropertyDescriptor(property_descriptor) => {
                            let result = validate_create_property_descriptor(
                                EntryCreationAction::Update(action.clone()),
                                property_descriptor.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_property_descriptor: Option<
                                    PropertyDescriptor,
                                > = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_property_descriptor = match original_property_descriptor {
                                    Some(property_descriptor) => property_descriptor,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_property_descriptor(
                                    action,
                                    property_descriptor,
                                    original_action,
                                    original_property_descriptor,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::HolonDescriptor(original_holon_descriptor) => {
                            validate_delete_holon_descriptor(
                                action,
                                original_action,
                                original_holon_descriptor,
                            )
                        }
                        EntryTypes::PropertyDescriptor(original_property_descriptor) => {
                            validate_delete_property_descriptor(
                                action,
                                original_action,
                                original_property_descriptor,
                            )
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::HolonDescriptorUpdates => {
                            validate_create_link_holon_descriptor_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllHolonTypes => {
                            validate_create_link_all_holon_types(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::PropertyDescriptorUpdates => {
                            validate_create_link_property_descriptor_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllPropertyDescriptors => {
                            validate_create_link_all_property_types(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::HolonDescriptorUpdates => {
                            validate_delete_link_holon_descriptor_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllHolonTypes => {
                            validate_delete_link_all_holon_types(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::PropertyDescriptorUpdates => {
                            validate_delete_link_property_descriptor_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllPropertyDescriptors => {
                            validate_delete_link_all_property_types(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
