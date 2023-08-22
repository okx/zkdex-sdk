const assert = require("assert");
const wasm = require('@okbchain/zkdex-sdk');
const utils = require('ethers');

const pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
const pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
const err_hash = "0acf01cf2a0f6b5fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725";

describe('test zkdex js function', function () {
    it('test sign withdraw', function () {
        let withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":2,\"amount\":3,\"eth_address\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\"}";
        let sig_str = wasm.sign_withdraw(withdraw_req, "1", pri_key);
        let hash = wasm.hash_withdraw(withdraw_req, "1");
        console.log(hash);
        let sig = JSON.parse(sig_str)
        console.log(sig)
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, err_hash), false);
    });

    it('test sgin trasnfer', function () {
        let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}"
        let sig_str = wasm.sign_transfer(transfer_req, pri_key);
        console.log(sig_str);
        let hash = wasm.hash_transfer(transfer_req);
        let sig = JSON.parse(sig_str);
        console.log("hash:", hash);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, err_hash), false);
    });

    it('test sign limit order', function () {
        let limit_order_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":3,\"amount_collateral\":4,\"amount_fee\":5,\"asset_id_synthetic\":6,\"asset_id_collateral\":\"7\",\"position_id\":8,\"is_buying_synthetic\":false}"
        let sig_str = wasm.sign_limit_order(limit_order_req, pri_key);
        console.log(sig_str);
        let hash = wasm.hash_limit_order(limit_order_req);
        let sig = JSON.parse(sig_str);
        console.log(sig);
        console.log(hash);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, err_hash), false);
    });

    it('test sign liquide', function () {
        let liquide_req = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":1,\"amount_collateral\":2,\"amount_fee\":3,\"asset_id_synthetic\":4,\"asset_id_collateral\":\"0x5\",\"position_id\":6,\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}"
        let sig_str = wasm.sign_liquidate(liquide_req, pri_key);
        console.log(sig_str);
        let hash = wasm.hash_liquidate(liquide_req);
        let sig = JSON.parse(sig_str);
        console.log(sig);
        console.log(hash);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, err_hash), false);
    });

    it('test sign signed oracle price', function () {
        let oracle_price_req = "{\"signer_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":1,\"timestamp\":2,\"signed_asset_id\":\"0x3\"}"
        let sig_str = wasm.sign_signed_oracle_price(oracle_price_req, pri_key);
        console.log(sig_str);
        let hash = wasm.hash_signed_oracle_price(oracle_price_req);
        let sig = JSON.parse(sig_str);
        console.log(sig);
        console.log(hash);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key, err_hash), false);
    });

    it('test l1_sign', function () {
        let hash = "1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3";
        let ret = wasm.l1_sign(hash, pri_key);
        let o = JSON.parse(ret);
        console.log(o);
        let expected = {
            x: "0x02c5c5ab6dc2ae39c6bf239acd233c412ceebba1370cd4679ff78c3e57a33f90",
            y: "0x1fc29405cb5021e77aec60bfdd9ed43b245569e4cfc6e5720207e015662fd3b9",
            s: "0x03fcedddaa3803bc26fa98926d224f13857c1b600a3e99ba01cfcee8d54deaa3",
            pk_x: "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b",
            pk_y: "0x2917e2b130d3c0b999870048591eff578da75c0b5fb1c4c5c99a7fd9cbd3cb42"
        }
        assert.equals(o, expected);

    });
})




