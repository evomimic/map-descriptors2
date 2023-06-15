import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { PropertyDescriptor } from './types';

@customElement('edit-property-descriptor')
export class EditPropertyDescriptor extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalPropertyDescriptorHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentPropertyDescriptor() {
    return decode((this.currentRecord.entry as any).Present.entry) as PropertyDescriptor;
  }
 

  isPropertyDescriptorValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-property-descriptor element`);
    }

    if (this.originalPropertyDescriptorHash === undefined) {
      throw new Error(`The originalPropertyDescriptorHash property is required for the edit-property-descriptor element`);
    }
    
  }

  async updatePropertyDescriptor() {
    const propertyDescriptor: PropertyDescriptor = { 
      property_descriptor_placeholder: this.currentPropertyDescriptor.property_descriptor_placeholder,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'update_property_descriptor',
        payload: {
          original_property_descriptor_hash: this.originalPropertyDescriptorHash,
          previous_property_descriptor_hash: this.currentRecord.signed_action.hashed.hash,
          updated_property_descriptor: propertyDescriptor
        },
      });
  
      this.dispatchEvent(new CustomEvent('property-descriptor-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalPropertyDescriptorHash: this.originalPropertyDescriptorHash,
          previousPropertyDescriptorHash: this.currentRecord.signed_action.hashed.hash,
          updatedPropertyDescriptorHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the property descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Property Descriptor</span>


        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isPropertyDescriptorValid()}
            @click=${() => this.updatePropertyDescriptor()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
