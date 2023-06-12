import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { HolonDescriptor } from './types';

@customElement('edit-holon-descriptor')
export class EditHolonDescriptor extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalHolonDescriptorHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentHolonDescriptor() {
    return decode((this.currentRecord.entry as any).Present.entry) as HolonDescriptor;
  }
 

  isHolonDescriptorValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-holon-descriptor element`);
    }

    if (this.originalHolonDescriptorHash === undefined) {
      throw new Error(`The originalHolonDescriptorHash property is required for the edit-holon-descriptor element`);
    }
    
  }

  async updateHolonDescriptor() {
    const holonDescriptor: HolonDescriptor = { 
      name: this.currentHolonDescriptor.name,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'update_holon_descriptor',
        payload: {
          original_holon_descriptor_hash: this.originalHolonDescriptorHash,
          previous_holon_descriptor_hash: this.currentRecord.signed_action.hashed.hash,
          updated_holon_descriptor: holonDescriptor
        },
      });
  
      this.dispatchEvent(new CustomEvent('holon-descriptor-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalHolonDescriptorHash: this.originalHolonDescriptorHash,
          previousHolonDescriptorHash: this.currentRecord.signed_action.hashed.hash,
          updatedHolonDescriptorHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the holon descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Holon Descriptor</span>


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
            .disabled=${!this.isHolonDescriptorValid()}
            @click=${() => this.updateHolonDescriptor()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
