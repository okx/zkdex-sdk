const assert = require("assert");
const zkdex = require('@okbchain/zkdex-sdk');
const utils = require('ethers');

const pri_key = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273";
const pub_key_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
const pub_key_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
const err_hash = "0x0acf01cf2a0f6b5fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725";

describe('test zkdex js function', function () {
    it('test sign withdraw', function () {
        let withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        let sig_str = zkdex.sign_withdraw(withdraw_req,pri_key);
        let hash = zkdex.hash_withdraw(withdraw_req);
        let sig = JSON.parse(sig_str)
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sgin trasnfer', function () {
        let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        let sig_str = zkdex.sign_transfer(transfer_req, pri_key);
        let hash = zkdex.hash_transfer(transfer_req);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign limit order', function () {
        let limit_order_req = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"0x6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        let sig_str = zkdex.sign_limit_order(limit_order_req, pri_key);
        let hash = zkdex.hash_limit_order(limit_order_req);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign liquide', function () {
        let liquide_req = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        let sig_str = zkdex.sign_liquidate(liquide_req, pri_key);
        let hash = zkdex.hash_liquidate(liquide_req);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test sign signed oracle price', function () {
        let oracle_price_req = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        let sig_str = zkdex.sign_signed_oracle_price(oracle_price_req, pri_key);
        let hash = zkdex.hash_signed_oracle_price(oracle_price_req);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    });

    it('test l1_sign', function () {
        let hash = "1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3";
        let ret = zkdex.l1_sign(hash, pri_key);
        let o = JSON.parse(ret);
        let expected = {
            x: "0x2521cad28a1fa5039ecf6bb6d06f021e34b6ee77e0f4e1eb9d612db97b14ca02",
            y: "0x076602691a75e7c60a3b84ed278a6a974f5be4d49870e9f78a2d4d8219ec1fdd",
            s: "0x00e73702dcf82eecd5263169a04e11eac7b8ecd386e173f31dddf11b5e84baa6",
            pk_x: "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
            pk_y: "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7"
        }
        assert.deepEqual(o, expected);

    });

    it('test sign', function (){
        let hash = "0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12";
        let sig_str = zkdex.sign(pri_key,hash);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test private key from seed',function ()  {
        let seed = "hello world good life 996 very nice";
        let priStr = zkdex.private_key_from_seed(seed);
        assert.equal(priStr,"0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    });

    it('test private key to public key xy', () => {
        let xy_str = zkdex.private_key_to_pubkey_xy(pri_key);
        let xy = JSON.parse(xy_str);
    });

    it('test is on curve', () => {
        let x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        let ret = zkdex.is_on_curve(x, y);
        assert.equal(true, ret);
    });

    it('test pub key to xy', () => {
        let xy_str = zkdex.public_key_to_xy("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa")
        let xy = JSON.parse(xy_str)
        let expected_xy = {
            x: '0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa',
            y: '0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7'
        }
        assert.deepEqual(xy, expected_xy)
    })

    it('test sign spot transfer', () => {
        let json = '{"nonce":"1","sender_public_key":"0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a","expiration_timestamp":"3608164305","amount":"10","asset_id":"0x00001","receiver_position_id":"1","receiver_public_key":"0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a","sender_position_id":"1"}';
        let sig_str = zkdex.sign_spot_transfer(json, pri_key);
        let hash = zkdex.hash_spot_transfer(json);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    })

    it('test sign spot limit order', () => {
        let json = '{"nonce":"0","expiration_timestamp":"0","public_key":"0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a","amount_buy":"0","amount_sell":"0","amount_fee":"0","asset_buy":"0x01","asset_sell":"0x02","position_id":"1"}';
        let sig_str = zkdex.sign_spot_limit_order(json, pri_key);
        let hash = zkdex.hash_spot_limit_order(json);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    })

    it('test sign spot withdrawal', () => {
        let json = '{"nonce":"1","public_key":"0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a","expiration_timestamp":"3608164305","amount":"1000000","asset_id":"0x00001","position_id":"1","chain_id":"1","eth_address":"0x0"}';
        let sig_str = zkdex.sign_spot_withdrawal(json, pri_key);
        let hash = zkdex.hash_spot_withdrawal(json);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    })

    it('test sign eth address', () => {

            let sig = zkdex.sign_eth_address("11155111","0x4b551A084cDdB1a5355Ce17155669A5ce6e94C4E","0x505cec5b6c108dbf289c935802d6f8b53b5ae5b2","0x864d63b304b5635579771c0864def9bbc166ae5b1f39a894998ef350f6c521ac","0x05b82dd4f0325bf5fe7cc45ed2e8e8b47388d905f6b1d87c437f9732197425c4");
            assert.equal(sig,"0x2c1d253bc91f3b6ddd93967869e659769a1acc6f97259074a13c4439c227ac551bf6f27503e58467eae5341b7ba9ae960eae1fedc5d3a1a370ec53debc76be0b0347f733683afd7ae99a74a38d708c432242516de06ab6e5eb76bfbfbd0258830fec8b35377b0f9bef295855de35e9d09e20379704d89f091f8343647490f68b");

     })
})




