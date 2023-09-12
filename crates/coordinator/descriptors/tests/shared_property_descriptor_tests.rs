//! Property Descriptor Updates Test Cases

mod shared_test;

use std::collections::BTreeMap;
use hdk::prelude::*;
use holochain::sweettest::{SweetCell, SweetConductor};

use descriptors::helpers::*;
use descriptors::property_descriptor_storage_fns::UpdatePropertyDescriptorInput;
use rstest::*;

use shared_test::test_data_types::{PropertyDescriptorTestCase, SharedTypesTestCase};
use shared_types_descriptor::error::DescriptorsError;
use shared_types_descriptor::property_descriptor::{PropertyDescriptor, PropertyDescriptorMap, PropertyDescriptorDetails, DescriptorSharing};
use shared_test::property_descriptor_fixtures::*;
use shared_types_descriptor::holon_descriptor::HolonReference;


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


    let mut created_shared_types: Vec<PropertyDescriptor> = Vec::new();
    let mut type_name_map: BTreeMap<String, ActionHash> = BTreeMap::new();

    // Create each shared type as an entry in Holochain, then collect them
    for descriptor in shared_types.clone() {
        println!("Creating shared type: {:#?} \n", &descriptor);
        let created_record: Record = conductor
            .call(
                &cell.zome("descriptors"),
                "create_property_descriptor",
                descriptor.clone(),
            )
            .await;
        let mut created_descriptor = descriptor.clone();
        type_name_map.push(created_descriptor.header.type_name.clone(), created_record.action_address().clone());
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

    for composite in referencing_types.clone() {

        // First get the composite's properties
        // let mut composite_properties = PropertyDescriptorMap::new(BTreeMap::new());
        //  let mut composite_property_map = PropertyDescriptorMap::new;
        let mut composite_properties = match composite.details {
            PropertyDescriptorDetails::Composite(composite_details) => {
                Ok(composite_details.properties)
            }
            _ => { Err("Error: Expected Composite Type") } // make this an Error: Expected Composite Type
        };

        // iterate through this composite's properties, extracting the name of the shared
        // Property Descriptor that that property references.
        // Then fetch the referenced descriptor and add its actionHash to the
        // PropertyDescriptorUsage's HolonReference.
        for (referenced_property, referenced_property_usage) in composite_properties.unwrap().properties.iter() {
            let referenced_name = match referenced_property_usage.sharing.clone() {
                DescriptorSharing::Shared(reference) => {
                    reference.name
                }
                _ => {
                    None
                }
            };

            let &mut property_usage_with_hash = referenced_property_usage.clone();

            if let Some(name) = referenced_name {
                let action_hash = type_name_map.get(&name);
                if action_hash.is_none() {
                    panic!("Couldn't find referenced type in the list of shared types provided.");
                };
                let reference = HolonReference::new(action_hash, name);
                property_usage_with_hash.sharing = DescriptorSharing::Shared(reference);
            } else {
                None
            }
        }

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
    }

    // let fetched_referenced_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         created_record.action_address().clone(),
    //     )
    //     .await;
    // let fetched_descriptor =
    //     get_property_descriptor_from_record(fetched_record.unwrap()).unwrap();


    // let created_shared_string_type: PropertyDescriptor = created_shared_types
    //     .iter()
    //     .find(|descriptor| descriptor.header.type_name == shared_string_name)
    //     .unwrap()
    //     .clone();
    //
    // let created_shared_integer_type: PropertyDescriptor = created_shared_types
    //     .iter()
    //     .find(|descriptor| descriptor.header.type_name == shared_integer_name)
    //     .unwrap()
    //     .clone();
    //
    // let created_shared_boolean_type: PropertyDescriptor = created_shared_types
    //     .iter()
    //     .find(|descriptor| descriptor.header.type_name == shared_boolean_name)
    //     .unwrap()
    //     .clone();

    // println!(
    //     "string: {:#?}, \n int: {:#?}, \n bool: {:#?}",
    //     shared_string_tuple, shared_integer_tuple, shared_boolean_tuple
    // );
    // println!("referencing types: {:#?}", referencing_types);

    // TESTING SHARED STRING TYPE //

    // Iterate referencing types to find TestComposite1
    // let mut composite1_referencing_string_descriptor: PropertyDescriptor = referencing_types
    //     .iter()
    //     .find(|descriptor| {
    //         descriptor.header.type_name
    //             == "TestComposite1__Composite_Type__referencing_string".to_string()
    //     })
    //     .unwrap()
    //     .clone();
    // // Get the associated map, then upsert the name and usage
    // let mut composite1_map: PropertyDescriptorMap =
    //     get_composite_descriptor_map(&composite1_referencing_string_descriptor.details);
    // let composite1_usage = PropertyDescriptorUsage::new(
    //     "testing reference to shared string type".to_string(),
    //     created_shared_string_type.clone(),
    // );
    // upsert_property_descriptor(
    //     &mut composite1_map,
    //     shared_string_name.clone(),
    //     &composite1_usage,
    // );
    // // Update the details
    // composite1_referencing_string_descriptor.details =
    //     PropertyDescriptorDetails::Composite(CompositeDescriptor {
    //         properties: composite1_map,
    //     });
    // // println!("{:#?}", composite1_referencing_string_descriptor);
    //
    // // Create the referencing Composite type, then fetch it and compare
    // let created_composite1_record: Record = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "create_property_descriptor",
    //         composite1_referencing_string_descriptor.clone(),
    //     )
    //     .await;
    // let fetched_composite1_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         created_composite1_record.action_address().clone(),
    //     )
    //     .await;
    // let fetched_composite1 =
    //     get_property_descriptor_from_record(fetched_composite1_record.unwrap()).unwrap();
    // assert_eq!(composite1_referencing_string_descriptor, fetched_composite1);
    // // Retrieve the ActionHash of the shared string type from the HolonReference, then fetch the property and compare
    // let referenced_string_sharing =
    //     get_composite_descriptor_from_details(&fetched_composite1.details)
    //         .properties
    //         .properties
    //         .get(&shared_string_name)
    //         .unwrap()
    //         .descriptor
    //         .sharing
    //         .clone();
    // let referenced_string_action_hash =
    //     get_holon_reference_from_sharing(&referenced_string_sharing)
    //         .id
    //         .unwrap();
    // let fetched_referenced_string_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         referenced_string_action_hash,
    //     )
    //     .await;
    // let fetched_referenced_string_descriptor =
    //     get_property_descriptor_from_record(fetched_referenced_string_record.unwrap()).unwrap();
    // assert_eq!(
    //     created_shared_string_type,
    //     fetched_referenced_string_descriptor
    // );
    //
    // // TESTING SHARED INTEGER TYPE I64 //
    //
    // // Iterate referencing types to find TestComposite2
    // let mut composite2_referencing_integer_descriptor: PropertyDescriptor = referencing_types
    //     .iter()
    //     .find(|descriptor| {
    //         descriptor.header.type_name
    //             == "TestComposite2__Composite_Type__referencing_integer".to_string()
    //     })
    //     .unwrap()
    //     .clone();
    // // Get the associated map, then upsert the name and usage
    // let mut composite2_map: PropertyDescriptorMap =
    //     get_composite_descriptor_map(&composite2_referencing_integer_descriptor.details);
    // let composite2_usage = PropertyDescriptorUsage::new(
    //     "testing reference to shared integer type".to_string(),
    //     created_shared_integer_type.clone(),
    // );
    // upsert_property_descriptor(
    //     &mut composite2_map,
    //     shared_integer_name.clone(),
    //     &composite2_usage,
    // );
    // // Update the details
    // composite2_referencing_integer_descriptor.details =
    //     PropertyDescriptorDetails::Composite(CompositeDescriptor {
    //         properties: composite2_map,
    //     });
    // // println!("{:#?}", composite2_referencing_integer);
    //
    // // Create the referencing Composite type, then fetch it and compare
    // let created_composite2_record: Record = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "create_property_descriptor",
    //         composite2_referencing_integer_descriptor.clone(),
    //     )
    //     .await;
    // let fetched_composite2_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         created_composite2_record.action_address().clone(),
    //     )
    //     .await;
    // let fetched_composite2 =
    //     get_property_descriptor_from_record(fetched_composite2_record.unwrap()).unwrap();
    // assert_eq!(
    //     composite2_referencing_integer_descriptor,
    //     fetched_composite2
    // );
    // // Retrieve the ActionHash of the shared integer type from the HolonReference, then fetch the property and compare
    // let referenced_integer_sharing =
    //     get_composite_descriptor_from_details(&fetched_composite2.details)
    //         .properties
    //         .properties
    //         .get(&shared_integer_name)
    //         .unwrap()
    //         .descriptor
    //         .sharing
    //         .clone();
    // let referenced_integer_action_hash =
    //     get_holon_reference_from_sharing(&referenced_integer_sharing)
    //         .id
    //         .unwrap();
    // let fetched_referenced_integer_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         referenced_integer_action_hash,
    //     )
    //     .await;
    // let fetched_referenced_integer_descriptor =
    //     get_property_descriptor_from_record(fetched_referenced_integer_record.unwrap()).unwrap();
    // assert_eq!(
    //     created_shared_integer_type,
    //     fetched_referenced_integer_descriptor
    // );
    //
    // // TESTING SHARED BOOLEAN TYPE //
    //
    // // Iterate referencing types to find TestComposite3
    // let mut composite3_referencing_boolean_descriptor: PropertyDescriptor = referencing_types
    //     .iter()
    //     .find(|descriptor| {
    //         descriptor.header.type_name
    //             == "TestComposite1__Composite_Type__referencing_boolean".to_string()
    //     })
    //     .unwrap()
    //     .clone();
    // // Get the associated map, then upsert the name and usage
    // let mut composite3_map: PropertyDescriptorMap =
    //     get_composite_descriptor_map(&composite3_referencing_boolean_descriptor.details);
    // let composite3_usage = PropertyDescriptorUsage::new(
    //     "testing reference to shared boolean type".to_string(),
    //     created_shared_boolean_type.clone(),
    // );
    // upsert_property_descriptor(
    //     &mut composite3_map,
    //     shared_boolean_name.clone(),
    //     &composite3_usage,
    // );
    // // Update the details
    // composite3_referencing_boolean_descriptor.details =
    //     PropertyDescriptorDetails::Composite(CompositeDescriptor {
    //         properties: composite3_map,
    //     });
    // // println!("{:#?}", composite3_referencing_boolean);
    //
    // // Create the referencing Composite type, then fetch it and compare
    // let created_composite3_record: Record = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "create_property_descriptor",
    //         composite3_referencing_boolean_descriptor.clone(),
    //     )
    //     .await;
    // let fetched_composite3_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         created_composite3_record.action_address().clone(),
    //     )
    //     .await;
    // let fetched_composite3 =
    //     get_property_descriptor_from_record(fetched_composite3_record.unwrap()).unwrap();
    // assert_eq!(
    //     composite3_referencing_boolean_descriptor,
    //     fetched_composite3
    // );
    // // Retrieve the ActionHash of the shared boolean type from the HolonReference, then fetch the property and compare
    // let referenced_boolean_sharing =
    //     get_composite_descriptor_from_details(&fetched_composite3.details)
    //         .properties
    //         .properties
    //         .get(&shared_boolean_name)
    //         .unwrap()
    //         .descriptor
    //         .sharing
    //         .clone();
    // let referenced_boolean_action_hash =
    //     get_holon_reference_from_sharing(&referenced_boolean_sharing)
    //         .id
    //         .unwrap();
    // let fetched_referenced_boolean_record: Option<Record> = conductor
    //     .call(
    //         &cell.zome("descriptors"),
    //         "get_property_descriptor",
    //         referenced_boolean_action_hash,
    //     )
    //     .await;
    // let fetched_referenced_boolean_descriptor =
    //     get_property_descriptor_from_record(fetched_referenced_boolean_record.unwrap()).unwrap();
    // assert_eq!(
    //     created_shared_boolean_type,
    //     fetched_referenced_boolean_descriptor
    // );
}