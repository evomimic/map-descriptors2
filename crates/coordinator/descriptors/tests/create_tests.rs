// use futures::future;
// use std::collections::BTreeMap;
/*
mod shared_test;

use descriptors::helpers::{get_holon_descriptor_from_record, get_property_descriptor_from_record};
use descriptors::mutators::new_property_descriptor;
use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;
use shared_types_descriptor::type_header::BaseType;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_holon_descriptor() {
    // Setup
    let (shared_test, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();
    // Execute
    let record: Record = shared_test
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            descriptors[0].clone(),
        )
        .await;

    // Verify
    let entry = get_holon_descriptor_from_record(record).unwrap();
    // println!("{:#?}", entry);
    assert_eq!(descriptors[0], entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_property_descriptor() {
    let (shared_test, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    let descriptor: PropertyDescriptor = new_property_descriptor(
        "ex_prop_desc".to_string(),
        "example description".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let record: Record = shared_test
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            descriptor.clone(),
        )
        .await;

    let entry = get_property_descriptor_from_record(record).unwrap();
    // println!("{:#?}", entry);
    assert_eq!(descriptor, entry);
}
*/