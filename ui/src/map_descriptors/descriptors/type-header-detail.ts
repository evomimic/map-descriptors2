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


import { clientContext } from '../../contexts';
import { TypeHeader, BaseType } from './types';

@customElement('type-header-detail')
export class TypeHeaderDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  typeHeaderHash!: ActionHash;

  _fetchRecord = new Task(this, ([typeHeaderHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'map_descriptors',
      zome_name: 'descriptors',
      fn_name: 'get_type_header',
      payload: typeHeaderHash,
  }) as Promise<Record | undefined>, () => [this.typeHeaderHash]);

  
  firstUpdated() {
    if (this.typeHeaderHash === undefined) {
      throw new Error(`The typeHeaderHash property is required for the type-header-detail element`);
    }
  }


  renderDetail(record: Record) {
    const typeHeader = decode((record.entry as any).Present.entry) as TypeHeader;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span style="margin-right: 4px"><strong>Type Name: </strong></span>
 	  <span style="white-space: pre-line">${ typeHeader.type_name }</span>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span style="margin-right: 4px"><strong>Base Type: </strong></span>
 	  <span style="white-space: pre-line">${  typeHeader.base_type.type === 'Holon' ? `Holon` :  typeHeader.base_type.type === 'Collection' ? `Collection` :  typeHeader.base_type.type === 'Composite' ? `Composite` :  typeHeader.base_type.type === 'Relationship' ? `Relationship` :  typeHeader.base_type.type === 'Boolean' ? `Boolean` :  typeHeader.base_type.type === 'Integer' ? `Integer` :  typeHeader.base_type.type === 'String' ? `String` :  `Enum`  }</span>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span style="margin-right: 4px"><strong>Description: </strong></span>
 	  <span style="white-space: pre-line">${ typeHeader.description }</span>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span style="margin-right: 4px"><strong>Is Dependent: </strong></span>
 	  <span style="white-space: pre-line">${ typeHeader.is_dependent ? 'Yes' : 'No' }</span>
        </div>

      </div>
    `;
  }
  
  renderTypeHeader(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested type header was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderTypeHeader(maybeRecord),
      error: (e: any) => html`<span>Error fetching the type header: ${e.data.data}</span>`
    });
  }
}
