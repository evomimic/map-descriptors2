mod conductor;
/*
use descriptors::helpers::{get_holon_descriptor_from_record, get_property_descriptor_from_record};
use descriptors::holon_descriptor_storage_fns::UpdateHolonDescriptorInput;
use descriptors::mutators::new_property_descriptor;
use descriptors::property_descriptor_storage_fns::UpdatePropertyDescriptorInput;
use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;
use shared_types_descriptor::type_header::BaseType;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_update_holon_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();
    // println!("{:#?}", descriptors);

    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            descriptors[0].clone(),
        )
        .await;

    // let _original_entry = get_holon_descriptor_from_record(created_record).unwrap();

    let original_action_hash: ActionHash = created_record.action_address().clone();

    let first_update_descriptor = descriptors[1].clone();

    let first_update_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: original_action_hash.clone(),
        previous_holon_descriptor_hash: original_action_hash.clone(),
        updated_holon_descriptor: first_update_descriptor.clone(),
    };

    let first_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            first_update_input,
        )
        .await;

    let updated_entry = get_holon_descriptor_from_record(first_updated_record.clone()).unwrap();

    let first_update_action_hash: ActionHash = first_updated_record.action_address().clone();

    let second_update_descriptor = descriptors[2].clone();

    let second_update_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: original_action_hash,
        previous_holon_descriptor_hash: first_update_action_hash.clone(),
        updated_holon_descriptor: second_update_descriptor.clone(),
    };

    let second_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            second_update_input,
        )
        .await;

    let next_updated_entry =
        get_holon_descriptor_from_record(second_updated_record.clone()).unwrap();

    let second_update_action_hash: ActionHash = second_updated_record.action_address().clone();

    assert_eq!(first_update_descriptor, updated_entry);
    assert_eq!(second_update_descriptor, next_updated_entry);

    // TESTING GETS

    let fetched_first_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            first_update_action_hash.clone(),
        )
        .await;

    let fetched_updated_entry =
        get_holon_descriptor_from_record(fetched_first_updated_record.clone().unwrap()).unwrap();

    let fetched_second_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            second_update_action_hash.clone(),
        )
        .await;

    let fetched_next_updated_entry =
        get_holon_descriptor_from_record(fetched_second_updated_record.clone().unwrap()).unwrap();

    assert!(fetched_first_updated_record.is_some());
    assert!(fetched_second_updated_record.is_some());
    // assert_eq!(first_updated_record, fetched_first_updated_record.unwrap());
    // assert_eq!(
    //     second_updated_record,
    //     fetched_second_updated_record.unwrap()
    // );
    assert_eq!(updated_entry, fetched_updated_entry);
    assert_eq!(next_updated_entry, fetched_next_updated_entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_update_property_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let descriptor1: PropertyDescriptor = new_property_descriptor(
        "ex_prop_desc".to_string(),
        "example description".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let descriptor2: PropertyDescriptor = new_property_descriptor(
        "first update".to_string(),
        "example updated descriptor".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let descriptor3: PropertyDescriptor = new_property_descriptor(
        "second update".to_string(),
        "another updated property descriptor".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            descriptor1.clone(),
        )
        .await;

    // let _original_entry = get_property_descriptor_from_record(created_record).unwrap();

    let original_action_hash: ActionHash = created_record.action_address().clone();

    let first_update_descriptor = descriptor2.clone();

    let first_update_input = UpdatePropertyDescriptorInput {
        original_property_descriptor_hash: original_action_hash.clone(),
        previous_property_descriptor_hash: original_action_hash.clone(),
        updated_property_descriptor: first_update_descriptor.clone(),
    };

    let first_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_property_descriptor",
            first_update_input,
        )
        .await;

    let updated_entry = get_property_descriptor_from_record(first_updated_record.clone()).unwrap();

    let first_update_action_hash: ActionHash = first_updated_record.action_address().clone();

    let second_update_descriptor = descriptor3.clone();

    let second_update_input = UpdatePropertyDescriptorInput {
        original_property_descriptor_hash: original_action_hash,
        previous_property_descriptor_hash: first_update_action_hash.clone(),
        updated_property_descriptor: second_update_descriptor.clone(),
    };

    let second_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_property_descriptor",
            second_update_input,
        )
        .await;

    let next_updated_entry =
        get_property_descriptor_from_record(second_updated_record.clone()).unwrap();

    let second_update_action_hash: ActionHash = second_updated_record.action_address().clone();

    assert_eq!(first_update_descriptor, updated_entry);
    assert_eq!(second_update_descriptor, next_updated_entry);

    // TESTING GETS

    let fetched_first_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            first_update_action_hash.clone(),
        )
        .await;

    let fetched_updated_entry =
        get_property_descriptor_from_record(fetched_first_updated_record.clone().unwrap()).unwrap();

    let fetched_second_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            second_update_action_hash.clone(),
        )
        .await;

    let fetched_next_updated_entry =
        get_property_descriptor_from_record(fetched_second_updated_record.clone().unwrap())
            .unwrap();

    assert!(fetched_first_updated_record.is_some());
    assert!(fetched_second_updated_record.is_some());
    // assert_eq!(first_updated_record, fetched_first_updated_record.unwrap());
    // assert_eq!(
    //     second_updated_record,
    //     fetched_second_updated_record.unwrap()
    // );
    assert_eq!(updated_entry, fetched_updated_entry);
    assert_eq!(next_updated_entry, fetched_next_updated_entry);
}
*/