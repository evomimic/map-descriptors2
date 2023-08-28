//! Property Descriptor Test Cases

// #![allow(unused_imports)]
// #![allow(unused_doc_comments)]

mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::get_property_descriptor_from_record;
use rstest::*;
use shared_test::fixture_defs::new_property_descriptors_fixture;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;

#[rstest]
#[case::mixture_of_property_types(new_property_descriptors_fixture())]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_property_descriptor_capabilities(
    #[case] input: Result<Vec<PropertyDescriptor>, DescriptorsError>,
) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    println!("******* STARTING TESTS FOR PROPERTY DESCRIPTORS *************************** \n");

    let mut descriptors = input.unwrap();
    descriptors.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    let d_count = descriptors.len();
    assert_eq!(d_count, 4);

    println!("Performing get_all_property_descriptors to ensure initial DB state is empty");
    let result: Vec<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_all_property_descriptors",
            (),
        )
        .await;
    assert_eq!(0, result.len());
    println!("Success! Initial DB state has no PropertyDescriptors \n");

    let mut created_action_hashes: Vec<ActionHash> = Vec::new();

    for descriptor in descriptors.clone() {
        println!("Starting create/get test for the following PropertyDescriptor");
        println!("{:#?}", descriptor);

        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                descriptor.clone(),
            )
            .await;

        let created_descriptor =
            get_property_descriptor_from_record(created_record.clone()).unwrap();
        assert_eq!(descriptor, created_descriptor);

        println!(
            "Created descriptor matches generated property descriptor, fetching created descriptor.."
        );

        let action_hash: ActionHash = created_record.action_address().clone();
        created_action_hashes.push(action_hash.clone());

        let fetched_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_property_descriptor",
                action_hash,
            )
            .await;

        let fetched_descriptor =
            get_property_descriptor_from_record(fetched_record.unwrap()).unwrap();
        assert_eq!(descriptor, fetched_descriptor);
        println!("...Success! Fetched descriptor matches generated descriptor. \n");
    }

    println!("All Property Descriptors Created... do a get_all_property_descriptors and compare result with test data...");
    let fetch_all: Vec<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_all_property_descriptors",
            (),
        )
        .await;
    let fetch_count = fetch_all.len();
    println!("Call to get_all_property_descriptors returned {fetch_count} Property Descriptors");
    assert_eq!(d_count, fetch_count);
    let mut fetched_entries = Vec::new();
    for fetched_record in fetch_all {
        let fetched_descriptor = get_property_descriptor_from_record(fetched_record)
            .clone()
            .unwrap();
        fetched_entries.push(fetched_descriptor);
    }
    fetched_entries.sort_by(|a, b| a.header.type_name.cmp(&b.header.type_name));
    assert_eq!(descriptors, fetched_entries);
}
