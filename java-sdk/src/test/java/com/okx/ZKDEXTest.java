package com.okx;

import com.alibaba.fastjson2.JSON;
import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class ZKDEXTest {
    static String priKey = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273";
    static String pubKeyX = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
    static String pubKeyY = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";

    @Test
    public void verifySignature() throws Exception {
        String sigr = "0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749";
        String sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
        String msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
        String errMsg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d1a5ab082f1a520";
        assert ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, msg);
        assert !ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, errMsg);
    }

    @Test(expected = java.lang.Exception.class)
    public void verifySignatureWithException() throws Exception {
        String sigr = "0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443";
        String sigs = "0xc80432d841049c2e71fcb590febcde58ae7cc1f064460bb4de474f930505";
        String msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520";

        ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, msg);
    }

    @Test(expected = java.lang.Exception.class)
    public void verifySignatureExpectedException() throws Exception {
        String sigr = "0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443";
        String sigs = "0xc80432d841049c2e71fcb590febcde58ae7cc1f064460bb4de474f930505";
        String msg = null;

        ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, msg);
    }

    @Test
    public void signWithdraw() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String sigStr = ZKDEX.signWithdraw(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0xa5d62dbb0566a1b69162df475097fbfca6a317535ea59ea3275580dce2d7043e", "0x03c61d342a339d329341494ee136ccadf10675b9f8f90894e6a9e86ac6a19dec");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashWithdraw(json);
        System.out.println(hash);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);

    }

    @Test(expected = java.lang.Exception.class)
    public void signWithdrawWithErrorJSON() throws Exception {
        String json = "{\"public_key\":\"0x4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String sigStr = ZKDEX.signWithdraw(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x6163ae912a4d58b227cabcd7aa064576dd8f7f5fb563010cb1f7774a3eaba911", "0x2aa5a2586d71bf67e7f906932b1662334a2101c585449bda890963a946c69f02");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashWithdraw(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);

    }

    @Test(expected = java.lang.Exception.class)
    public void signWithdrawWithNullParams() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String sigStr = ZKDEX.signWithdraw(null, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x6163ae912a4d58b227cabcd7aa064576dd8f7f5fb563010cb1f7774a3eaba911", "0x2aa5a2586d71bf67e7f906932b1662334a2101c585449bda890963a946c69f02");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashWithdraw(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);

    }

    @Test
    public void signTransfer() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String sigStr = ZKDEX.signTransfer(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0xa5920612d2b265813f31ee169b9e96e89548bdd53e9f4541e53fcdb1205c9c9a", "0x0028bdb4cc8f9f70c6ad081c03d662599fe732c118f268e537da019e3b473a09");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashTransfer(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signTransferWithErrorJSON() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"1\"}";
        String sigStr = ZKDEX.signTransfer(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099", "0xb7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashTransfer(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signTransferWithNullParam() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String sigStr = ZKDEX.signTransfer(null, null);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099", "0xb7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashTransfer(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
   public void signLimitOrder() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"0x6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String sigStr = ZKDEX.signLimitOrder(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0xb009ccc02daa847671c14bbe2ae576076d0ed8e4ed9af3b8553b1090a122f2b7", "0x0319dcc4dde119be949f194aeaa727d4ac0a1666f4e260436b1a9fd5b9d4e739");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLimitOrder(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signLimitOrderWithErrJSON() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":5,\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String sigStr = ZKDEX.signLimitOrder(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0276d07a348630978fdecb67956c02ad9f244f2d072b5f8149814e041114950d", "0x43a5a30e6490dd002ca6743f5aab2f291930a489516336e1dcee57be84ead802");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLimitOrder(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signLimitOrderWithNullParm() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String sigStr = ZKDEX.signLimitOrder(null, null);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x0276d07a348630978fdecb67956c02ad9f244f2d072b5f8149814e041114950d", "0x43a5a30e6490dd002ca6743f5aab2f291930a489516336e1dcee57be84ead802");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLimitOrder(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void signLiquidate() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String sigStr = ZKDEX.signLiquidate(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0xa2b928904a4015f324244432ac4cc28286446f93cc6e0e8fcd0f6a9278a152f5", "0x01b612dd6801d8044f3ad6e345cabc3c7f41a02ecfdfe3c48fd81eb4ac01fd36");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLiquidate(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signLiquidateWithErrJSON() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaaaaa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String sigStr = ZKDEX.signLiquidate(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x19f6e2a51958df5649b6301e83dfc6b8fc34c140c929adf6896d5860d8f56b1b", "0x4c1b8c06fb73cdd4484ebd8199f0f2b0b5696fc3510a08a84681342ad4a48a05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLiquidate(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signLiquidateWithNullParam() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String sigStr = ZKDEX.signLiquidate(null, null);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x19f6e2a51958df5649b6301e83dfc6b8fc34c140c929adf6896d5860d8f56b1b", "0x4c1b8c06fb73cdd4484ebd8199f0f2b0b5696fc3510a08a84681342ad4a48a05");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLiquidate(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void signSignedOraclePrice() throws Exception {
        String json = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        String sigStr = ZKDEX.signSignedOraclePrice(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x8510a3eab6ac786e2c97c59db9fc5ea60eb39057b61e746fe2120e02c163fd4b", "0x035ac9dd0980f0625b5d540ce43b62171cb80ed07cc63df88a8990ce2f4ea293");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSignedOraclePrice(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signSignedOraclePriceWithErrJSON() throws Exception {
        String json = "{\"signer_key\":\"0x42cbdc4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":\"1\",\"signed_asset_id\":\"0x3\"}";
        String sigStr = ZKDEX.signSignedOraclePrice(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6", "0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSignedOraclePrice(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signSignedOraclePriceWithNullParam() throws Exception {
        String json = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        String sigStr = ZKDEX.signSignedOraclePrice(null, null);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6", "0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSignedOraclePrice(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void hashWithdraw() throws Exception {
        String withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String hash = ZKDEX.hashWithdraw(withdraw_req);
        assertEquals("0x22e58e85163d975aba853ef13742320fc8f7b5e1fed5667e37a275916e96a561", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashWithdrawWithErrJSON() throws Exception {
        String withdraw_req = "{\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String hash = ZKDEX.hashWithdraw(withdraw_req);
        assertEquals("0xc915ffc0969b9232594c47eb6046f575eee0b5c4fcdf65508135aaad199ba008", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashWithdrawWithNullParam() throws Exception {
        String withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        String hash = ZKDEX.hashWithdraw(null);
        assertEquals("0xc915ffc0969b9232594c47eb6046f575eee0b5c4fcdf65508135aaad199ba008", hash);
    }


    @Test
    public void hashTransfer() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String hash = ZKDEX.hashTransfer(json);
        assertEquals("0x023408af1feaf9432599c6562003b4f105a83aa7fa5bf9dbfb17e37d2f876601", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashTransferWithErrJSON() throws Exception {
        String json = "{\"nonce\":\"0\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String hash = ZKDEX.hashTransfer(json);
        assertEquals("0x4a1b4eb5df9b7d3809b6d3d45466a0bfd98248db13cca04c538b184f4b76bd10", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashTransferWithNullParam() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String hash = ZKDEX.hashTransfer(null);
        assertEquals("0x4a1b4eb5df9b7d3809b6d3d45466a0bfd98248db13cca04c538b184f4b76bd10", hash);
    }

    @Test
    public void hashLimitOrder() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String hash = ZKDEX.hashLimitOrder(json);
        assertEquals("0x151301a401fab9fdf8d88f5d28261740a9fb7ecbfc1110312e67480a40deb51c", hash);
    }


    @Test(expected = java.lang.Exception.class)
    public void hashLimitOrderWithErrJSON() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":1,\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String hash = ZKDEX.hashLimitOrder(json);
        assertEquals("0x0acf01cf2a0fa95fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725", hash);
    }


    @Test(expected = java.lang.Exception.class)
    public void hashLimitOrderWithNullParam() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        String hash = ZKDEX.hashLimitOrder(null);
        assertEquals("0x0acf01cf2a0fa95fe13c2ff4f6a38fa382e3b10acf342bab5f8826d5feada725", hash);
    }


    @Test
    public void hashLiquidate() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String hash = ZKDEX.hashLiquidate(json);
        assertEquals("0x11fbfdb033ed2a370a6213e172d48aa152254597ec9d16b7d851ffacfa9ae29e", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashLiquidateWithErrJSON() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":1,\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String hash = ZKDEX.hashLiquidate(json);
        assertEquals("0x5097ece4492d9b285998543201ec03a4a2324408d5ac9fa5942e4aa27919fe00", hash);
    }


    @Test(expected = java.lang.Exception.class)
    public void hashLiquidateWithNullParam() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        String hash = ZKDEX.hashLiquidate(null);
        assertEquals("0x5097ece4492d9b285998543201ec03a4a2324408d5ac9fa5942e4aa27919fe00", hash);
    }


    @Test
    public void hashSignedOraclePrice() throws Exception {
        String json = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        String hash = ZKDEX.hashSignedOraclePrice(json);
        assertEquals("0x2aff026c07ab995e4874dd866d81ed332ed5580d938a4a4ef0b7f54ed500c9e2", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashSignedOraclePriceWithErrJSON() throws Exception {
        String json = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        String hash = ZKDEX.hashSignedOraclePrice(json);
        assertEquals("0x20a5f182b00a166df3922b8cb92b2ef5156cb0ed18be0f2c08d0a1bed57e8101", hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void hashSignedOraclePriceWithNullParam() throws Exception {
        String json = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        String hash = ZKDEX.hashSignedOraclePrice(null);
        assertEquals("0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520", hash);
    }


    @Test
    public void sign() throws Exception {
        String hash = "0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12";
        String sigStr = ZKDEX.sign(priKey, hash);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        assertEquals(new Signature("0x1fb6b3bd2d5cf21862d38a189a16056926d00dc37cdc36586b60b2c9115c762c", "0x002b7e082743fd090f407430dc28e7031af4191f7e876b1abb01c901583170ca"), signature);
    }

    @Test(expected = java.lang.Exception.class)
    public void signWithErrParam() throws Exception {
        String hash = "0x4068df2520d7b11133a1c6ef279a59765a6bc400e55bba6bf9b59c6cef63cb37d12";
        String sigStr = ZKDEX.sign(priKey, hash);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        assertEquals(new Signature("0x7e43943b23c798aebfbc002aa15d01a8a1a413c64773e241e78848a953b7a92a", "0x0088408fadc401b97354c9bfb982cdb0885c97f038de1e131a52772720944900"), signature);
    }

    @Test(expected = java.lang.Exception.class)
    public void signWithNullParam() throws Exception {
        String hash = "0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12";
        String sigStr = ZKDEX.sign(priKey, null);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        assertEquals(new Signature("0x7e43943b23c798aebfbc002aa15d01a8a1a413c64773e241e78848a953b7a92a", "0x0088408fadc401b97354c9bfb982cdb0885c97f038de1e131a52772720944900"), signature);
    }

    @Test
    public void isOnCurve() throws Exception {
        String x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        String y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        boolean ret = ZKDEX.isOnCurve(x, y);
        assertEquals(true, ret);
    }

    @Test(expected = java.lang.Exception.class)
    public void isOnCurveWithErrParam() throws Exception {
        String x = "0x0d93a09887aaba49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        String y = "0x0a966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        boolean ret = ZKDEX.isOnCurve(x, y);
        assertEquals(ret, true);
    }

    @Test(expected = java.lang.Exception.class)
    public void isOnCurveWithNullParam() throws Exception {
        String x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        String y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        boolean ret = ZKDEX.isOnCurve(null, y);
        assertEquals(true, ret);
    }


    @Test
    public void privateKeyFromSeed() throws Exception {
        String seed = "hello world good life 996 very nice";
        String priKey = ZKDEX.privateKeyFromSeed(seed);
        assertEquals(priKey, "0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    }

    @Test(expected = java.lang.Exception.class)
    public void privateKeyFromSeedWithNotEnoughLength() throws Exception {
        String seed = "hello world good life 996 very";
        String priKey = ZKDEX.privateKeyFromSeed(seed);
        assertEquals(priKey, "0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    }

    @Test(expected = java.lang.Exception.class)
    public void privateKeyFromSeedWithNullParam() throws Exception {
        String priKey = ZKDEX.privateKeyFromSeed(null);
        assertEquals(priKey, "0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9");
    }

    @Test
    public void privateKeyToPublicKeyXY() throws Exception {
        String pri_key = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273";
        String publickeyStr = ZKDEX.privateKeyToPublicKeyXY(pri_key);
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test(expected = java.lang.Exception.class)
    public void privateKeyToPublicKeyXYWithErrParam() throws Exception {
        String pri_key = "0x0551091cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        String publickeyStr = ZKDEX.privateKeyToPublicKeyXY(pri_key);
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test(expected = java.lang.Exception.class)
    public void privateKeyToPublicKeyXYWithNullParam() throws Exception {
        String pri_key = null;
        String publickeyStr = ZKDEX.privateKeyToPublicKeyXY(pri_key);
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test
    public void publicKeyToXY() throws Exception {
        String publickeyStr = ZKDEX.publicKeyToXY("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa");
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test(expected = java.lang.Exception.class)
    public void publicKeyToXYWithErrParam() throws Exception {
        String publickeyStr = ZKDEX.publicKeyToXY("0x42ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9");
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test(expected = java.lang.Exception.class)
    public void publicKeyToXYWithNullParam() throws Exception {
        String publickeyStr = ZKDEX.publicKeyToXY(null);
        Publickey publickey = JSON.parseObject(publickeyStr, Publickey.class);
        assertEquals("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa", publickey.getX());
        assertEquals("0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7", publickey.getY());
    }

    @Test
    public void l1sign() throws Exception {
//        String msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
//        String pri = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
//        String json = ZKDEX.ethSign(pri, msg);
//        EthAddressSignature signature = JSON.parseObject(json, EthAddressSignature.class);
//
//        EthAddressSignature expectedSig = new EthAddressSignature();
//        expectedSig.setX("0x062b74e4bde7c5655093bcfd717b2be2757fc7c85f2b5fdc0f43820df2ce510a");
//        expectedSig.setY("0x124c1159c6164b8f80348f23a39ff79af229ecb2f00e806e60798601607c4595");
//        expectedSig.setS("0x04f89ebc83800e89f19e3501562793e2d9097b921ee0759b5f37017b993238c4");
//        expectedSig.setPkX("0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86");
//        expectedSig.setPkY("0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55");

//        assertEquals(expectedSig, signature);
//        assertEquals(true,ZKDEX.l2Verify(signature.getX(),signature.getY(),signature.getS(),signature.getPkX(),signature.getPkY(),msg));
        Boolean ret = ZKDEX.l2Verify("1b788bae91e209e0c0bd3378c436c8dffd77c309bf91e079269209c7b2a4420a","21c7312575dd3ceae528c7437e3d25d39f76ce950ffff9d069e78d3735114486","0437fdd0062cbbb0b884c88855c07270f1b91a62ba5a1754c59f8be7c35655f8","0x00ff19b5400701b6e8d4b2344763f965197459e0d327833a4f1eb9175a863c5d","0x1627e696b01c3f6b7889d6fb6fd99a62d53e2915319319b8d8c9cdc7e2272a76","0x08739c26a8663449be48fe75bc1c73982e81e14de24c287a169cb83ead8dcba0");
        assertEquals(true,ret);

    }

    @Test
    public void test_sign_spot_transfer() throws Exception {
        String json = "{\"nonce\":\"1\",\"sender_public_key\":\"0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a\",\"expiration_timestamp\":\"3608164305\",\"amount\":\"10\",\"asset_id\":\"0x00001\",\"receiver_position_id\":\"1\",\"receiver_public_key\":\"0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a\",\"sender_position_id\":\"1\"}";
        String sigStr = ZKDEX.signSpotTransfer(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x2e3aaadfec701f1b18b0fc95798d93c6a5a4ac24117c18200b2010aadb67248c", "0x04b67a05dda815d69c1334e772c73f662c0df65a8c0e4a74a672e6823c133ddf");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSpotTransfer(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void test_sign_spot_withdrawal() throws Exception {
        String json = "{\"nonce\":\"1\",\"public_key\":\"0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a\",\"expiration_timestamp\":\"3608164305\",\"amount\":\"1000000\",\"asset_id\":\"0x00001\",\"position_id\":\"1\",\"chain_id\":\"1\",\"fee\":\"0\",\"eth_address\":\"0x0\"}";
        String sigStr = ZKDEX.signSpotWithdrawal(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0xa5ddaa85042f91be1d036a89d49cb9532f063a911516f3c13a55fa7889e03d70", "0x05cfefa9b959b4538bf2050286025dd522ad047e1f1ae499ae3627ac6ba5aa59");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSpotWithdrawal(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void test_sign_spot_limit_order() throws Exception {
        String json = "{\"nonce\":\"0\",\"expiration_timestamp\":\"0\",\"public_key\":\"0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a\",\"amount_buy\":\"0\",\"amount_sell\":\"0\",\"amount_fee\":\"0\",\"asset_buy\":\"0x01\",\"asset_sell\":\"0x02\",\"position_id\":\"1\"}";
        String sigStr = ZKDEX.signSpotLimitOrder(json, priKey);
        Signature signature = JSON.parseObject(sigStr, Signature.class);
        Signature expectSig = new Signature("0x01aabe43b11787a211f9960a2abd2de3667965c52b5ff23ac853a91ebfc9b6c2", "0x01ffebd7ab388ae453baa839f123116bdfac8b57931bbbc463cf8dfcfab6fc02");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashSpotLimitOrder(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test
    public void test_l2_verify() throws Exception{
        Boolean ret = ZKDEX.l2Verify("1b788bae91e209e0c0bd3378c436c8dffd77c309bf91e079269209c7b2a4420a",
                "21c7312575dd3ceae528c7437e3d25d39f76ce950ffff9d069e78d3735114486",
                "0437fdd0062cbbb0b884c88855c07270f1b91a62ba5a1754c59f8be7c35655f8",
                "0x00ff19b5400701b6e8d4b2344763f965197459e0d327833a4f1eb9175a863c5c",
                "0x0cc0b1fbe52a846fcbc3bd7d238146ff19ed76a78fddff0dd5fb98d911b36984",
                "0x11abcd25a3a8f132217fd411397cc2e38521f62dd1afafec2e9956b28d53044c");
        assertEquals(false,ret);

        Boolean ret2 = ZKDEX.l2Verify("2cdb8f1de3bf58c83060aab3d5fa041812dbe114aca6a32f2e5d43be2e1ead0c",
                "0c0cd3a4ee3a61157a99c051fbc6e0f809820a7b6430740ecc9651f4be68b901",
                "0500a34df5f0d7e3b7aefc1f1a796b2fe990c49afa10e84703b34493f125779a",
                "0x87736b70b0761c82dbc5950e434b71517b4b7e78e35fcf4ae3e12bbb9be38109",
                "0x0246d308bb9bff172be1858fee0cd3da2d1870818a8901cc5c1e124dc9814a6d",
                "0x03a8a1fd4975219bdb8b098f86ed49a52a05257be2f2ce75950fdfc9f95e66b2");
        assertEquals(true,ret2);
    }
}