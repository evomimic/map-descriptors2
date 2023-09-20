//! Property Descriptor Updates Test Cases

mod shared_test;

use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use std::collections::BTreeMap;

use descriptors::helpers::*;
use rstest::*;

use shared_test::property_descriptor_fixtures::*;
use shared_test::test_data_types::SharedTypesTestCase;
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::holon_descriptor::HolonReference;
use shared_types_descriptor::property_descriptor::{
    CompositeDescriptor, DescriptorSharing, PropertyDescriptor, PropertyDescriptorDetails,
};

/// To selectively run JUST THE TESTS in this file, use:
///      cargo test -p descriptors --test shared_property_descriptor_tests -- --show-output
///
/// This function exercises a broad range of capabilities related to references to shared types
/// from within composite types. The heavy lifting for this test is in the
/// SharedTypesTestCase creation done within fixtures.
///
/// Each such test case contains a vector of shared property types and a vector of referencing
/// property types. The latter are composites that contain properties that reference the
/// shared types. Since the fixture doesn't know the ActionHash of the shared type at the time
/// the referenced_types are created, it stores the type_name of the shared type in the
/// HolonReference.
///
/// In the test function, a type_name_map of (type_name -> ActionHash) is populated as the
/// shared_types are persisted in holochain.
///
/// Test Outline:
/// 1. After initial setup, create & persist the shared_types. For each shared_type,
///    perform a `create` followed by a `get`, assert input type = fetched type. Then add
///    an entry for that shared type to the type_name_map.
/// 2. Once all data has been created in DHT, perform `get_all_holon_types` and verify the result.
/// 3. Once the shared_types have been verified as having been created correctly, iterate through
///     the referencing_types.
/// 4. For each referencing_type, iterate through its contained properties, extract the name of the
///    shared_type from its HolonReference, lookup the ActionHash for that type_name in the
///    type_name_map and update the HolonReference with that ActionHash.
///
///    Once HolonReference for all of the referencing type's properties have been populated, then
///    persist the referencing_type in holochain, fetch it, and assert expected = actual.
/// 5. Iterate through the fetched referencing type descriptor and, for each of its properties,
///    fetch the shared property descriptor using via the ActionHash within its HolonRefernce.
///    Verify that the fetched type descriptor = the expected shared_type descriptor.
///
///
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
    let mut referencing_types = test_case.referencing_types;

    let mut created_shared_types: Vec<PropertyDescriptor> = Vec::new();
    let mut type_name_map: BTreeMap<String, ActionHash> = BTreeMap::new();

    // Create each shared type as an entry in Holochain, then collect them
    for descriptor in shared_types.clone() {
        // println!("shared type: {:#?}", descriptor);
        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                descriptor.clone(),
            )
            .await;
        type_name_map.insert(
            descriptor.header.type_name.clone(),
            created_record.action_address().clone(),
        );
        created_shared_types.push(descriptor.clone());
        let fetched_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_property_descriptor",
                created_record.action_address().clone(),
            )
            .await;
        let fetched_descriptor =
            get_property_descriptor_from_record(fetched_record.unwrap()).unwrap();
        assert_eq!(&descriptor, &fetched_descriptor);
    }
    // All shared types have been created, now create the type(s) that reference them
    // Each referencing type is assumed to be a composite containing one or more properties
    // that references one or more of the shared types.
    // In order to set the HolonReference ActionHash properly, we need to get the name
    // of the referenced type out of each property's HolonReference,
    // fetch that PropertyType and use the ActionHash from the fetched record to set the
    // id in the HolonReference for that property.
    // Then create the referencing type descriptor.
    //
    // Iterate through the referenced types

    for composite in &mut referencing_types {
        // println!("composite: {:#?}", composite);
        // First get the composite's properties
        let composite_properties_result = match composite.details.clone() {
            PropertyDescriptorDetails::Composite(composite_details) => {
                Ok(composite_details.properties)
            }
            _ => Err("Error: Expected Composite Type"), // make this an Error: Expected Composite Type
        };
        let mut composite_properties = composite_properties_result.unwrap();

        // iterate through this composite's properties, extracting the name of the shared
        // Property Descriptor that that property references.
        // Then fetch the referenced descriptor and add its actionHash to the
        // PropertyDescriptorUsage's HolonReference.
        for (referenced_property_name, referenced_property_usage) in
        composite_properties.clone().properties
        {
            let referenced_name = match referenced_property_usage.sharing.clone() {
                DescriptorSharing::Shared(reference) => reference.name,
                _ => None,
            };

            let mut property_usage_with_hash = referenced_property_usage.clone();

            if let Some(name) = referenced_name {
                let action_hash = type_name_map.get(&name);
                if let Some(action_hash) = action_hash {
                    let reference = HolonReference::new(Some(action_hash.clone()), Some(name));
                    property_usage_with_hash.sharing = DescriptorSharing::Shared(reference);
                } else {
                    panic!("Couldn't find referenced type in the list of shared types provided.");
                }
            };
            composite_properties
                .properties
                .insert(referenced_property_name, property_usage_with_hash);

            composite.details = PropertyDescriptorDetails::Composite(CompositeDescriptor::new(
                composite_properties.clone(),
            ));
        }

        println!("composite {:#?}", composite);

        // All of the HolonReferences have been updated with their id, ready to create the composite type

        let created_composite_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                composite.clone(),
            )
            .await;
        let fetched_composite_record: Option<Record> = conductor
            .call(
                &cell.zome("descriptors"),
                "get_property_descriptor",
                created_composite_record.action_address().clone(),
            )
            .await;
        assert_eq!(
            created_composite_record.clone(),
            fetched_composite_record.clone().unwrap()
        );

        let fetched_property_descriptor =
            get_property_descriptor_from_record(fetched_composite_record.unwrap()).unwrap();

        // println!(
        //     "created_composite_descriptor {:#?}",
        //     &fetched_property_descriptor
        // );

        let fetched_composite_map =
            get_composite_descriptor_map(&fetched_property_descriptor.details);

        for (fetched_property_name, fetched_property_usage) in
        fetched_composite_map.properties.iter()
        {
            let fetched_holon_reference =
                get_holon_reference_from_sharing(&fetched_property_usage.sharing);

            // Not sure the following is foolproof way of retrieving the original shared_type
            // for this usage, but it is probably good enough for now

            let created_shared_type = composite_properties
                .clone()
                .properties
                .get(fetched_property_name)
                .unwrap()
                .clone()
                .descriptor;

            let fetched_shared_descriptor_record: Option<Record> = conductor
                .call(
                    &cell.zome("descriptors"),
                    "get_property_descriptor",
                    fetched_holon_reference.id,
                )
                .await;

            let fetched_shared_descriptor =
                get_property_descriptor_from_record(fetched_shared_descriptor_record.unwrap())
                    .unwrap();

            assert_eq!(created_shared_type, fetched_shared_descriptor);
        }
    }
}
