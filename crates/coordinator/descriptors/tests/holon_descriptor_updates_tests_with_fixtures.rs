// //! Holon Descriptor Updates Test Cases

// #![allow(unused_imports)]

// use core::panic;
// use std::collections::BTreeMap;
// mod shared_test;

// use hdk::prelude::*;
// use holochain::sweettest::{SweetCell, SweetConductor};

// use descriptors::helpers::get_holon_descriptor_from_record;
// use descriptors::holon_descriptor_storage_fns::UpdateHolonDescriptorInput;
// use descriptors::mutators::{
//     new_boolean_descriptor, new_composite_descriptor, new_integer_descriptor, new_string_descriptor,
// };
// use descriptors::property_map_builder::{insert_property_descriptor, remove_property_descriptor};
// use rstest::*;
// use shared_test::fixture_defs::{
//     add_string_property, add_string_property_to_composite, derive_type_name,
//     remove_string_property, remove_string_property_from_composite, update_each_scalar_details,
// };
// use shared_test::UpdateData;
// use shared_types_descriptor::error::DescriptorsError;
// use shared_types_descriptor::holon_descriptor::HolonDescriptor;
// use shared_types_descriptor::property_descriptor::{
//     CompositeDescriptor, IntegerFormat, PropertyDescriptor, PropertyDescriptorDetails,
//     PropertyDescriptorMap,
// };

// #[rstest]
// #[case::add_string_property_to_holon_descriptor(add_string_property())]
// #[case::remove_string_property_from_holon_descriptor(remove_string_property(add_string_property()))]
// #[case::update_each_scalar_property_details(update_each_scalar_details())]
// #[case::add_property_to_composite_descriptor(add_string_property_to_composite())]
// #[case::remove_property_to_composite_descriptor(remove_string_property_from_composite(
//     add_string_property_to_composite()
// ))]
// #[tokio::test(flavor = "multi_thread")]
// async fn rstest_holon_descriptor_update(#[case] input: Result<UpdateData, DescriptorsError>) {
//     let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
//         shared_test::setup_conductor().await;

//     let input_values = input.unwrap();
//     let orig_descriptor: HolonDescriptor = input_values.orig;
//     let expected_descriptor: HolonDescriptor = input_values.updates;
//     println!("original: {:#?} \n", orig_descriptor);
//     println!("expected: {:#?} \n", expected_descriptor);
//     println!("*** CREATING HOLON DESCRIPTOR IN MOCK DHT *** \n");

//     let created_record: Record = conductor
//         .call(
//             &cell.zome("descriptors"),
//             "create_holon_descriptor",
//             orig_descriptor.clone(),
//         )
//         .await;
//     let created_descriptor = get_holon_descriptor_from_record(created_record.clone()).unwrap();
//     assert_eq!(orig_descriptor, created_descriptor);
//     println!(
//         "Created descriptor matches generated holon descriptor, fetching created descriptor \n"
//     );
//     let created_action_hash: ActionHash = created_record.action_address().clone();
//     let fetched_record: Option<Record> = conductor
//         .call(
//             &cell.zome("descriptors"),
//             "get_holon_descriptor",
//             created_action_hash.clone(),
//         )
//         .await;
//     let fetched_descriptor = get_holon_descriptor_from_record(fetched_record.unwrap()).unwrap();
//     assert_eq!(orig_descriptor, created_descriptor);
//     assert_eq!(created_descriptor, fetched_descriptor);
//     println!("...Success! Fetched descriptor matches generated descriptor. \n");

//     println!(
//         "******* STARTING TEST CASES FOR UPDATING HOLON DESCRIPTOR *************************** \n"
//     );

//     let update_input = UpdateHolonDescriptorInput {
//         original_holon_descriptor_hash: created_action_hash.clone(),
//         previous_holon_descriptor_hash: created_action_hash.clone(),
//         updated_holon_descriptor: expected_descriptor.clone(),
//     };
//     let updated_record: Record = conductor
//         .call(
//             &cell.zome("descriptors"),
//             "update_holon_descriptor",
//             update_input,
//         )
//         .await;
//     let updated_entry = get_holon_descriptor_from_record(updated_record.clone()).unwrap();
//     let update_action_hash: ActionHash = updated_record.action_address().clone();
//     assert_eq!(expected_descriptor, updated_entry);
//     let fetched_updated_record: Option<Record> = conductor
//         .call(
//             &cell.zome("descriptors"),
//             "get_holon_descriptor",
//             update_action_hash.clone(),
//         )
//         .await;
//     let fetched_updated_entry =
//         get_holon_descriptor_from_record(fetched_updated_record.unwrap()).unwrap();
//     assert_eq!(updated_entry, fetched_updated_entry);
// }
