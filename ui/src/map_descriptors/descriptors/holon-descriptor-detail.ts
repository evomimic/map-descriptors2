import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import './edit-holon-descriptor';

import { clientContext } from '../../contexts';
import { HolonDescriptor } from './types';

@customElement('holon-descriptor-detail')
export class HolonDescriptorDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  holonDescriptorHash!: ActionHash;

  _fetchRecord = new Task(this, ([holonDescriptorHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'map_descriptors',
      zome_name: 'descriptors',
      fn_name: 'get_holon_descriptor',
      payload: holonDescriptorHash,
  }) as Promise<Record | undefined>, () => [this.holonDescriptorHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.holonDescriptorHash === undefined) {
      throw new Error(`The holonDescriptorHash property is required for the holon-descriptor-detail element`);
    }
  }

  async deleteHolonDescriptor() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'delete_holon_descriptor',
        payload: this.holonDescriptorHash,
      });
      this.dispatchEvent(new CustomEvent('holon-descriptor-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          holonDescriptorHash: this.holonDescriptorHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the holon descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const holonDescriptor = decode((record.entry as any).Present.entry) as HolonDescriptor;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteHolonDescriptor()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderHolonDescriptor(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested holon descriptor was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-holon-descriptor
    	  .originalHolonDescriptorHash=${this.holonDescriptorHash}
    	  .currentRecord=${maybeRecord}
    	  @holon-descriptor-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-holon-descriptor>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderHolonDescriptor(maybeRecord),
      error: (e: any) => html`<span>Error fetching the holon descriptor: ${e.data.data}</span>`
    });
  }
}
