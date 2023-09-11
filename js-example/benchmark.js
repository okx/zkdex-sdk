const Benchmark = require('benchmark');
const wasm = require('@okbchain/zkdex-sdk')
const assert = require("assert");
var suite = new Benchmark.Suite();

let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
let sig_r = "0x094a47cb182c7eb24e3c34a473def9d356bb30161179e4bbaeaa48c6d18844f8";
let sig_s = "0x05534d29f2f1d3ba474f7cec4f9f545924924e5f4261577d09ed9a85df252d5d";
let pub_key_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
let pub_key_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";

let pri_key = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273";
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