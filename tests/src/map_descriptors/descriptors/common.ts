import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';


interface Dictionary<T> {
    [Key: string]: T;
}

enum BaseType {
    Holon,
    Collection,
    Composite,
    Relationship,
    Boolean,
    Integer,
    String,
    Enum,
}

interface SemanticVersion {
    major: number,
    minor: number,
    patch: number,
}

export interface TypeHeader {
    type_name: String,
    base_type: BaseType,
    description: String,
    version: SemanticVersion,
    is_dependent: boolean,
}

type BooleanDescriptorDetail = {
    is_fuzzy: boolean, // if true, this property has FuzzyBoolean value, otherwise just true or false
}

interface CompositeDescriptor {
    properties: PropertyDescriptorMap,
}

interface IntegerDescriptor {
   // format: IntegerFormat,
    min_value: number,
    max_value: number,
}

enum PropertyDescriptorDetails {
		BooleanDescriptor,
    CompositeDescriptor,
    //Enum(EnumDescriptor),
   IntegerDescriptor,
    StringDescriptor,
   // ValueCollection(ValueCollectionDescriptor), // can only contain collections of PropertyTypes (not Holons)
}

interface PropertyDescriptor {
    header: TypeHeader,
    details: PropertyDescriptorDetails,
}

interface PropertyDescriptorMap {
    properties: Dictionary<PropertyDescriptor>,
}


export interface HolonDescriptor {
    header: TypeHeader,
    properties: PropertyDescriptorMap,
}

export async function sampleTypeHeader(cell: CallableCell, partialTypeHeader = {}) {
	return {
          ...{
							type_name: "holon_desc1",
							base_type: { type: 'Holon' },
							description: "holon descriptor 1",
							version: { major: 0, minor: 0, patch: 0},
							is_dependent: false,
					},
					...partialTypeHeader 
	};
}

export async function samplePropertyDescriptorMap(cell: CallableCell, partialPropertyDescriptor = {}) {
	const typeheader = await sampleTypeHeader(cell)
	return {
			...{
				properties: {["descriptor1"]: {header: {...typeheader}, details:{boolean:{isFuzzy: false}}}}
			},
			...partialPropertyDescriptor
	};
}



export async function sampleHolonDescriptor(cell: CallableCell, partialHolonDescriptor = {}) {
  const typeheader = await sampleTypeHeader(cell)
	const propertydescriptormap = await samplePropertyDescriptorMap(cell)
  return {
          header: { ...typeheader},
          properties : { ...propertydescriptormap},
        	...partialHolonDescriptor
  };
}

export async function createHolonDescriptor(cell: CallableCell, holonDescriptor = undefined): Promise<Record> {
    console.log("sample data: ",sampleHolonDescriptor(cell))
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_holon_descriptor",
      payload: holonDescriptor || await sampleHolonDescriptor(cell),
    });
}



//-----------  ??

export async function createTypeHeader(cell: CallableCell, typeHeader = undefined): Promise<Record> {
	return cell.callZome({
		zome_name: "descriptors",
		fn_name: "create_type_header",
		payload: typeHeader || await sampleTypeHeader(cell),
	});
}


export async function createPropertyDescriptor(cell: CallableCell, propertyDescriptor = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_property_descriptor",
      payload: propertyDescriptor || await samplePropertyDescriptorMap(cell),
    });
}