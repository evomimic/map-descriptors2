mod conductor;

use descriptors::mutators::new_property_descriptor;
use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::PropertyDescriptor;
use shared_types_descriptor::type_header::BaseType;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_delete_holon_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();

    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            descriptors[0].clone(),
        )
        .await;

    let action_hash_of_create: ActionHash = created_record.action_address().clone();

    let _action_hash_of_delete: ActionHash = conductor
        .call(
            &cell.zome("descriptors"),
            "delete_holon_descriptor",
            action_hash_of_create.clone(),
        )
        .await;

    let try_query: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            action_hash_of_create,
        )
        .await;

    assert!(try_query.is_none());
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_delete_property_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let descriptor: PropertyDescriptor = new_property_descriptor(
        "ex_prop_desc".to_string(),
        "example description".to_string(),
        BaseType::Composite,
        true,
    )
    .unwrap();

    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            descriptor.clone(),
        )
        .await;

    let action_hash_of_create: ActionHash = created_record.action_address().clone();

    let _action_hash_of_delete: ActionHash = conductor
        .call(
            &cell.zome("descriptors"),
            "delete_property_descriptor",
            action_hash_of_create.clone(),
        )
        .await;

    let try_query: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            action_hash_of_create,
        )
        .await;

    assert!(try_query.is_none());
}
