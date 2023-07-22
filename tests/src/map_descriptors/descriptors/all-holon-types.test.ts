import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createHolonDescriptor } from './common.js';

test('create a HolonDescriptor and get all holon types', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/map-descriptors2.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all holon types
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "descriptors",
      fn_name: "get_all_holon_types",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);
    console.log("Result:")
    console.log(collectionOutput)

    // Alice creates a HolonDescriptor
    console.log("Alice creates a new holon descriptor")
    const createdRecord: Record = await createHolonDescriptor(alice.cells[0]);
    assert.ok(createdRecord);
    
    await pause(1200);
    
    // Bob gets all holon types again
    console.log("Result:")
    console.log(collectionOutput)


    // Bob gets all holon types again
    console.log("Bobby gets all holon types again")
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "descriptors",
      fn_name: "get_all_holon_types",
      payload: null
    });
    console.log("Result:")
    console.log(collectionOutput)
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);    
  });
});

