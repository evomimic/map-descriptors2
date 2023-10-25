//! Property Descriptor Updates Test Cases

mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::get_value_descriptor_from_record;
use descriptors::value_descriptor_storage_fns::UpdateValueDescriptorInput;
use rstest::*;
use shared_test::value_descriptor_fixtures::*;
use shared_test::test_data_types::ValueDescriptorTestCase;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::value_descriptor::ValueDescriptor;

/// These tests exercise update actions on ValueDescriptors
/// To execute ONLY the tests in this file, use:
///      cargo test -p descriptors --test value_descriptor_updates_tests -- --show-output
#[rstest]
#[case::update_composite_descriptor(update_value_descriptor_composite())]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_value_descriptor_updates(
    #[case] input: Result<ValueDescriptorTestCase, DescriptorsError>,
) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    let input_values = input.unwrap();
    let original_descriptor: ValueDescriptor = input_values.original;
    let expected_descriptors: Vec<ValueDescriptor> = input_values.updates;
    println!(
        "******* STARTING TEST CASES FOR UPDATING VALUE DESCRIPTOR *************************** \n"
    );
    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_value_descriptor",
            original_descriptor.clone(),
        )
        .await;
    let created_descriptor = get_value_descriptor_from_record(created_record.clone()).unwrap();
    assert_eq!(original_descriptor, created_descriptor);

    let created_action_hash: ActionHash = created_record.action_address().clone();
    let previous_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_value_descriptor",
            created_action_hash.clone(),
        )
        .await;
    let mut previous_record = previous_record.unwrap();
    let fetched_descriptor = get_value_descriptor_from_record(previous_record.clone()).unwrap();
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
    original_value_descriptor_record: &Record,
    expected_value_descriptor: &ValueDescriptor,
) -> Record {
    let original_action_hash: ActionHash =
        original_value_descriptor_record.action_address().clone();

    let original_descriptor =
        get_value_descriptor_from_record(original_value_descriptor_record.clone()).unwrap();

    println!("original: {:#?} \n", original_descriptor);
    println!("expected: {:#?} \n", expected_value_descriptor);

    let update_input = UpdateValueDescriptorInput {
        original_value_descriptor_hash: created_action_hash.clone(),
        previous_value_descriptor_hash: original_action_hash.clone(),
        updated_value_descriptor: expected_value_descriptor.clone(),
    };
    let updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_value_descriptor",
            update_input,
        )
        .await;
    let updated_descriptor = get_value_descriptor_from_record(updated_record.clone()).unwrap();
    let updated_action_hash: ActionHash = updated_record.action_address().clone();
    assert_eq!(*expected_value_descriptor, updated_descriptor);
    let fetched_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_value_descriptor",
            updated_action_hash.clone(),
        )
        .await;
    let fetched_updated_descriptor =
        get_value_descriptor_from_record(fetched_updated_record.clone().unwrap()).unwrap();
    assert_eq!(updated_descriptor, fetched_updated_descriptor);

    fetched_updated_record.unwrap()
}
