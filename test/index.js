const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/orion-holochain.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])


//todo
scenario.runTape("create broker", async (t, { alice }) => {
    const res1 = alice.call("orion_project1", "create_broker", {"name": "broker1"});
    var {Ok: addr} = res1;
    t.deepEqual(res1.Ok, addr);

    //todo
    t.deepEqual(res1.Ok, undefined);
    t.equal(res1.Ok, undefined);

    //todo
    // t.ok(res1.Ok !== undefined && res1.Err === undefined);
    // t.ok(res1.Ok !== undefined);
    t.deepEqual(res1.Ok, undefined);
    t.deepEqual(res1.Okdfsfds, undefined);

    //todo: error --> Internal: 'Argument deserialization failed'
    t.deepEqual(res1.Err, undefined);
});

scenario.runTape("create order", async (t, { alice }) => {
    //todo: why will it pass?
    const res1 = alice.call("orion_project1", "initialize_order", {
      "base_asset_code": "EUR",
      "quoted_asset_code": "GBP",
      "direction": {"Buy": {}},
      "quoted_price_per_unit": 33.5,
      "amount": 44.6
    });

    var {Ok: addr1} = res1;
    t.deepEqual(res1.Ok, addr1);
    // t.equal(addr1, 1234);
    t.ok(res1.Ok != undefined);
    t.ok(res1.Err == undefined);
    t.ok(res1.Err, '1234');

    //todo: for debugging
    t.equal(addr1, "aabbcc"); 


    // todo
    // const res2 = alice.call("orion_project1", "get_order", {"addr": addr1});
    // t.equal(res2.Err, undefined);
    // t.notEqual(res2.Ok, undefined);


    // //todo
    // //check that 'status' is 'new'
    // var {Ok: ord2} = res2;
    // // t.equal(ord2.status, "fsafd");
    // t.equal(ord2, "test2");

    // const res3 = alice.call("orion_project1", "approve_order", {"addr": addr});
    // t.equal(res3.Err, undefined);

    //todo
    //check that 'status' is 'approved'
});

scenario.runTape("create trade", async (t, { alice }) => {

});

scenario.runTape("add balance", async (t, { alice }) => {

});