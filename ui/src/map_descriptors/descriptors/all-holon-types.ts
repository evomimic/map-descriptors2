import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { AppAgentClient, AgentPubKey, EntryHash, ActionHash, Record, NewEntryAction } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import '@material/mwc-circular-progress';

import { clientContext } from '../../contexts';
import { DescriptorsSignal } from './types';

import './holon-descriptor-detail';

@customElement('all-holon-types')
export class AllHolonTypes extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @state()
  signaledHashes: Array<ActionHash> = [];
  
  _fetchHolonDescriptors = new Task(this, ([]) => this.client.callZome({
      cap_secret: null,
      role_name: 'map_descriptors',
      zome_name: 'descriptors',
      fn_name: 'get_all_holon_types',
      payload: null,
  }) as Promise<Array<Record>>, () => []);

  firstUpdated() {
    this.client.on('signal', signal => {
      if (signal.zome_name !== 'descriptors') return; 
      const payload = signal.payload as DescriptorsSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'HolonDescriptor') return;
      this.signaledHashes = [payload.action.hashed.hash, ...this.signaledHashes];
    });
  }
  
  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No holon descriptors found.</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash => 
          html`<holon-descriptor-detail .holonDescriptorHash=${hash} style="margin-bottom: 16px;" @holon-descriptor-deleted=${() => { this._fetchHolonDescriptors.run(); this.signaledHashes = []; } }></holon-descriptor-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchHolonDescriptors.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (records) => this.renderList([...this.signaledHashes, ...records.map(r => r.signed_action.hashed.hash)]),
      error: (e: any) => html`<span>Error fetching the holon descriptors: ${e.data.data}.</span>`
    });
  }
}
