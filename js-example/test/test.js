const assert = require("assert");
const wasm = require('@okbchain/zkdex-sdk');
const utils = require('ethers');

const pri_key = "0x05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
const pub_key_x = "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
const pub_key_y = "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
const err_hash = "0x0acf01cf2a0f6b5fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725";

describe('test zkdex js function', function () {
    it('test sign withdraw', function () {
        let withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"asset_id\":\"0x1\"}";
        let sig_str = wasm.sign_withdraw(withdraw_req,pri_key);
        let hash = wasm.hash_withdraw(withdraw_req, "1");
        let sig = JSON.parse(sig_str)
        let expected_sig = {
            r: '0x6163ae912a4d58b227cabcd7aa064576dd8f7f5fb563010cb1f7774a3eaba911',
            s: '0x2aa5a2586d71bf67e7f906932b1662334a2101c585449bda890963a946c69f02'
        }
        assert.deepEqual(sig, expected_sig)
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sgin trasnfer', function () {
        let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        let sig_str = wasm.sign_transfer(transfer_req, pri_key);
        let hash = wasm.hash_transfer(transfer_req);
        let sig = JSON.parse(sig_str);
        let expected_sig = {r:"0x0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099",
            s:"0xb7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05"};
        assert.deepEqual(sig, expected_sig)

        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign limit order', function () {
        let limit_order_req = "{\"nonce\":\"1\",\"public_key\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"0x6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        let sig_str = wasm.sign_limit_order(limit_order_req, pri_key);
        let hash = wasm.hash_limit_order(limit_order_req);
        let sig = JSON.parse(sig_str);
        let expected_sig = {
            r: '0x0276d07a348630978fdecb67956c02ad9f244f2d072b5f8149814e041114950d',
            s: '0x43a5a30e6490dd002ca6743f5aab2f291930a489516336e1dcee57be84ead802'
        };
        assert.deepEqual(sig, expected_sig);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign liquide', function () {
        let liquide_req = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        let sig_str = wasm.sign_liquidate(liquide_req, pri_key);
        let hash = wasm.hash_liquidate(liquide_req);
        let sig = JSON.parse(sig_str);
        let expected_sig = {
            r: '0x19f6e2a51958df5649b6301e83dfc6b8fc34c140c929adf6896d5860d8f56b1b',
            s: '0x4c1b8c06fb73cdd4484ebd8199f0f2b0b5696fc3510a08a84681342ad4a48a05'
        }
        assert.deepEqual(sig, expected_sig);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign signed oracle price', function () {
        let oracle_price_req = "{\"signer_key\":\"0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        let sig_str = wasm.sign_signed_oracle_price(oracle_price_req, pri_key);
        let hash = wasm.hash_signed_oracle_price(oracle_price_req);
        let sig = JSON.parse(sig_str);
        let expected_sig = {
            r: '0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6',
            s: '0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502'
        };
        assert.deepEqual(sig,expected_sig);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test l1_sign', function () {
        let hash = "1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3";
        let ret = wasm.l1_sign(hash, pri_key);
        let o = JSON.parse(ret);
        let expected = {
            x: "0x02c5c5ab6dc2ae39c6bf239acd233c412ceebba1370cd4679ff78c3e57a33f90",
            y: "0x1fc29405cb5021e77aec60bfdd9ed43b245569e4cfc6e5720207e015662fd3b9",
            s: "0x03fcedddaa3803bc26fa98926d224f13857c1b600a3e99ba01cfcee8d54deaa3",
            pk_x: "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
            pk_y: "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b"
        }
        assert.deepEqual(o, expected);

    });

    it('test sign', function (){
        let hash = "0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12";
        let sig_str = wasm.sign(pri_key,hash);
        let sig = JSON.parse(sig_str);
        console.log(sig);
        assert.equal(wasm.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test private key from seed',function ()  {
        let seed = "hello world good life 996 very nice";
        let priStr = wasm.private_key_from_seed(seed);
        assert.equal(priStr,"0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    });

    it('test private key to public key xy', () => {
        let xy_str = wasm.private_key_to_pubkey_xy(pri_key);
        let xy = JSON.parse(xy_str);
        let expected = {
            x: "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
            y: "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b",
        }

        assert.deepEqual(xy, expected);
    });

    it('test is on curve', () => {
        let x = "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
        let y = "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
        let ret = wasm.is_on_curve(x, y);
        assert.equal(true, ret);
    });

    it('test pub key to xy', () => {
        let xy_str = wasm.public_key_to_xy("0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9")
        let xy = JSON.parse(xy_str)
        let expected_xy = {
            x: '0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9',
            y: '0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b'
        }
        assert.deepEqual(xy, expected_xy)
    })
})




