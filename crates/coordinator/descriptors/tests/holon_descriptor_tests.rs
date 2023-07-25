//! Holon Descriptor Test Cases

// use futures::future;
// use std::collections::BTreeMap;
mod conductor;

use descriptors::helpers::{get_holon_descriptor_from_record, get_property_descriptor_from_record};

use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;
use shared_types_descriptor::type_header::BaseType;
/// This function exercises a broad range of capabilities. The heavy lifting for this test is in the
/// test data set creation done by the `create_dummy_data` function. Each member of the vector of
/// Holon Descriptors can vary greatly, starting with simpler structures.
///
/// Test Outline:
/// 1. After initial setup, perform a `get_all_holon_types`, with an expectation of an empty result
/// 2. For each member of the `descriptors` vector, perform a `create` followed by a `get` and verify
/// 3. Once all data has been created in DHT, perform `get_all_holon_types` and verify the result.
///
/// Note that this will exercise, create, get, and get_all capabilities across a variety of
/// holon descriptors
///

#[tokio::test(flavor = "multi_thread")]
pub async fn test_holon_descriptor_capabilities() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    // The heavy lifting for this test is in the test data set creation
    // Richer descriptors can be built in the create_dummy_data fn to test a
    // broader range of data structures

    println!("******************* STARTING HOLON DESCRIPTOR TESTS *******************************");

    /* TODO: Fix this so it correctly handles an empty result from get_all_holon_types
    let descriptors: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "get_all_holon_types",
            (),
        )
        .await;
    // TODO Add assertion for the (expected) empty result
    */



    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();

    for descriptor in descriptors {
        let name = descriptor.header.type_name.clone();
        let p_count = descriptor.properties.properties.len();
        println!("Creating {name} with {p_count} properties");
        let record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_holon_descriptor",
                descriptor.clone(),
            )
            .await;

        let result = get_holon_descriptor_from_record(record).unwrap();
        assert_eq!(descriptor, result);
        println!("... Success!");
    }



}


