import * as wasm from '@okbchain/zkdex-sdk';
import {ethers as utils} from 'ethers';
import * as assert from "assert";

const pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
const pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";

describe('test zkdex js function', function () {
    it('test sign withdraw', function () {
        let withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":2,\"amount\":3,\"eth_address\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\"}";
        let sig_str = wasm.sign_withdraw(withdraw_req, "1", pri_key);
        let hash = wasm.hash_withdraw(withdraw_req, "1");
        console.log(hash);
        let sig = JSON.parse(sig_str)
        console.log(sig)
        assert.equal(wasm.verify_signature(sig.r.replaceAll("0x",""), sig.s.replaceAll("0x",""), pub_key, hash), true);
    });
})




