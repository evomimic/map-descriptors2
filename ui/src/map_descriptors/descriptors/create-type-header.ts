import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-select';
import '@material/mwc-checkbox';
import '@material/mwc-formfield';

import '@material/mwc-textarea';
import { clientContext } from '../../contexts';
import { TypeHeader, BaseType } from './types';

@customElement('create-type-header')
export class CreateTypeHeader extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;


  @state()
  _typeName: string = '';

  @state()
  _baseType: BaseType = { type: 'Holon' };

  @state()
  _description: string = '';

  @state()
  _isDependent: boolean = true;

  
  firstUpdated() {
  }

  isTypeHeaderValid() {
    return true && this._typeName !== '' && true && this._description !== '' && true;
  }

  async createTypeHeader() {
    const typeHeader: TypeHeader = { 
        type_name: this._typeName,
        base_type: this._baseType,
        description: this._description,
        is_dependent: this._isDependent,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'map_descriptors',
        zome_name: 'descriptors',
        fn_name: 'create_type_header',
        payload: typeHeader,
      });

      this.dispatchEvent(new CustomEvent('type-header-created', {
        composed: true,
        bubbles: true,
        detail: {
          typeHeaderHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the type header: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Type Header</span>

          <div style="margin-bottom: 16px">
            <mwc-textarea outlined label="Type Name" .value=${ this._typeName } @input=${(e: CustomEvent) => { this._typeName = (e.target as any).value;} } required></mwc-textarea>          
          </div>
            
          <div style="margin-bottom: 16px">
            <mwc-select outlined helper="Base Type" required>
  <mwc-list-item .selected=${ this._baseType.type === 'Holon' } @request-selected=${() => { this._baseType = { type: 'Holon' }; } }>Holon</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Collection' } @request-selected=${() => { this._baseType = { type: 'Collection' }; } }>Collection</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Composite' } @request-selected=${() => { this._baseType = { type: 'Composite' }; } }>Composite</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Relationship' } @request-selected=${() => { this._baseType = { type: 'Relationship' }; } }>Relationship</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Boolean' } @request-selected=${() => { this._baseType = { type: 'Boolean' }; } }>Boolean</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Integer' } @request-selected=${() => { this._baseType = { type: 'Integer' }; } }>Integer</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'String' } @request-selected=${() => { this._baseType = { type: 'String' }; } }>String</mwc-list-item>
  <mwc-list-item .selected=${ this._baseType.type === 'Enum' } @request-selected=${() => { this._baseType = { type: 'Enum' }; } }>Enum</mwc-list-item>
</mwc-select>          
          </div>
            
          <div style="margin-bottom: 16px">
            <mwc-textarea outlined label="Description" .value=${ this._description } @input=${(e: CustomEvent) => { this._description = (e.target as any).value;} } required></mwc-textarea>          
          </div>
            
          <div style="margin-bottom: 16px">
            <mwc-formfield label="Is Dependent">
              <mwc-checkbox .checked=${ this._isDependent } @change=${(e: CustomEvent) => { this._isDependent = (e.target as any).checked;} }></mwc-checkbox>
            </mwc-formfield>          
          </div>
            

        <mwc-button 
          raised
          label="Create Type Header"
          .disabled=${!this.isTypeHeaderValid()}
          @click=${() => this.createTypeHeader()}
        ></mwc-button>
    </div>`;
  }
}
