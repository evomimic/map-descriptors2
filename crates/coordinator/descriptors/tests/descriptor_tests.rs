#![warn(warnings)]
mod conductor;
/*
// use futures::future;
// use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::sweettest::{SweetAgents, SweetCell, SweetConductor, SweetDnaFile};

use shared_types_descriptor::holon_descriptor::HolonDescriptor;

const DNA_FILEPATH: &str = "../../../workdir/map_descriptors.dna";

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_holontypes() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        conductor::setup_conductor().await;

    let testing_descriptors: Vec<HolonDescriptor> = conductor
        .call(&cell.zome("descriptors"), "get_all_holontypes", ())
        .await;

    println!("{:?}", testing_descriptors);
}
*/
