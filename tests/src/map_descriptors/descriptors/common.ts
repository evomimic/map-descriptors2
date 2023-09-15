import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';


interface Dictionary<T> {
    [Key: string]: T;
}

//{type: 'Holon'}
enum BaseType {
    Holon = "Holon",
    Collection = "Collection",
    Composite = "Composite",
    Relationship = "Relationship",
    Boolean = "Boolean",
    Integer = "Integer",
    String = "String",
    Enum = "Enum",
}

interface SemanticVersion {
    major: number,
    minor: number,
    patch: number,
}

type BaseTypeObject = { type: string}

export interface TypeHeader {
    type_name: String,
    base_type: BaseTypeObject,
    description: String,
    version: SemanticVersion,
    is_dependent: boolean,
}

type BooleanDescriptor = {
    is_fuzzy: boolean, // if true, this property has FuzzyBoolean value, otherwise just true or false
}

type CompositeDescriptor = {
    properties: PropertyDescriptorMap,
}

export enum IntegerFormat {
    I8 = "i8",
    I16 = "i16",
    I32 = "i32",
    I64 = "i64",
    // I128(),
    U8 = "u8",
    U16 = "u16",
    U32 = "u32",
    U64 = "u64",
    // U128(),
}

//type IntegerObjectFormat = {["IntegerFormat"]:IntegerFormat.I32}

type IntegerDescriptor = {
    format: IntegerFormat,
    min_value: number,
    max_value: number,
}

type StringDescriptor = {
    min_length: number,
    max_length: number,
    //pattern: String,
}

type ValueCollectionDescriptor = {
    contains_items_of_type: String, // TODO: replace this with a ref
    min_items: number,
    max_items: number,
    unique_items: boolean, // true means duplicate items are not allowed
    is_ordered: boolean,   // if items have an intrinsic order
}


type PropertyDescriptorDetails =
    | { ["boolean"]: BooleanDescriptor}
    | { ["composite"]: CompositeDescriptor}
    | { ["integer"]: IntegerDescriptor}
    | { ["string"]: StringDescriptor}
    | { ["valuecollection"]: ValueCollectionDescriptor}

//enum PropertyDescriptorDetails {
  //  BooleanDescriptor,
  //  CompositeDescriptor,
    //Enum(EnumDescriptor),
  //  IntegerDescriptor,// = {type: { IntegerDescriptor }},
   // StringDescriptor,
    // ValueCollection(ValueCollectionDescriptor), // can only contain collections of PropertyTypes (not Holons)
//}

interface PropertyDescriptor {
    header: TypeHeader,
    details: PropertyDescriptorDetails,
}

interface PropertyDescriptorUsage {
    description: String,
    descriptor: PropertyDescriptor,
}

interface PropertyDescriptorMap {
    properties: Dictionary<PropertyDescriptorUsage>,
}


export interface HolonDescriptor {
    header: TypeHeader,
    properties: PropertyDescriptorMap,
}





/*new_integer_descriptor : PropertyDescriptor (
    type_name: String,
    description: String,
    is_dependent: bool,
    format: IntegerFormat,
    min_value: i64,
    max_value: i64,
) {*/
    //const details: IntegerDescriptor = {
    //    format: ,
    //    min_value: 0,
    //    max_value: 100 //PropertyDescriptorDetails.IntegerDescriptor,
        
   // }
        
        /*= IntegerDescriptor(format, min_value, max_value));
    let desc = new_property_descriptor(
        type_name,
        description,
        BaseType::Integer,
        is_dependent,
        details,
    )?;
    Ok(desc)
}*/


function new_type_header(_type_name:string, _base_type: BaseTypeObject, _description:string, _version: SemanticVersion, _is_dependent:boolean): TypeHeader {
    return { type_name:_type_name, base_type:_base_type, description: _description, version:_version, is_dependent:_is_dependent}
}

// Creates an empty holon descriptor.
///
//function new_holon_descriptor( type_name: String, description: String, is_dependent: boolean): HolonDescriptor {
  //  let header = new_type_header(type_name, JSON.parse(BaseType.Holon), description, is_dependent)?

 //   let descriptor = HolonDescriptor::new(header, PropertyDescriptorMap::new(BTreeMap::new()));

//    Ok(descriptor)/
//}



export function samplePropertyDescriptorMap( partialPropertyDescriptor = {}): PropertyDescriptorMap {
	const typeheader: TypeHeader = { 
        type_name: "sub_holon_prop",
        base_type: { type: BaseType.String},
        description: "holon title",
        version: { major: 0, minor: 0, patch: 0},
        is_dependent: false
    }
    //const propertyDescriptorDetails : PropertyDescriptorDetails = {

    const propertyDescriptor: PropertyDescriptor = {
            header: typeheader,
            details: {["integer"]: { format:(IntegerFormat.I32), min_value: 0, max_value: 100 } }
    // {"Y":0} {"Integer": {}}
    }
    const propertyDescriptorUsage: PropertyDescriptorUsage = {
        description: "shared",
        descriptor: propertyDescriptor
    }
	return {
			properties: {["my_usage1"]: propertyDescriptorUsage
            },
			...partialPropertyDescriptor
	};
}


//allows override of any property
export function sampleHolonDescriptor( partialHolonDescriptor = {}): HolonDescriptor {
    const typeheader: TypeHeader = { 
        type_name: "holon_desc1",
        base_type: {type: BaseType.Holon},
        description: "holon descriptor 1",
        version: { major: 0, minor: 0, patch: 0},
        is_dependent: false
    }
	const propertydescriptormap = samplePropertyDescriptorMap()
  return {
          header: typeheader,
          properties : propertydescriptormap,
        	...partialHolonDescriptor
  };
}

export async function createHolonDescriptor(cell: CallableCell, holonDescriptor = undefined): Promise<Record> {
    console.log("sample data: ",sampleHolonDescriptor())
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_holon_descriptor",
      payload: holonDescriptor || sampleHolonDescriptor(),
    });
}



//-----------  ??

/*export async function createTypeHeader(cell: CallableCell, typeHeader = undefined): Promise<Record> {
	return cell.callZome({
		zome_name: "descriptors",
		fn_name: "create_type_header",
		payload: typeHeader || await sampleTypeHeader(cell),
	});
}*/


export async function createPropertyDescriptor(cell: CallableCell, propertyDescriptor = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_property_descriptor",
      payload: propertyDescriptor || await samplePropertyDescriptorMap(cell),
    });
}

function new_integer_descriptor(type_name: any, String: StringConstructor, description: any, String1: StringConstructor, is_dependent: any, bool: any, format: any, IntegerFormat: any, min_value: any, i64: any, max_value: any, i641: any) {
    throw new Error('Function not implemented.');
}


