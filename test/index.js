const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/orion-holochain.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])


//todo
scenario.runTape("create broker", async (t, { alice }) => {
    let ord1 = {"name": "broker1"};
    const res1 = alice.call("orion_project1", "create_broker", ord1);

    t.deepEqual(res1, {Ok: "QmYfmS8M4EZRkpkNqyAtCgPaMjguAnbwupvRfpASYo4j9k"}, "address of 'create broker' is correct");
    t.ok(res1, res1.Ok !== undefined && res1.Err === undefined);
});

scenario.runTape("create order", async (t, { alice }) => {
    const res1 = alice.call("orion_project1", "initialize_order", {
      "base_asset_code": "EUR",
      "quoted_asset_code": "GBP",
      "direction": "Buy",
      "quoted_price_per_unit": 33.5,
      "amount": 44.6
    });

    // var {Ok: addr1} = res1;
    // t.deepEqual(res1.Ok, addr1);
    t.ok(res1.Ok !== undefined);
    t.ok(res1.Err === undefined);
    // t.ok(res1.Err, '1234');
    t.deepEqual(res1, {Ok: "QmemoDUBtg7wAoHA6CWniGDNsfxKhaaREdUzLFyvKC12ZH"}, "address of 'create order' is correct");

    //todo: for debugging
    // t.equal(res1, "aabbcc"); 




    //todo
});

scenario.runTape("create trade", async (t, { alice }) => {

});

scenario.runTape("add balance", async (t, { alice }) => {

});