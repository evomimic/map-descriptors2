//! Value Descriptor Test Cases

// #![allow(unused_imports)]
// #![allow(unused_doc_comments)]

mod shared_test;

// use core::panic;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::get_value_descriptor_from_record;
use rstest::*;
use shared_test::value_descriptor_fixtures::new_dedicated_value_descriptors_fixture;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::value_descriptor::ValueDescriptor;
use tracing::{info, trace};

/// This function exercises a broad range of capabilities. The heavy lifting for this test is in the
/// test data set creation done within fixtures.
///
/// Test Outline:
/// 1. After initial setup, perform a `get_all_value_types`, with an expectation of an empty result
/// 2. For each member of the `descriptors` vector, perform a `create` followed by a `get` and verify
/// 3. Once all data has been created in DHT, perform `get_all_value_types` and verify the result.
///
/// Note that this will exercise, create, get, and get_all capabilities across a variety of
/// holon descriptors
///
/// To selectively run JUST THE TESTS in this file, use:
///      cargo test -p descriptors --test value_descriptor_tests  -- --show-output
///
///
#[rstest]
#[case::mixture_of_dedicated_value_types(new_dedicated_value_descriptors_fixture())]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_value_descriptor_capabilities(
    #[case] input: Result<Vec<ValueDescriptor>, DescriptorsError>,
) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    info!("******* STARTING TESTS FOR VALUE DESCRIPTORS *************************** \n");

    let mut descriptors = input.unwrap();
    descriptors.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    let d_count = descriptors.len();
    assert_eq!(d_count, 4);

    info!("Performing get_all_value_descriptors to ensure initial DB state is empty");
    let result: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_value_descriptors", ())
        .await;
    assert_eq!(0, result.len());
    info!("Success! Initial DB state has no ValueDescriptors \n");

    let mut created_action_hashes: Vec<ActionHash> = Vec::new();

    for descriptor in descriptors.clone() {
        info!("Starting create/get test for the following ValueDescriptor");
        trace!("{:#?}", descriptor);

        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_value_descriptor",
                descriptor.clone(),
            )
            .await;

        let created_descriptor = get_value_descriptor_from_record(created_record.clone()).unwrap();
        assert_eq!(descriptor, created_descriptor);

        info!(
            "Created descriptor matches generated property descriptor, fetching created descriptor.."
        );

        let action_hash: ActionHash = created_record.action_address().clone();
        created_action_hashes.push(action_hash.clone());

        let fetched_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_value_descriptor",
                action_hash,
            )
            .await;

        let fetched_descriptor = get_value_descriptor_from_record(fetched_record.unwrap()).unwrap();
        assert_eq!(descriptor, fetched_descriptor);
        info!("...Success! Fetched descriptor matches generated descriptor. \n");
    }

    info!("All Value Descriptors Created... do a get_all_value_descriptors and compare result with test data...");
    let fetch_all: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_value_descriptors", ())
        .await;
    let fetch_count = fetch_all.len();
    info!("Call to get_all_value_descriptors returned {fetch_count} Value Descriptors");
    assert_eq!(d_count, fetch_count);
    let mut fetched_entries = Vec::new();
    for fetched_record in fetch_all {
        let fetched_descriptor = get_value_descriptor_from_record(fetched_record)
            .clone()
            .unwrap();
        fetched_entries.push(fetched_descriptor);
    }
    fetched_entries.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    assert_eq!(descriptors, fetched_entries);
}
