const Benchmark = require('benchmark');
const wasm = require('@okbchain/zkdex-sdk')
const assert = require("assert");
var suite = new Benchmark.Suite();
suite.add('test sign trannsfer', function() {
    let pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let transfer_req  = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
    let sig_str = wasm.sign_transfer(transfer_req, pri_key);
})
.add('test verify transfer', function() {
    let transfer_req  = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}"
    let sig_r = "0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099";
    let sig_s ="b7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05";
    let pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    let hash = wasm.hash_transfer(transfer_req);
    assert.equal(wasm.verify_signature(sig_r, sig_s, pub_key, hash), true);
})
// add listeners
.on('cycle', function(event) {
    console.log(String(event.target));
})
.on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'));
})
// run async
.run({ 'async': true });