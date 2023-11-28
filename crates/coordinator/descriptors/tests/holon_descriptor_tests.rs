//! Holon Descriptor Test Cases

#![allow(unused_imports)]

// use futures::future;
use std::collections::BTreeMap;

mod shared_test;

// use tracing::{debug, info, instrument, Level};
use log::{debug, info};
use test_log::test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
//use std::arch::x86_64::__cpuid_count;
use async_std::task;
use descriptors::helpers::get_holon_descriptor_from_record;
use descriptors::holon_descriptor_storage_fns::UpdateHolonDescriptorInput;
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_integer_descriptor, new_string_descriptor,
};
use descriptors::property_map_builder::{remove_property_descriptor, upsert_property_descriptor};
use rstest::*;
// use shared_test::data_fixtures::{create_dummy_data,derive_type_name,insert_property_descriptor};
use shared_test::fixture_helpers::derive_type_name;
use shared_test::holon_descriptor_fixtures::*;
use shared_test::setup_conductor;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::value_descriptor::{
    CompositeDescriptor, PropertyDescriptorMap, ValueDescriptor, ValueDescriptorDetails,
};

/// This function exercises a broad range of capabilities. The heavy lifting for this test is in the
/// test data set creation done within fixtures.
///
/// Test Outline:
/// 1. After initial setup, perform a `get_all_holon_types`, with an expectation of an empty result
/// 2. For each member of the `descriptors` vector, perform a `create` followed by a `get` and verify
/// 3. Once all data has been created in DHT, perform `get_all_holon_types` and verify the result.
///
/// Note that this will exercise, create, get, and get_all capabilities across a variety of
/// holon descriptors
///
/// To selectively run JUST THE TESTS in this file, use:
///      cargo test -p descriptors --test holon_descriptor_tests  -- --show-output
///
#[rstest]
#[case::mixture_of_holon_types(new_holons_fixture())]
#[self::test(tokio::test(flavor = "multi_thread"))]
#[instrument]
async fn rstest_holon_descriptor_capabilities(
    #[case] input: Result<Vec<HolonDescriptor>, DescriptorsError>, //HolonTestCase
) {
    // Setup

    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    // The heavy lifting for this test is in the test data set creation. Rich descriptors can be
    // built in the create_dummy_data fn to test a broad range of data structures

    let mut descriptors: Vec<HolonDescriptor> = input.unwrap();
    descriptors.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    let h_count = descriptors.len();

    info!("******* STARTING TESTS WITH {h_count} HOLON DESCRIPTORS ***************************");

    info!("Performing get_all_holon_types to ensure initial DB state is empty");
    let result: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await;
    assert_eq!(0, result.len());
    info!("Success! Initial DB state has no HolonDescriptors");

    let mut created_action_hashes: Vec<ActionHash> = Vec::new();

    // Iterate through the vector of generated holon descriptors, creating each descriptor,
    // then get the created descriptor and comparing it to the generated descriptor.
    for descriptor in descriptors.clone() {
        let name = descriptor.header.type_name.clone();
        let p_count = descriptor.property_map.properties.len();
        println!();
        info!("****** Starting create/get test for the following HolonDescriptor");
        debug!("{:#?}", descriptor);
        debug!("Creating {name} with {p_count} properties");

        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_holon_descriptor",
                descriptor.clone(),
            )
            .await;
        /*
        let created_record: Record = match result {
            Ok(record)=>record,
            Err(error)=>panic!("Problem executing Conductor call: {:?}", error),
        };
        */
        let created_descriptor = get_holon_descriptor_from_record(created_record.clone()).unwrap();
        assert_eq!(descriptor, created_descriptor);

        info!("Created descriptor matches generated holon descriptor, fetching created descriptor");

        let action_hash: ActionHash = created_record.action_address().clone();
        created_action_hashes.push(action_hash.clone());

        let fetched_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_holon_descriptor",
                action_hash,
            )
            .await;

        let fetched_descriptor = get_holon_descriptor_from_record(fetched_record.unwrap()).unwrap();
        assert_eq!(descriptor, fetched_descriptor);
        info!("...Success! Fetched descriptor matches generated descriptor ******");
    }

    info!("All Holon Descriptors Created... do a get_all_holon_types and compare result with test data...");
    let fetch_all: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await;
    let d_count = fetch_all.len();
    debug!("Call to get_all_holon_types returned {d_count} Holon Descriptors \n");
    assert_eq!(d_count, h_count);
    let mut fetched_entries = Vec::new();
    for fetched_record in fetch_all {
        let fetched_descriptor = get_holon_descriptor_from_record(fetched_record)
            .clone()
            .unwrap();
        fetched_entries.push(fetched_descriptor);
    }
    fetched_entries.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    assert_eq!(descriptors, fetched_entries);

    // TESTING DELETES //
    info!("\n\n *********** TESTING DELETES *******************\n");

    for hash in created_action_hashes {
        let _action_hash_of_delete: ActionHash = conductor
            .call(
                &cell.zome("descriptors"),
                "delete_holon_descriptor",
                hash.clone(),
            )
            .await;
        let try_query: Option<Record> = conductor
            .call(&cell.zome("descriptors"), "get_holon_descriptor", hash)
            .await;
        assert!(try_query.is_none());
    }
    let fetch_all_check_deleted: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await;
    assert!(fetch_all_check_deleted.is_empty());
    info!("...get_all_holon_types confirms, all holon descriptors have been deleted.\n");
}
