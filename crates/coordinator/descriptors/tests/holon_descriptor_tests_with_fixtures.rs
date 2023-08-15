//! Holon Descriptor Test Cases

#![allow(unused_imports)]
// use futures::future;
use std::collections::BTreeMap;
mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
//use std::arch::x86_64::__cpuid_count;
use async_std::task;
use descriptors::helpers::get_holon_descriptor_from_record;
use descriptors::holon_descriptor_storage_fns::UpdateHolonDescriptorInput;
use descriptors::mutators::{
    new_boolean_descriptor, new_composite_descriptor, new_integer_descriptor, new_string_descriptor,
};
use descriptors::property_map_builder::{insert_property_descriptor, remove_property_descriptor};
use rstest::*;
// use shared_test::data_fixtures::{create_dummy_data,derive_type_name,insert_property_descriptor};
use shared_test::fixture_defs::{derive_type_name, rs_dummy_data};
use shared_test::setup_conductor;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonDescriptor;
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails,
    PropertyDescriptorMap,
};
/// This function exercises a broad range of capabilities. The heavy lifting for this test is in the
/// test data set creation done by the `rs_create_dummy_data` fixture. Each member of the vector of
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

#[rstest]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_holon_descriptor_capabilities(
    rs_dummy_data: Result<Vec<HolonDescriptor>, DescriptorsError>,
) {
    // Setup

    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    // The heavy lifting for this test is in the test data set creation. Rich descriptors can be
    // built in the create_dummy_data fn to test a broad range of data structures

    let descriptors: Vec<HolonDescriptor> = rs_dummy_data.unwrap();
    let h_count = descriptors.len();

    println!("******* STARTING TESTS WITH {h_count} HOLON DESCRIPTORS ***************************");

    println!("Performing get_all_holon_types to ensure initial DB state is empty");
    let result: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await;
    assert_eq!(0, result.len());
    println!("Success! Initial DB state has no HolonDescriptors");

    let mut created_action_hashes: Vec<ActionHash> = Vec::new();

    // Iterate through the vector of generated holon descriptors, creating each descriptor,
    // then get the created descriptor and comparing it to the generated descriptor.
    for descriptor in descriptors.clone() {
        let name = descriptor.header.type_name.clone();
        let p_count = descriptor.properties.properties.len();
        println!();
        println!("Starting create/get test for the following HolonDescriptor");
        println!("{:#?}", descriptor);
        println!("Creating {name} with {p_count} properties");

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

        println!(
            "Created descriptor matches generated holon descriptor, fetching created descriptor"
        );

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
        println!("...Success! Fetched descriptor matches generated descriptor.");
    }

    println!("All Holon Descriptors Created... to a get_all_holon_types and compare result with test data...");
    let fetch_all: Vec<Record> = conductor
        .call(&cell.zome("descriptors"), "get_all_holon_types", ())
        .await;
    let d_count = fetch_all.len();
    println!("Call to get_all_holon_types returned {d_count} Holon Descriptors");
    assert_eq!(d_count, h_count);
    for i in 0..d_count {
        let fetched_descriptor =
            get_holon_descriptor_from_record(fetch_all.get(i).unwrap().clone()).unwrap();
        assert_eq!(descriptors[i].clone(), fetched_descriptor);
        println!("Fetched descriptor {i} matches generated descriptor {i}");
    }
    println!();

    // TESTING UPDATES //
    println!("Testing updates...\n");
    // ADD PROPERTY
    let example_string_descriptor_property: PropertyDescriptor = new_string_descriptor(
        "ex_string_prop_desc_update".to_string(),
        "string property description".to_string(),
        true,
        1,
        10,
    )
    .unwrap();
    let mut holon_descriptor1 = descriptors[0].clone();
    insert_property_descriptor(
        &mut holon_descriptor1.properties,
        "additional property".to_string(),
        &example_string_descriptor_property,
    );
    let add_string_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[0].clone(),
        previous_holon_descriptor_hash: created_action_hashes[0].clone(),
        updated_holon_descriptor: holon_descriptor1.clone(),
    };
    let example_add_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            add_string_property_input,
        )
        .await;
    let example_add_property_updated_entry =
        get_holon_descriptor_from_record(example_add_property_updated_record.clone()).unwrap();
    let example_add_property_updated_action_hash: ActionHash =
        example_add_property_updated_record.action_address().clone();
    let fetched_example_add_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            example_add_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_example_add_property_updated_entry = get_holon_descriptor_from_record(
        fetched_example_add_property_updated_record.clone().unwrap(),
    )
    .unwrap();
    println!();
    println!("Added string property to HolonDescriptor1:");
    println!("{:#?}", example_add_property_updated_entry);
    assert_eq!(
        example_add_property_updated_entry,
        fetched_example_add_property_updated_entry
    );

    // REMOVE PROPERTY
    let mut holon_descriptor2 = descriptors[1].clone();
    remove_property_descriptor(
        &mut holon_descriptor2.properties,
        "a_boolean_property".to_string(),
    );
    let remove_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[1].clone(),
        previous_holon_descriptor_hash: created_action_hashes[1].clone(),
        updated_holon_descriptor: holon_descriptor2.clone(),
    };
    let example_remove_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            remove_property_input,
        )
        .await;
    let example_remove_property_updated_entry =
        get_holon_descriptor_from_record(example_remove_property_updated_record.clone()).unwrap();
    let example_remove_property_updated_action_hash: ActionHash =
        example_remove_property_updated_record
            .action_address()
            .clone();
    let fetched_example_remove_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            example_remove_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_example_remove_property_updated_entry = get_holon_descriptor_from_record(
        fetched_example_remove_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Removed boolean property from HolonDescriptor2:");
    println!("{:#?}", example_remove_property_updated_entry);
    assert_eq!(
        example_remove_property_updated_entry,
        fetched_example_remove_property_updated_entry
    );

    // UPDATE STRING SCALAR PROPERTY
    let update_string_descriptor_scalar_property: PropertyDescriptor = new_string_descriptor(
        "ex_string_prop_desc_update".to_string(),
        "change string scalar min max".to_string(),
        true,
        3,
        55,
    )
    .unwrap();
    let mut holon_descriptor3 = descriptors[2].clone();
    insert_property_descriptor(
        &mut holon_descriptor3.properties,
        "a_string_property".to_string(),
        &update_string_descriptor_scalar_property,
    );
    let update_string_scalar_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[2].clone(),
        previous_holon_descriptor_hash: created_action_hashes[2].clone(),
        updated_holon_descriptor: holon_descriptor3.clone(),
    };
    let update_string_scalar_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            update_string_scalar_property_input,
        )
        .await;
    let update_string_scalar_property_updated_entry =
        get_holon_descriptor_from_record(update_string_scalar_property_updated_record.clone())
            .unwrap();
    let update_string_scalar_property_updated_action_hash: ActionHash =
        update_string_scalar_property_updated_record
            .action_address()
            .clone();
    let fetched_update_string_scalar_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            update_string_scalar_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_update_string_scalar_property_updated_entry = get_holon_descriptor_from_record(
        fetched_update_string_scalar_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Updated string scalar property on HolonDescriptor3:");
    println!("{:#?}", update_string_scalar_property_updated_entry);
    assert_eq!(
        update_string_scalar_property_updated_entry,
        fetched_update_string_scalar_property_updated_entry
    );

    // UPDATE INTEGER SCALAR PROPERTY
    let update_integer_descriptor_scalar_property: PropertyDescriptor = new_integer_descriptor(
        "ex_int_prop_desc_update".to_string(),
        "change integer scalar min max".to_string(),
        true,
        IntegerFormat::I8(),
        -42,
        42,
    )
    .unwrap();
    let mut holon_descriptor3 = descriptors[2].clone();
    insert_property_descriptor(
        &mut holon_descriptor3.properties,
        "an_I8_property".to_string(),
        &update_integer_descriptor_scalar_property,
    );
    let update_integer_scalar_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[2].clone(),
        previous_holon_descriptor_hash: created_action_hashes[2].clone(),
        updated_holon_descriptor: holon_descriptor3.clone(),
    };
    let update_integer_scalar_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            update_integer_scalar_property_input,
        )
        .await;
    let update_integer_scalar_property_updated_entry =
        get_holon_descriptor_from_record(update_integer_scalar_property_updated_record.clone())
            .unwrap();
    let update_integer_scalar_property_updated_action_hash: ActionHash =
        update_integer_scalar_property_updated_record
            .action_address()
            .clone();
    let fetched_update_integer_scalar_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            update_integer_scalar_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_update_integer_scalar_property_updated_entry = get_holon_descriptor_from_record(
        fetched_update_integer_scalar_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Updated integer scalar property on HolonDescriptor3:");
    println!("{:#?}", update_integer_scalar_property_updated_entry);
    assert_eq!(
        update_integer_scalar_property_updated_entry,
        fetched_update_integer_scalar_property_updated_entry
    );

    // UPDATE BOOLEAN SCALAR PROPERTY
    let update_boolean_descriptor_scalar_property: PropertyDescriptor = new_boolean_descriptor(
        "ex_bool_prop_desc_update".to_string(),
        "change boolean scalar is_fuzzy to true".to_string(),
        true,
        true,
    )
    .unwrap();
    let mut holon_descriptor3 = descriptors[2].clone();
    insert_property_descriptor(
        &mut holon_descriptor3.properties,
        "a_boolean_property".to_string(),
        &update_boolean_descriptor_scalar_property,
    );
    let update_boolean_scalar_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[2].clone(),
        previous_holon_descriptor_hash: created_action_hashes[2].clone(),
        updated_holon_descriptor: holon_descriptor3.clone(),
    };
    let update_boolean_scalar_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            update_boolean_scalar_property_input,
        )
        .await;
    let update_boolean_scalar_property_updated_entry =
        get_holon_descriptor_from_record(update_boolean_scalar_property_updated_record.clone())
            .unwrap();
    let update_boolean_scalar_property_updated_action_hash: ActionHash =
        update_boolean_scalar_property_updated_record
            .action_address()
            .clone();
    let fetched_update_boolean_scalar_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            update_boolean_scalar_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_update_boolean_scalar_property_updated_entry = get_holon_descriptor_from_record(
        fetched_update_boolean_scalar_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Updated boolean scalar property on HolonDescriptor3:");
    println!("{:#?}", update_boolean_scalar_property_updated_entry);
    assert_eq!(
        update_boolean_scalar_property_updated_entry,
        fetched_update_boolean_scalar_property_updated_entry
    );

    // ADD COMPOSITE PROPERTY
    let example_property_descriptor_map = BTreeMap::from([
        (
            "ex_string_desc".to_string(),
            new_string_descriptor(
                "example composite string descriptor".to_string(),
                "composite string description".to_string(),
                true,
                0,
                7,
            )
            .unwrap(),
        ),
        (
            "ex_integer_desc".to_string(),
            new_integer_descriptor(
                "example composite integer descriptor".to_string(),
                "composite integer description".to_string(),
                true,
                IntegerFormat::I32(),
                -987654321,
                123456789,
            )
            .unwrap(),
        ),
        (
            "ex_boolean_desc".to_string(),
            new_boolean_descriptor(
                "example composite boolean descriptor".to_string(),
                "composite boolean description".to_string(),
                true,
                false,
            )
            .unwrap(),
        ),
    ]);
    let mut holon_descriptor4 = descriptors[3].clone();
    //?TODO: break out the following into function for updating a composite descriptor
    let original_composite_property_descriptor = descriptors[3]
        .properties
        .properties
        .get("a_composite_property")
        .unwrap();
    let mut composite_descriptor_map =
        get_composite_descriptor_map(&original_composite_property_descriptor.details);
    for (k, v) in example_property_descriptor_map.iter() {
        composite_descriptor_map
            .properties
            .insert(k.clone(), v.clone());
    }
    let updated_composite_descriptor = PropertyDescriptor {
        header: original_composite_property_descriptor.header.clone(),
        details: PropertyDescriptorDetails::Composite(CompositeDescriptor {
            properties: composite_descriptor_map.clone(),
        }),
    };
    insert_property_descriptor(
        &mut holon_descriptor4.properties,
        "a_composite_property".to_string(),
        &updated_composite_descriptor,
    );
    let add_composite_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[3].clone(),
        previous_holon_descriptor_hash: created_action_hashes[3].clone(),
        updated_holon_descriptor: holon_descriptor4.clone(),
    };
    let example_add_composite_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            add_composite_property_input,
        )
        .await;
    let example_add_composite_property_updated_entry =
        get_holon_descriptor_from_record(example_add_composite_property_updated_record.clone())
            .unwrap();
    let example_add_composite_property_updated_action_hash: ActionHash =
        example_add_composite_property_updated_record
            .action_address()
            .clone();
    let fetched_example_add_composite_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            example_add_composite_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_example_add_composite_property_updated_entry = get_holon_descriptor_from_record(
        fetched_example_add_composite_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Added properties to composite descriptor on HolonDescriptor4:");
    println!("{:#?}", example_add_composite_property_updated_entry);
    assert_eq!(
        example_add_composite_property_updated_entry,
        fetched_example_add_composite_property_updated_entry
    );

    // REMOVE COMPOSITE PROPERTY
    composite_descriptor_map.properties.remove("ex_string_desc");
    let updated_composite_descriptor_removed_ex_string = PropertyDescriptor {
        header: original_composite_property_descriptor.header.clone(),
        details: PropertyDescriptorDetails::Composite(CompositeDescriptor {
            properties: composite_descriptor_map,
        }),
    };
    insert_property_descriptor(
        &mut holon_descriptor4.properties,
        "a_composite_property".to_string(),
        &updated_composite_descriptor_removed_ex_string,
    );
    let remove_composite_property_input = UpdateHolonDescriptorInput {
        original_holon_descriptor_hash: created_action_hashes[3].clone(),
        previous_holon_descriptor_hash: example_add_composite_property_updated_action_hash.clone(),
        updated_holon_descriptor: holon_descriptor4.clone(),
    };
    let example_remove_composite_property_updated_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "update_holon_descriptor",
            remove_composite_property_input,
        )
        .await;
    let example_remove_composite_property_updated_entry =
        get_holon_descriptor_from_record(example_remove_composite_property_updated_record.clone())
            .unwrap();
    let example_remove_composite_property_updated_action_hash: ActionHash =
        example_remove_composite_property_updated_record
            .action_address()
            .clone();
    let fetched_example_remove_composite_property_updated_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_holon_descriptor",
            example_remove_composite_property_updated_action_hash.clone(),
        )
        .await;
    let fetched_example_remove_composite_property_updated_entry = get_holon_descriptor_from_record(
        fetched_example_remove_composite_property_updated_record
            .clone()
            .unwrap(),
    )
    .unwrap();
    println!();
    println!("Removed ex_string_desc property from composite descriptor on HolonDescriptor4:");
    println!("{:#?}", example_remove_composite_property_updated_entry);
    assert_eq!(
        example_remove_composite_property_updated_entry,
        fetched_example_remove_composite_property_updated_entry
    );

    // TESTING DELETES //

    // for hash in created_action_hashes {
    //     let _action_hash_of_delete: ActionHash = conductor
    //         .call(
    //             &cell.zome("descriptors"),
    //             "delete_holon_descriptor",
    //             hash.clone(),
    //         )
    //         .await;

    //     let try_query: Option<Record> = conductor
    //         .call(&cell.zome("descriptors"), "get_holon_descriptor", hash)
    //         .await;

    //     assert!(try_query.is_none());
    // }

    // let fetch_all_check_deleted: Vec<Record> = conductor
    //     .call(&cell.zome("descriptors"), "get_all_holon_types", ())
    //     .await;
    // assert!(fetch_all_check_deleted.is_empty());
}

fn get_composite_descriptor_map(details: &PropertyDescriptorDetails) -> PropertyDescriptorMap {
    match details {
        PropertyDescriptorDetails::Composite(map) => map.properties.clone(),
        _ => panic!("error matching composite details"),
    }
}
