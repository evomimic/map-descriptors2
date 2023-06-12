import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleTypeHeader(cell: CallableCell, partialTypeHeader = {}) {
    return {
        ...{
	  type_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  base_type: { type: 'Holon' },
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  is_dependent: true,
        },
        ...partialTypeHeader
    };
}

export async function createTypeHeader(cell: CallableCell, typeHeader = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_type_header",
      payload: typeHeader || await sampleTypeHeader(cell),
    });
}



export async function sampleHolonDescriptor(cell: CallableCell, partialHolonDescriptor = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialHolonDescriptor
    };
}

export async function createHolonDescriptor(cell: CallableCell, holonDescriptor = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "descriptors",
      fn_name: "create_holon_descriptor",
      payload: holonDescriptor || await sampleHolonDescriptor(cell),
    });
}

