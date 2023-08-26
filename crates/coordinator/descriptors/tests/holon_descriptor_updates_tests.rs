//! Holon Descriptor Updates Test Cases

#![allow(unused_imports)]

use core::panic;
use std::collections::BTreeMap;

mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::get_holon_descriptor_from_record;
use descriptors::holon_descriptor_storage_fns::UpdateHolonDescriptorInput;
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_integer_descriptor, new_string_descriptor,
};
use descriptors::property_map_builder::{insert_property_descriptor, remove_property_descriptor};
use rstest::*;
use shared_test::fixture_defs::{
    add_properties, add_properties_to_composite, derive_type_name, remove_properties,
    remove_properties_from_composite, update_each_scalar_details,
};
use shared_test::HolonDescriptorTestCase;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails,
    PropertyDescriptorMap,
};


/// These tests exercise update actions on HolonDescriptors
/// To execute ONLY the tests in this file, use:
///      cargo test -p descriptors --test holon_descriptor_updates_tests -- --show-output
#[rstest]
#[case::add_string_property_to_holon_descriptor(add_properties())]
#[case::remove_properties_from_holon_descriptor(remove_properties())]
#[case::update_each_scalar_property_details(update_each_scalar_details())]
#[case::add_property_to_composite_descriptor(add_properties_to_composite())]
#[case::remove_property_to_composite_descriptor(remove_properties_from_composite(
add_properties_to_composite()
))]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_holon_descriptor_updates(
    #[case] input: Result<HolonDescriptorTestCase, DescriptorsError>,
) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    let input_values = input.unwrap();
    let original_descriptor: HolonDescriptor = input_values.original;
    let expected_descriptors: Vec<HolonDescriptor> = input_values.updates;
    println!(
        "******* STARTING TEST CASES FOR UPDATING HOLON DESCRIPTOR *************************** \n"
    );
    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            original_descriptor.clone(),
        )
        .await;
    let created_descriptor = get_holon_descriptor_from_record(created_record.clone()).unwrap();
    assert_eq!(original_descriptor, created_descriptor);

    let created_action_hash: ActionHash = created_record.action_address().clone();
    let previous_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            created_action_hash.clone(),
        )
        .await;
    let mut previous_record = previous_record.unwrap();
    let fetched_descriptor = get_holon_descriptor_from_record(previous_record.clone()).unwrap();
    assert_eq!(original_descriptor, created_descriptor);
    assert_eq!(created_descriptor, fetched_descriptor);

    for descriptor in expected_descriptors {
        previous_record = rstest_1_holon_descriptor_update(
            &conductor,
            &cell,
            created_action_hash.clone(),
            &previous_record,
            &descriptor,
        )
            .await;
    }
}

pub async fn rstest_1_holon_descriptor_update(
    conductor: &SweetConductor,
    cell: &SweetCell,
    created_action_hash: ActionHash,
    original_holon_descriptor_record: &Record,
    expected_holon_descriptor: &HolonDescriptor,
) -> Record {
    let original_action_hash: ActionHash =
        original_holon_descriptor_record.action_address().clone();

    let original_descriptor =
        get_holon_descriptor_from_record(original_holon_descriptor_record.clone()).unwrap();

    // println!("original: {:#?} \n", original_descriptor);
    // println!("expected: {:#?} \n", expected_holon_descriptor);

    let update_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hash.clone(),
        previous_holon_descriptor_hash: original_action_hash.clone(),
        updated_holon_descriptor: expected_holon_descriptor.clone(),
    };
    let updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            update_input,
        )
        .await;
    let updated_descriptor = get_holon_descriptor_from_record(updated_record.clone()).unwrap();
    let updated_action_hash: ActionHash = updated_record.action_address().clone();
    assert_eq!(*expected_holon_descriptor, updated_descriptor);
    let fetched_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            updated_action_hash.clone(),
        )
        .await;
    let fetched_updated_descriptor =
        get_holon_descriptor_from_record(fetched_updated_record.clone().unwrap()).unwrap();
    assert_eq!(updated_descriptor, fetched_updated_descriptor);

    fetched_updated_record.unwrap()
}
