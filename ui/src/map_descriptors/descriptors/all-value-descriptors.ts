import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { AppAgentClient, AgentPubKey, EntryHash, ActionHash, Record, NewEntryAction } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import '@material/mwc-circular-progress';

import { clientContext } from '../../contexts';
import { DescriptorsSignal } from './types';

import './value-descriptor-detail';

@customElement('all-value-descriptors')
export class AllPropertyDescriptors extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @state()
  signaledHashes: Array<ActionHash> = [];
  
  _fetchPropertyDescriptors = new Task(this, ([]) => this.client.callZome({
      cap_secret: null,
      role_name: 'map_descriptors',
      zome_name: 'descriptors',
      fn_name: 'get_all_property_descriptors',
      payload: null,
  }) as Promise<Array<Record>>, () => []);

  firstUpdated() {
    this.client.on('signal', signal => {
      if (signal.zome_name !== 'descriptors') return; 
      const payload = signal.payload as DescriptorsSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'PropertyDescriptor') return;
      this.signaledHashes = [payload.action.hashed.hash, ...this.signaledHashes];
    });
  }
  
  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No property descriptors found.</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash => 
          html`<value-descriptor-detail .propertyDescriptorHash=${hash} style="margin-bottom: 16px;" @value-descriptor-deleted=${() => { this._fetchPropertyDescriptors.run(); this.signaledHashes = []; } }></value-descriptor-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchPropertyDescriptors.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (records) => this.renderList([...this.signaledHashes, ...records.map(r => r.signed_action.hashed.hash)]),
      error: (e: any) => html`<span>Error fetching the property descriptors: ${e.data.data}.</span>`
    });
  }
}
