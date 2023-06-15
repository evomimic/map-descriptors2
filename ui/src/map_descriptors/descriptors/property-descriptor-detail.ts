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

import './edit-property-descriptor';

import { clientContext } from '../../contexts';
import { PropertyDescriptor } from './types';

@customElement('property-descriptor-detail')
export class PropertyDescriptorDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  propertyDescriptorHash!: ActionHash;

  _fetchRecord = new Task(this, ([propertyDescriptorHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'map_descriptors',
      zome_name: 'descriptors',
      fn_name: 'get_property_descriptor',
      payload: propertyDescriptorHash,
  }) as Promise<Record | undefined>, () => [this.propertyDescriptorHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.propertyDescriptorHash === undefined) {
      throw new Error(`The propertyDescriptorHash property is required for the property-descriptor-detail element`);
    }
  }

  async deletePropertyDescriptor() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'delete_property_descriptor',
        payload: this.propertyDescriptorHash,
      });
      this.dispatchEvent(new CustomEvent('property-descriptor-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          propertyDescriptorHash: this.propertyDescriptorHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the property descriptor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const propertyDescriptor = decode((record.entry as any).Present.entry) as PropertyDescriptor;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deletePropertyDescriptor()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderPropertyDescriptor(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested property descriptor was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-property-descriptor
    	  .originalPropertyDescriptorHash=${this.propertyDescriptorHash}
    	  .currentRecord=${maybeRecord}
    	  @property-descriptor-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-property-descriptor>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderPropertyDescriptor(maybeRecord),
      error: (e: any) => html`<span>Error fetching the property descriptor: ${e.data.data}</span>`
    });
  }
}
