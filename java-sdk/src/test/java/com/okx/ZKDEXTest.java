package com.okx;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONObject;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class ZKDEXTest {
    static String priKey = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    static String pubKeyX = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    static String pubKeyY = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";

    @Test
    void verifySignature() throws Exception {
        String sigr = "353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6";
        String sigs = "c80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502";
        String pubKey = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
        String msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520";
        String errMsg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d1a5ab082f1a520";
        assert ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, msg);
        assert !ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, errMsg);
    }

    @Test
    void signWithdraw() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":2,\"amount\":3,\"eth_address\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\"}";
        String sigStr = ZKDEX.signWithdraw(json, "1", priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x6163ae912a4d58b227cabcd7aa064576dd8f7f5fb563010cb1f7774a3eaba911", "0x2aa5a2586d71bf67e7f906932b1662334a2101c585449bda890963a946c69f02");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashWithdraw(json, "1");
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);

    }

    @Test
    void signTransfer() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
        String sigStr = ZKDEX.signTransfer(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099", "0xb7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashTransfer(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    void signLimitOrder() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":3,\"amount_collateral\":4,\"amount_fee\":5,\"asset_id_synthetic\":6,\"asset_id_collateral\":\"7\",\"position_id\":8,\"is_buying_synthetic\":false}";
        String sigStr = ZKDEX.signLimitOrder(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0276d07a348630978fdecb67956c02ad9f244f2d072b5f8149814e041114950d", "0x43a5a30e6490dd002ca6743f5aab2f291930a489516336e1dcee57be84ead802");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLimitOrder(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    void signLiquidate() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":1,\"amount_collateral\":2,\"amount_fee\":3,\"asset_id_synthetic\":4,\"asset_id_collateral\":\"0x5\",\"position_id\":6,\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String sigStr = ZKDEX.signLiquidate(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x19f6e2a51958df5649b6301e83dfc6b8fc34c140c929adf6896d5860d8f56b1b", "0x4c1b8c06fb73cdd4484ebd8199f0f2b0b5696fc3510a08a84681342ad4a48a05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLiquidate(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    void signSignedOraclePrice() throws Exception {
        String json = "{\"signer_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":1,\"timestamp\":2,\"signed_asset_id\":\"0x3\"}";
        String sigStr = ZKDEX.signSignedOraclePrice(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6", "0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSignedOraclePrice(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    void hashWithdraw() throws Exception {
        String withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":2,\"amount\":3,\"eth_address\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\"}";
        String hash = ZKDEX.hashWithdraw(withdraw_req, "1");
        assertEquals("c915ffc0969b9232594c47eb6046f575eee0b5c4fcdf65508135aaad199ba008", hash);
    }

    @Test
    void hashTransfer() throws Exception {
        String transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
        String hash = ZKDEX.hashTransfer(transfer_req);
        assertEquals("4a1b4eb5df9b7d3809b6d3d45466a0bfd98248db13cca04c538b184f4b76bd10", hash);
    }

    @Test
    void hashLimitOrder() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":3,\"amount_collateral\":4,\"amount_fee\":5,\"asset_id_synthetic\":6,\"asset_id_collateral\":\"7\",\"position_id\":8,\"is_buying_synthetic\":false}";
        String hash = ZKDEX.hashLimitOrder(json);
        assertEquals("0acf01cf2a0fa95fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725", hash);
    }

    @Test
    void hashLiquidate() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":1,\"amount_collateral\":2,\"amount_fee\":3,\"asset_id_synthetic\":4,\"asset_id_collateral\":\"0x5\",\"position_id\":6,\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String hash = ZKDEX.hashLiquidate(json);
        assertEquals("5097ece4492d9b285998543201ec03a4a2324408d5ac9fa5942e4aa27919fe00", hash);
    }

    @Test
    void hashSignedOraclePrice() throws Exception {
        String json = "{\"signer_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":1,\"timestamp\":2,\"signed_asset_id\":\"0x3\"}";
        String hash = ZKDEX.hashSignedOraclePrice(json);
        assertEquals("01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520", hash);
    }


    @Test
    void sign() throws Exception {
        String hash = "0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12";
        String sigStr = ZKDEX.sign(priKey, hash);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        assertEquals(new Signature("0x7e43943b23c798aebfbc002aa15d01a8a1a413c64773e241e78848a953b7a92a", "0x0088408fadc401b97354c9bfb982cdb0885c97f038de1e131a52772720944900"), signature);
    }

    @Test
    void isOnCurve() throws Exception {
        String x = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
        String y = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
        boolean ret = ZKDEX.isOnCurve(x, y);
        assertEquals(true, ret);
    }


    @Test
    void privateKeyFromSeed() throws Exception {
        String seed = "hello world good life 996 very nice";
        String priKey = ZKDEX.privateKeyFromSeed(seed);
        assertEquals(priKey, "02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    }

    @Test
    void privateKeyToPublicKeyXY() throws Exception {
        String pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        String xy = ZKDEX.privateKeyToPublicKeyXY(pri_key);
        JSONObject object = JSON.parseObject(xy);
        assertEquals("42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9", object.get("x"));
        assertEquals("210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b", object.get("y"));
    }
}