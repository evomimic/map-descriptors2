import { 
  Record, 
  ActionHash, 
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type DescriptorsSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'PropertyDescriptor'; } & PropertyDescriptor)
 | ({ type: 'HolonDescriptor'; } & HolonDescriptor)
 | ({  type: 'TypeHeader'; } & TypeHeader);


export interface BaseType {
  type:  
    | 'Holon'
        | 'Collection'
        | 'Composite'
        | 'Relationship'
        | 'Boolean'
        | 'Integer'
        | 'String'
        | 'Enum'
    ;
}

export interface TypeHeader { 
  type_name: string;

  base_type: BaseType;

  description: string;

  is_dependent: boolean;
}





export interface HolonDescriptor { 
  name: string;
}





export interface PropertyDescriptor { 
  property_descriptor_placeholder: string;
}


