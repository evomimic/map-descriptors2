import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { PropertyDescriptor } from './types';

@customElement('create-value-descriptor')
export class CreatePropertyDescriptor extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  propertyDescriptorPlaceholder!: string;

  
  firstUpdated() {
    if (this.propertyDescriptorPlaceholder === undefined) {
      throw new Error(`The propertyDescriptorPlaceholder input is required for the create-value-descriptor element`);
    }
  }

  isPropertyDescriptorValid() {
    return true;
  }

  async createPropertyDescriptor() {
    const propertyDescriptor: PropertyDescriptor = { 
        property_descriptor_placeholder: this.propertyDescriptorPlaceholder,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'create_property_descriptor',
        payload: propertyDescriptor,
      });

      this.dispatchEvent(new CustomEvent('value-descriptor-created', {
        composed: true,
        bubbles: true,
        detail: {
          propertyDescriptorHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the property descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Property Descriptor</span>


        <mwc-button 
          raised
          label="Create Property Descriptor"
          .disabled=${!this.isPropertyDescriptorValid()}
          @click=${() => this.createPropertyDescriptor()}
        ></mwc-button>
    </div>`;
  }
}
