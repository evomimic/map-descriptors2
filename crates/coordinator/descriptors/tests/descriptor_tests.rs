#![warn(warnings)]

// use futures::future;
// use std::collections::BTreeMap;
use descriptors::helpers::get_descriptor_from_record;
use descriptors::stub_data_creator::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetAgents, SweetCell, SweetConductor, SweetDnaFile};

use shared_types_descriptor::holon_descriptor::HolonDescriptor;

const DNA_FILEPATH: &str = "../../../workdir/map_descriptors.dna";

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_all_holontypes() {
//     let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
//         setup_conductor().await;

//     let testing_descriptors: Vec<HolonDescriptor> = conductor
//         .call(&cell.zome("descriptors"), "get_all_holontypes", ())
//         .await;

//     println!("{:#?}", testing_descriptors);
// }

/// MOCK CONDUCTOR

async fn setup_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
    let dna = SweetDnaFile::from_bundle(std::path::Path::new(&DNA_FILEPATH))
        .await
        .unwrap();

    // let dna_path = std::env::current_dir().unwrap().join(DNA_FILEPATH);
    // println!("{}", dna_path.to_string_lossy());
    // let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    let mut conductor = SweetConductor::from_standard_config().await;

    let holo_core_agent = SweetAgents::one(conductor.keystore()).await;
    let app = conductor
        .setup_app_for_agent("app", holo_core_agent.clone(), &[dna.clone()])
        .await
        .unwrap();

    let cell = app.into_cells()[0].clone();

    let agent_hash = holo_core_agent.into_inner();
    let agent = AgentPubKey::from_raw_39(agent_hash).unwrap();

    (conductor, agent, cell)
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_holon_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();

    let record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            descriptors[0].clone(),
        )
        .await;

    let entry = get_descriptor_from_record(record).unwrap();
    // println!("{:#?}", entry);
    assert_eq!(descriptors[0], entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_holon_descriptor() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = create_dummy_data(()).unwrap();

    // println!("{:#?}", descriptors);

    let created_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_holon_descriptor",
            descriptors[0].clone(),
        )
        .await;

    let action_hash: ActionHash = created_record.action_address().clone();

    let fetched_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            action_hash,
        )
        .await;
    // println!("{:#?}", fetched_record);

    let entry = get_descriptor_from_record(fetched_record.unwrap()).unwrap();
    println!("{:#?}", entry);

    assert_eq!(descriptors[0], entry);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_holon_types() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

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

    for hash in action_hashes {
        let fetched_record: Option<Record> = conductor
            .call(&cell.zome("descriptors"), "get_holon_descriptor", hash)
            .await;

        let entry = get_descriptor_from_record(fetched_record.unwrap()).unwrap();

        fetched_entries.push(entry);
    }

    assert_eq!(mock_descriptors, fetched_entries);
}
