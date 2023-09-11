//! Property Descriptor Test Cases

// #![allow(unused_imports)]
// #![allow(unused_doc_comments)]

mod shared_test;

// use core::panic;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::{
    get_composite_descriptor_from_details, get_composite_descriptor_map,
    get_holon_reference_from_sharing, get_property_descriptor_from_record,
};
use descriptors::property_map_builder::upsert_property_descriptor;

use rstest::*;
use shared_test::fixture_defs::{
    new_dedicated_property_descriptors_fixture, new_shared_property_descriptors_fixture,
};
use shared_test::SharedTypesTestCase;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, DescriptorSharing, PropertyDescriptor, PropertyDescriptorDetails,
    PropertyDescriptorMap, PropertyDescriptorUsage,
};

use shared_types_descriptor::holon_descriptor::HolonReference;

/// This function exercises a broad range of capabilities. The heavy lifting for this test is in the
/// test data set creation done within fixtures.
///
/// Test Outline:
/// 1. After initial setup, perform a `get_all_property_types`, with an expectation of an empty result
/// 2. For each member of the `descriptors` vector, perform a `create` followed by a `get` and verify
/// 3. Once all data has been created in DHT, perform `get_all_property_types` and verify the result.
///
/// Note that this will exercise, create, get, and get_all capabilities across a variety of
/// holon descriptors
///
/// To selectively run JUST THE TESTS in this file, use:
///      cargo test -p descriptors --test property_descriptor_tests  -- --show-output
///
///
#[rstest]
#[case::mixture_of_dedicated_property_types(new_dedicated_property_descriptors_fixture())]
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
        // println!("{:#?}", descriptor);

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

/// Testing shared variants of properties within a composite
#[rstest]
#[case::mixture_of_property_types(new_shared_property_descriptors_fixture())]
#[tokio::test(flavor = "multi_thread")]
async fn rstest_shared_properties(#[case] input: Result<SharedTypesTestCase, DescriptorsError>) {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        shared_test::setup_conductor().await;

    println!(
        "******* STARTING TESTS FOR SHARED PROPERTY DESCRIPTORS *************************** \n"
    );

    let test_case = input.unwrap();
    let shared_types = test_case.shared_types;
    let referencing_types = test_case.referencing_types;

    let shared_string_name = "Shared_String_Type_example".to_string();
    let shared_integer_name = "Shared_I64_Integer_Type_example".to_string();
    let shared_boolean_name = "Shared_Boolean_Type_example".to_string();

    let mut created_shared_types: Vec<PropertyDescriptor> = Vec::new();
    // Create each shared type as an entry in Holochain, then collect them
    for descriptor in shared_types {
        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                descriptor.clone(),
            )
            .await;
        let mut created_descriptor = descriptor.clone();
        let name = get_holon_reference_from_sharing(&descriptor.sharing).name;
        created_descriptor.sharing = DescriptorSharing::Shared(HolonReference {
            id: Some(created_record.action_address().clone()),
            name: name,
        });
        created_shared_types.push(created_descriptor);
        let fetched_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_property_descriptor",
                created_record.action_address().clone(),
            )
            .await;
        let fetched_descriptor =
            get_property_descriptor_from_record(fetched_record.unwrap()).unwrap();
        assert_eq!(descriptor, fetched_descriptor);
    }

    let created_shared_string_type: PropertyDescriptor = created_shared_types
        .iter()
        .find(|descriptor| descriptor.header.type_name == shared_string_name)
        .unwrap()
        .clone();

    let created_shared_integer_type: PropertyDescriptor = created_shared_types
        .iter()
        .find(|descriptor| descriptor.header.type_name == shared_integer_name)
        .unwrap()
        .clone();

    let created_shared_boolean_type: PropertyDescriptor = created_shared_types
        .iter()
        .find(|descriptor| descriptor.header.type_name == shared_boolean_name)
        .unwrap()
        .clone();

    // println!(
    //     "string: {:#?}, \n int: {:#?}, \n bool: {:#?}",
    //     shared_string_tuple, shared_integer_tuple, shared_boolean_tuple
    // );
    // println!("referencing types: {:#?}", referencing_types);

    // TESTING SHARED STRING TYPE //

    // Iterate referencing types to find TestComposite1
    let mut composite1_referencing_string_descriptor: PropertyDescriptor = referencing_types
        .iter()
        .find(|descriptor| {
            descriptor.header.type_name
                == "TestComposite1__Composite_Type__referencing_string".to_string()
        })
        .unwrap()
        .clone();
    // Get the associated map, then upsert the name and usage
    let mut composite1_map: PropertyDescriptorMap =
        get_composite_descriptor_map(&composite1_referencing_string_descriptor.details);
    let composite1_usage = PropertyDescriptorUsage::new(
        "testing reference to shared string type".to_string(),
        created_shared_string_type.clone(),
    );
    upsert_property_descriptor(
        &mut composite1_map,
        shared_string_name.clone(),
        &composite1_usage,
    );
    // Update the details
    composite1_referencing_string_descriptor.details =
        PropertyDescriptorDetails::Composite(CompositeDescriptor {
            properties: composite1_map,
        });
    // println!("{:#?}", composite1_referencing_string_descriptor);

    // Create the referencing Composite type, then fetch it and compare
    let created_composite1_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            composite1_referencing_string_descriptor.clone(),
        )
        .await;
    let fetched_composite1_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            created_composite1_record.action_address().clone(),
        )
        .await;
    let fetched_composite1 =
        get_property_descriptor_from_record(fetched_composite1_record.unwrap()).unwrap();
    assert_eq!(composite1_referencing_string_descriptor, fetched_composite1);
    // Retrieve the ActionHash of the shared string type from the HolonReference, then fetch the property and compare
    let referenced_string_sharing =
        get_composite_descriptor_from_details(&fetched_composite1.details)
            .properties
            .properties
            .get(&shared_string_name)
            .unwrap()
            .descriptor
            .sharing
            .clone();
    let referenced_string_action_hash =
        get_holon_reference_from_sharing(&referenced_string_sharing)
            .id
            .unwrap();
    let fetched_referenced_string_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            referenced_string_action_hash,
        )
        .await;
    let fetched_referenced_string_descriptor =
        get_property_descriptor_from_record(fetched_referenced_string_record.unwrap()).unwrap();
    assert_eq!(
        created_shared_string_type,
        fetched_referenced_string_descriptor
    );

    // TESTING SHARED INTEGER TYPE I64 //

    // Iterate referencing types to find TestComposite2
    let mut composite2_referencing_integer_descriptor: PropertyDescriptor = referencing_types
        .iter()
        .find(|descriptor| {
            descriptor.header.type_name
                == "TestComposite2__Composite_Type__referencing_integer".to_string()
        })
        .unwrap()
        .clone();
    // Get the associated map, then upsert the name and usage
    let mut composite2_map: PropertyDescriptorMap =
        get_composite_descriptor_map(&composite2_referencing_integer_descriptor.details);
    let composite2_usage = PropertyDescriptorUsage::new(
        "testing reference to shared integer type".to_string(),
        created_shared_integer_type.clone(),
    );
    upsert_property_descriptor(
        &mut composite2_map,
        shared_integer_name.clone(),
        &composite2_usage,
    );
    // Update the details
    composite2_referencing_integer_descriptor.details =
        PropertyDescriptorDetails::Composite(CompositeDescriptor {
            properties: composite2_map,
        });
    // println!("{:#?}", composite2_referencing_integer);

    // Create the referencing Composite type, then fetch it and compare
    let created_composite2_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            composite2_referencing_integer_descriptor.clone(),
        )
        .await;
    let fetched_composite2_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            created_composite2_record.action_address().clone(),
        )
        .await;
    let fetched_composite2 =
        get_property_descriptor_from_record(fetched_composite2_record.unwrap()).unwrap();
    assert_eq!(
        composite2_referencing_integer_descriptor,
        fetched_composite2
    );
    // Retrieve the ActionHash of the shared integer type from the HolonReference, then fetch the property and compare
    let referenced_integer_sharing =
        get_composite_descriptor_from_details(&fetched_composite2.details)
            .properties
            .properties
            .get(&shared_integer_name)
            .unwrap()
            .descriptor
            .sharing
            .clone();
    let referenced_integer_action_hash =
        get_holon_reference_from_sharing(&referenced_integer_sharing)
            .id
            .unwrap();
    let fetched_referenced_integer_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            referenced_integer_action_hash,
        )
        .await;
    let fetched_referenced_integer_descriptor =
        get_property_descriptor_from_record(fetched_referenced_integer_record.unwrap()).unwrap();
    assert_eq!(
        created_shared_integer_type,
        fetched_referenced_integer_descriptor
    );

    // TESTING SHARED BOOLEAN TYPE //

    // Iterate referencing types to find TestComposite3
    let mut composite3_referencing_boolean_descriptor: PropertyDescriptor = referencing_types
        .iter()
        .find(|descriptor| {
            descriptor.header.type_name
                == "TestComposite1__Composite_Type__referencing_boolean".to_string()
        })
        .unwrap()
        .clone();
    // Get the associated map, then upsert the name and usage
    let mut composite3_map: PropertyDescriptorMap =
        get_composite_descriptor_map(&composite3_referencing_boolean_descriptor.details);
    let composite3_usage = PropertyDescriptorUsage::new(
        "testing reference to shared boolean type".to_string(),
        created_shared_boolean_type.clone(),
    );
    upsert_property_descriptor(
        &mut composite3_map,
        shared_boolean_name.clone(),
        &composite3_usage,
    );
    // Update the details
    composite3_referencing_boolean_descriptor.details =
        PropertyDescriptorDetails::Composite(CompositeDescriptor {
            properties: composite3_map,
        });
    // println!("{:#?}", composite3_referencing_boolean);

    // Create the referencing Composite type, then fetch it and compare
    let created_composite3_record: Record = conductor
        .call(
            &cell.zome("descriptors"),
            "create_property_descriptor",
            composite3_referencing_boolean_descriptor.clone(),
        )
        .await;
    let fetched_composite3_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            created_composite3_record.action_address().clone(),
        )
        .await;
    let fetched_composite3 =
        get_property_descriptor_from_record(fetched_composite3_record.unwrap()).unwrap();
    assert_eq!(
        composite3_referencing_boolean_descriptor,
        fetched_composite3
    );
    // Retrieve the ActionHash of the shared boolean type from the HolonReference, then fetch the property and compare
    let referenced_boolean_sharing =
        get_composite_descriptor_from_details(&fetched_composite3.details)
            .properties
            .properties
            .get(&shared_boolean_name)
            .unwrap()
            .descriptor
            .sharing
            .clone();
    let referenced_boolean_action_hash =
        get_holon_reference_from_sharing(&referenced_boolean_sharing)
            .id
            .unwrap();
    let fetched_referenced_boolean_record: Option<Record> = conductor
        .call(
            &cell.zome("descriptors"),
            "get_property_descriptor",
            referenced_boolean_action_hash,
        )
        .await;
    let fetched_referenced_boolean_descriptor =
        get_property_descriptor_from_record(fetched_referenced_boolean_record.unwrap()).unwrap();
    assert_eq!(
        created_shared_boolean_type,
        fetched_referenced_boolean_descriptor
    );
}
