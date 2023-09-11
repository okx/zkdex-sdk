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
        Signature expectSig = new Signature("0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749", "0x037fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f");
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
        Signature expectSig = new Signature("0x094a47cb182c7eb24e3c34a473def9d356bb30161179e4bbaeaa48c6d18844f8", "0x05534d29f2f1d3ba474f7cec4f9f545924924e5f4261577d09ed9a85df252d5d");
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
        Signature expectSig = new Signature("0x003f3ab7c200c727633be0544f1bebe49ebb6ceebb6d76a026eef036d12ef2d1", "0x03a2ffcb3a5dfb21f0d58724f0451b18203f697095a61485fde4aa4601e2a8a3");
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
        Signature expectSig = new Signature("0x083facdecdd8609358af1614474074134649141567ffc6367e134b7dee33b367", "0x04e1852d5be741ad5c367622c5f4e3e86d5a78fa07dd93775105eba8c65ee40d");
        assertEquals(expectSig, signature);

        String hash = ZKDEX.hashLiquidate(json);
        assert ZKDEX.verifySignature(signature.getR(), signature.getS(), pubKeyX, pubKeyY, hash);
    }

    @Test(expected = java.lang.Exception.class)
    public void signLiquidateWithErrJSON() throws Exception {
        String json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x42cb7f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
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
        Signature expectSig = new Signature("0x22d1741361d9e4c2870a18d73daa0c4a7925aeb8ddfd04a2ea4886de7fb784f4", "0x0416c1bab51ba9906dfb4f7cb72468f0452cc7061030a85af272bd3ef84e24bc");
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
        assertEquals("0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9", hash);
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
        assertEquals("0x10bd764b4f188b534ca0cc13db4882d9bfa06654d4d3b609387d9bdfb54e1b4a", hash);
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
        assertEquals("0x25a7adfed526885fab2b34cf0ab1e382a38fa3f6f42f3ce15fa90f2acf01cf0a", hash);
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
        assertEquals("0x00fe1979a24a2e94a59facd5084432a2a403ec0132549859289b2d49e4ec9750", hash);
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
        assertEquals("0x20a5f182b00a166df3922b8cb92b2ef5156cb0ed18be0f2c08d0a1bed57e8101", hash);
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
        String x = "0x42cbb15f0b5ca78d57ff1e5948008799b9c0d330b1e26423467867643217a9";
        String y = "0x210add7125394a55df3e022f3994164c31803b3c8ac18edc91730b";
        boolean ret = ZKDEX.isOnCurve(x, y);
        assertEquals(true, ret);
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
        String msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
        String pri = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
        String json = ZKDEX.ethSign(pri, msg);
        EthAddressSignature signature = JSON.parseObject(json, EthAddressSignature.class);

        EthAddressSignature expectedSig = new EthAddressSignature();
        expectedSig.setX("0x062b74e4bde7c5655093bcfd717b2be2757fc7c85f2b5fdc0f43820df2ce510a");
        expectedSig.setY("0x124c1159c6164b8f80348f23a39ff79af229ecb2f00e806e60798601607c4595");
        expectedSig.setS("0x04f89ebc83800e89f19e3501562793e2d9097b921ee0759b5f37017b993238c4");
        expectedSig.setPkX("0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86");
        expectedSig.setPkY("0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55");

        assertEquals(expectedSig, signature);
    }
}