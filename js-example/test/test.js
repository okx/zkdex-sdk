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
        let json = '{"nonce":"1","public_key":"0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a","expiration_timestamp":"3608164305","amount":"1000000","asset_id":"0x00001","position_id":"1","chain_id":"1","fee":"0","eth_address":"0x0"}';
        let sig_str = zkdex.sign_spot_withdrawal(json, pri_key);
        let hash = zkdex.hash_spot_withdrawal(json);
        let sig = JSON.parse(sig_str);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, err_hash), false);
    })

    it('test sign eth address', () => {
            let sig = zkdex.sign_eth_address("0x505cec5b6c108dbf289c935802d6f8b53b5ae5b2","0x864d63b304b5635579771c0864def9bbc166ae5b1f39a894998ef350f6c521ac","0x05b82dd4f0325bf5fe7cc45ed2e8e8b47388d905f6b1d87c437f9732197425c4");
            assert.equal(sig,"0x209012cba7e208ab4a9338225568ffb87736721bdfad1168062eaf4a9c9ed04c0f5b1f07a4535f2ff29fe95a61166be31e62a7a418a8e1f1b51fd6ddaa566e090107f225011c74063739dfbee26f81f30d2ac0bfad5b8e188c8e48b4cc19fcd10fec8b35377b0f9bef295855de35e9d09e20379704d89f091f8343647490f68b");
     })

    it('test unified sign withdrawal', ()=> {
        let json = `
                                {
                                   "amount": "1682637359498011204",
                                   "eth_address": "0xB6aD5EfBd6aDfa29dEfad5BC0f8cE0ad57d4c5Fb",
                                   "expiration_timestamp": "2101470722",
                                   "asset_id": "0x11111",
                                   "nonce": "4265854110",
                                   "position_id": "775817640",
                                   "fee":"0",
                                   "public_key": "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a",
                                   "chain_id": "123"
                               }
        
        `;

        let sig_str = zkdex.unified_sign_withdrawal(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0xac9e44326ff48c57b47370a51adc0c8de9a9a3c84a9dc22db5c6777a1a640fe8');
        assert.equal(sig.s,'0x018b5aa8267edecdb21a7383831c448c9cb93965cc76e12b796a66920e3482b7');

        let hash = zkdex.unified_hash_withdrawal(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test unified sign transfer', ()=> {
        let json = `
                         {
                            "amount": "7758176404715800194",
                            "asset_id": "0x1234",
                            "synthetic_id" : "0x0",
                            "expiration_timestamp": "2404381470",
                            "nonce": "2195908194",
                            "receiver_position_id": "609106",
                            "receiver_public_key": "0x259f432e6f4590b9a164106cf6a659eb4862b21fb97d43588561712e8e5216b",
                            "sender_position_id": "93098",
                            "sender_public_key": "0x28e4d45cd0538ffa6fdc09e70f0fea4e56c47fda87a2a969c22b4fdfe997f60"
                        }
        
        `;

        let sig_str = zkdex.unified_sign_transfer(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0x281b28a1a2548cb0ca16a8c49b0039dfb48fb59d46a8dc82a2d73f44005bdc9a');
        assert.equal(sig.s,'0x047a122cb46c03a131e671dea7f2545ac503c141810bc1d8040111649be7adc6');

        let hash = zkdex.unified_hash_transfer(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test unified sign spot trade', ()=> {
        let json = `
                         {
                            "party_a_order": {
                                "amount_buy": "80",
                                "amount_sell": "70",
                                "amount_fee": "111",
                                "expiration_timestamp": "3396833",
                                "nonce": "1654615998",
                                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                                "asset_buy": "0x22222",
                                "asset_sell": "0x1111",
                                "position_id": "922337"
                            },
                            "party_b_order": {
                                "amount_buy": "80",
                                "amount_sell": "70",
                                "amount_fee": "111",
                                "expiration_timestamp": "3396833",
                                "nonce": "1654615998",
                                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                                "asset_buy": "0x2222",
                                "asset_sell": "0x111",
                                "position_id": "9223"
                            },
                            "actual_a_sold": "30",
                            "actual_b_sold": "40",
                            "actual_a_fee": "1",
                            "actual_b_fee": "-2"
                        }
        
        `;

        let sig_str = zkdex.unified_sign_spot_trade(json, pri_key,pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.signature_a.r,'0x0a2b0c3cf58f4eeca57fd7681d273e7ed024857334a153f97987adba5462d094');
        assert.equal(sig.signature_a.s,'0x0291850c33dd523e361bfa3518e7c8e4079227ec1874f3bbf0c308e3e398e0dd');
        assert.equal(sig.signature_b.r,'0x815275ff98bfd56ac5548d33949c739ba8ac8fddd9545456570f137aa241320f');
        assert.equal(sig.signature_b.s,'0x01ec94f6488ee3e9d2a6e38082bd5ea175b52aaec7407aab14d10efa2e0f55b4');

        let hash_str = zkdex.unified_hash_spot_trade(json);
        let hash = JSON.parse(hash_str);
        assert.equal(zkdex.verify_signature(sig.signature_a.r, sig.signature_a.s, pub_key_x,pub_key_y, hash.hash_a), true);
        assert.equal(zkdex.verify_signature(sig.signature_b.r, sig.signature_b.s, pub_key_x,pub_key_y, hash.hash_b), true);
    })

    it('test unified sign perpetual trade', ()=> {
        let json = `
                         {
                    "party_a_order":{
                        "type":"PERP_CROSS",
                        "amount_collateral":"15334874",
                        "amount_fee":"1767749",
                        "amount_synthetic":"15460142",
                        "asset_id_collateral":"0x57d05d",
                        "asset_id_synthetic":"0x2",
                        "expiration_timestamp":"3608164305",
                        "is_buying_synthetic":true,
                        "nonce":"1210484339",
                        "order_type":"LIMIT_ORDER_WITH_FEES",
                        "position_id":"4805234",
                        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
                    },
                    "party_b_order":{
                        "type":"PERP_CROSS",
                        "amount_collateral":"15334874138764573096",
                        "amount_fee":"17677494534592486883",
                        "amount_synthetic":"15460142528840632302",
                        "asset_id_collateral":"0x57d05d",
                        "asset_id_synthetic":"0x2",
                        "expiration_timestamp":"36081",
                        "is_buying_synthetic":true,
                        "nonce":"12104",
                        "order_type":"LIMIT_ORDER_WITH_FEES",
                        "position_id":"48052349",
                        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
                   
                    },
                    "actual_a_fee":"87916620",
                    "actual_b_fee":"-9309",
                    "actual_collateral":"775817",
                    "actual_synthetic":"1530808"
                }
        
        `;

        let sig_str = zkdex.unified_sign_perpetual_trade(json, pri_key,pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.signature_a.r,'0x05b3949d9397f8aa5bff3e2858f493e16691965d5d09e59d94213583ba2b85a5');
        assert.equal(sig.signature_a.s,'0x01f87f794dc75a3e157b8b2b8ebd3781842d84404c91b76c624cb94f8566cb2b');
        assert.equal(sig.signature_b.r,'0x8bf248588ff8a993641394280d5db01b5c2c378bea1fe5f14b6d05539274ee6f');
        assert.equal(sig.signature_b.s,'0x03f7800345fa619567b92791ea323e709ea3466a0be3dafc118981fc1d9ef422');

        let hash_str = zkdex.unified_hash_perpetual_trade(json);
        let hash = JSON.parse(hash_str);
        assert.equal(zkdex.verify_signature(sig.signature_a.r, sig.signature_a.s, pub_key_x,pub_key_y, hash.hash_a), true);
        assert.equal(zkdex.verify_signature(sig.signature_b.r, sig.signature_b.s, pub_key_x,pub_key_y, hash.hash_b), true);
    })

    it('test unified sign oracle', ()=> {
        let json = `
                          {
                            "signer_key": "0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83",
                            "external_price": "30000000",
                            "timestamp": "1651148012",
                            "signed_asset_id": "0x425443555344000000000000000000004d616b6572"
                          }
        `;

        let sig_str = zkdex.unified_sign_oracle_price(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0x094cd1d065e17ee1dd32682eb7328c0981501f93fc1a9f6befd93d81f18c4ac6');
        assert.equal(sig.s,'0x008a8d751047b04ee9080ca0b58330dd6a847a3954f95dab3c04585437ca8458');

        let hash = zkdex.unified_hash_oracle_price(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test unified sign liquidate', ()=> {
        let json = `
                          {
                    "actual_collateral":"7758176404715800194",
                    "actual_liquidator_fee":"8791662011684601223",
                    "actual_synthetic":"15308084094301570617",
                    "liquidated_position_id":"1541968236",
                    "liquidated_type":"PERP_CROSS",
                    "liquidator_order":{
                        "amount_collateral":"8187132600743567510",
                        "amount_fee":"11081939229867047606",
                        "amount_synthetic":"16558026091473266411",
                        "asset_id_collateral":"0x57d05d1",
                        "asset_id_synthetic":"0x2",
                        "expiration_timestamp":"1430804514",
                        "is_buying_synthetic":false,
                        "type":"PERP_CROSS",
                        "nonce":"3900315155",
                        "position_id":"11534",
                        "public_key":"0x5db665983e23607de57d6dc068797336bfdcb954238044688bec922ca296d3e"
                        }
                    }
        `;

        let sig_str = zkdex.unified_sign_liquidate(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0x908bcabbc7593af06c834eb8ae3db82883028eae8f68897b034e26b2fde76000');
        assert.equal(sig.s,'0x020de17410d65b6a93680f854cdb7f3d4cfbd4f55ffd0c8f6bcba945eec9ac5f');

        let hash = zkdex.unified_hash_liquidate(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test unified sign spot limit order', ()=> {
        let json = `
                          {
                                     "amount_buy": "80",
                                     "amount_sell": "70",
                                     "amount_fee": "111",
                                     "expiration_timestamp": "3396833",
                                     "nonce": "1654615998",
                                     "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                                     "asset_buy": "0x22222",
                                     "asset_sell": "0x1111",
                                     "position_id": "922337"
                }
        `;

        let sig_str = zkdex.unified_sign_spot_limit_order(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0x0a2b0c3cf58f4eeca57fd7681d273e7ed024857334a153f97987adba5462d094');
        assert.equal(sig.s,'0x0291850c33dd523e361bfa3518e7c8e4079227ec1874f3bbf0c308e3e398e0dd');

        let hash = zkdex.unified_hash_spot_limit_order(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })

    it('test unified sign perpetual limit order', ()=> {
        let json = `
                          {
                         "type":"PERP_CROSS",
                         "amount_collateral":"15334874",
                         "amount_fee":"1767749",
                         "amount_synthetic":"15460142",
                         "asset_id_collateral":"0x57d05d",
                         "asset_id_synthetic":"0x2",
                         "expiration_timestamp":"3608164305",
                         "is_buying_synthetic":true,
                         "nonce":"1210484339",
                         "order_type":"LIMIT_ORDER_WITH_FEES",
                         "position_id":"4805234",
                         "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
                }
        `;

        let sig_str = zkdex.unified_sign_perpetual_limit_order(json, pri_key);
        let sig = JSON.parse(sig_str);
        assert.equal(sig.r,'0x05b3949d9397f8aa5bff3e2858f493e16691965d5d09e59d94213583ba2b85a5');
        assert.equal(sig.s,'0x01f87f794dc75a3e157b8b2b8ebd3781842d84404c91b76c624cb94f8566cb2b');

        let hash = zkdex.unified_hash_perpetual_limit_order(json);
        assert.equal(zkdex.verify_signature(sig.r, sig.s, pub_key_x,pub_key_y, hash), true);
    })
})




