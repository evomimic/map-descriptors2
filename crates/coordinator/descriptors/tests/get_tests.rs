
/*
mod conductor;
use descriptors::helpers::{get_holon_descriptor_from_record, get_property_descriptor_from_record};
use descriptors::mutators::new_property_descriptor;
use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;
use shared_types_descriptor::type_header::BaseType;

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_all_holontypes() {
//     let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
//         setup_conductor().await;

//     let testing_descriptors: Vec<HolonDescriptor> = conductor
//         .call(&cell.zome("descriptors"), "get_all_holontypes", ())
//         .await;

//     println!("{:#?}", testing_descriptors);
// }

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_holon_descriptor() {
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

    let entry = get_holon_descriptor_from_record(created_record.clone()).unwrap();

    assert_eq!(descriptors[0], entry);

    let action_hash: ActionHash = created_record.action_address().clone();

    let fetched_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            action_hash,
        )
        .await;
    // println!("{:#?}", fetched_record);

    let fetched_entry = get_holon_descriptor_from_record(fetched_record.unwrap()).unwrap();
    // println!("{:#?}", entry);

    assert_eq!(descriptors[0], fetched_entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_holon_types() {
    // Setup
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;


    let mock_descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();

    let mut action_hashes = Vec::new();

    for descriptor in mock_descriptors.clone() {
        let record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_holon_descriptor",
                descriptor,
            )
            .await;

        let action_hash = record.action_address().clone();
        action_hashes.push(action_hash);
    }

    let mut fetched_entries = Vec::new();

    let fetched_holon_types: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await; // may need to sort

    for record in fetched_holon_types {
        let entry: HolonDescriptor = get_holon_descriptor_from_record(record).unwrap();

        fetched_entries.push(entry);
    }
    // println!("{:#?}", mock_descriptors);
    // println!("{:#?}", fetched_entries);

    assert_eq!(mock_descriptors, fetched_entries);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_property_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let descriptor: PropertyDescriptor = new_property_descriptor(
        "ex_prop_desc".to_string(),
        "example description".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            descriptor.clone(),
        )
        .await;

    let entry = get_property_descriptor_from_record(record.clone()).unwrap();

    assert_eq!(descriptor, entry);

    let action_hash: ActionHash = record.action_address().clone();

    let fetched_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            action_hash,
        )
        .await;

    assert!(fetched_record.is_some());

    let fetched_entry = get_property_descriptor_from_record(fetched_record.unwrap()).unwrap();

    assert_eq!(descriptor, fetched_entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_property_types() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let mut mock_descriptors: Vec<PropertyDescriptor> = Vec::new();

    let string_descriptor = new_property_descriptor(
        "ex_string_name".to_string(),
        "example string descriptor".to_string(),
        BaseType::String,
        true,
    )
    .unwrap();
    mock_descriptors.push(string_descriptor);

    let integer_descriptor = new_property_descriptor(
        "ex_integer_name".to_string(),
        "example integer descriptor".to_string(),
        BaseType::Integer,
        true,
    )
    .unwrap();
    mock_descriptors.push(integer_descriptor);

    let boolean_descriptor = new_property_descriptor(
        "ex_boolean_name".to_string(),
        "example boolean descriptor".to_string(),
        BaseType::Boolean,
        true,
    )
    .unwrap();
    mock_descriptors.push(boolean_descriptor);

    let composite_descriptor = new_property_descriptor(
        "ex_composite_name".to_string(),
        "example composite descriptor".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();
    mock_descriptors.push(composite_descriptor);

    let mut action_hashes = Vec::new();

    for descriptor in mock_descriptors.clone() {
        let record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                descriptor,
            )
            .await;

        let action_hash = record.action_address().clone();
        action_hashes.push(action_hash);
    }

    let mut fetched_entries = Vec::new();

    let fetched_property_descriptors: Vec<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_all_property_descriptors",
            (),
        )
        .await; // may need to sort

    for record in fetched_property_descriptors {
        let entry: PropertyDescriptor = get_property_descriptor_from_record(record).unwrap();

        fetched_entries.push(entry);
    }
    // println!("{:#?}", mock_descriptors);
    // println!("{:#?}", fetched_entries);

    assert_eq!(mock_descriptors, fetched_entries);
}
*/