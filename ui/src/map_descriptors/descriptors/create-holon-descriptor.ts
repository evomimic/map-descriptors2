import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { HolonDescriptor } from './types';

@customElement('create-holon-descriptor')
export class CreateHolonDescriptor extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  name!: string;

  
  firstUpdated() {
    if (this.name === undefined) {
      throw new Error(`The name input is required for the create-holon-descriptor element`);
    }
  }

  isHolonDescriptorValid() {
    return true;
  }

  async createHolonDescriptor() {
    const holonDescriptor: HolonDescriptor = { 
        name: this.name,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'create_holon_descriptor',
        payload: holonDescriptor,
      });

      this.dispatchEvent(new CustomEvent('holon-descriptor-created', {
        composed: true,
        bubbles: true,
        detail: {
          holonDescriptorHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the holon descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Holon Descriptor</span>


        <mwc-button 
          raised
          label="Create Holon Descriptor"
          .disabled=${!this.isHolonDescriptorValid()}
          @click=${() => this.createHolonDescriptor()}
        ></mwc-button>
    </div>`;
  }
}
