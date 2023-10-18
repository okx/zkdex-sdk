const Benchmark = require('benchmark');
const wasm = require('@okbchain/zkdex-sdk')
const assert = require("assert");
var suite = new Benchmark.Suite();

let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
let sig_r = "0x1c929aba1dd2f9cacf5c857e014b2ea1bbd98e5758821a20293b12c869e51732";
let sig_s = "0x03d739463c57a40e49b8e52f54c18acce5f205ee9ffcee2b96ac83bc3fbcf476";
let pub_key_x = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
let pub_key_y = "0x0a3b966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";

let pri_key = "0x01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
suite.add('test sign transfer', function() {
    let sig_str = wasm.sign_transfer(json, pri_key);
})
.add('test verify transfer', function() {
    let hash = wasm.hash_transfer(json);
    assert.equal(wasm.verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, hash), true);
})
// add listeners
.on('cycle', function(event) {
    console.log(String(event.target));
})
.on('complete', function() {
})
// run async
.run({ 'async': true });
