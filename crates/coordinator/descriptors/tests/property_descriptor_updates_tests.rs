//! Property Descriptor Updates Test Cases

mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::get_property_descriptor_from_record;
use descriptors::property_descriptor_storage_fns::UpdatePropertyDescriptorInput;
use rstest::*;
use shared_test::property_descriptor_fixtures::*;
use shared_test::test_data_types::PropertyDescriptorTestCase;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::property_descriptor::ValueDescriptor;

/// These tests exercise update actions on PropertyDescriptors
/// To execute ONLY the tests in this file, use:
///      cargo test -p descriptors --test property_descriptor_updates_tests -- --show-output
#[rstest]
#[case::update_composite_descriptor(update_property_descriptor_composite())]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_property_descriptor_updates(
    #[case] input: Result<PropertyDescriptorTestCase, DescriptorsError>,
) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    let input_values = input.unwrap();
    let original_descriptor: ValueDescriptor = input_values.original;
    let expected_descriptors: Vec<ValueDescriptor> = input_values.updates;
    println!(
        "******* STARTING TEST CASES FOR UPDATING PROPERTY DESCRIPTOR *************************** \n"
    );
    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            original_descriptor.clone(),
        )
        .await;
    let created_descriptor = get_property_descriptor_from_record(created_record.clone()).unwrap();
    assert_eq!(original_descriptor, created_descriptor);

    let created_action_hash: ActionHash = created_record.action_address().clone();
    let previous_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            created_action_hash.clone(),
        )
        .await;
    let mut previous_record = previous_record.unwrap();
    let fetched_descriptor = get_property_descriptor_from_record(previous_record.clone()).unwrap();
    assert_eq!(original_descriptor, created_descriptor);
    assert_eq!(created_descriptor, fetched_descriptor);

    for descriptor in expected_descriptors {
        previous_record = rstest_1_property_descriptor_update(
            &conductor,
            &cell,
            created_action_hash.clone(),
            &previous_record,
            &descriptor,
        )
            .await;
    }
}

pub async fn rstest_1_property_descriptor_update(
    conductor: &SweetConductor,
    cell: &SweetCell,
    created_action_hash: ActionHash,
    original_property_descriptor_record: &Record,
    expected_property_descriptor: &ValueDescriptor,
) -> Record {
    let original_action_hash: ActionHash =
        original_property_descriptor_record.action_address().clone();

    let original_descriptor =
        get_property_descriptor_from_record(original_property_descriptor_record.clone()).unwrap();

    println!("original: {:#?} \n", original_descriptor);
    println!("expected: {:#?} \n", expected_property_descriptor);

    let update_input = UpdatePropertyDescriptorInput {
        original_property_descriptor_hash: created_action_hash.clone(),
        previous_property_descriptor_hash: original_action_hash.clone(),
        updated_property_descriptor: expected_property_descriptor.clone(),
    };
    let updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_property_descriptor",
            update_input,
        )
        .await;
    let updated_descriptor = get_property_descriptor_from_record(updated_record.clone()).unwrap();
    let updated_action_hash: ActionHash = updated_record.action_address().clone();
    assert_eq!(*expected_property_descriptor, updated_descriptor);
    let fetched_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            updated_action_hash.clone(),
        )
        .await;
    let fetched_updated_descriptor =
        get_property_descriptor_from_record(fetched_updated_record.clone().unwrap()).unwrap();
    assert_eq!(updated_descriptor, fetched_updated_descriptor);

    fetched_updated_record.unwrap()
}
