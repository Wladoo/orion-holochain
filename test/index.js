const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/orion-holochain.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])


// todo
scenario.runTape("create broker", async (t, { alice }) => {
    const addr = alice.call("broker", "create_broker", {"name": "broker1"})
    t.deepEqual(result, { Ok: { App: ["broker", '{"name": "broker1"}' ] } })
})
