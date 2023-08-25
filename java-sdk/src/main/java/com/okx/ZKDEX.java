package com.okx;

public class ZKDEX {
    static {
        System.loadLibrary("zkdex_sdk");
    }

    public static native boolean verifySignature(String sig_r, String sig_s, String pubKeyX,String pubKeyY, String msg);

    public static native String signWithdraw(String json, String assetId, String priKey) throws Exception;

    public static native String signTransfer(String json, String priKey) throws Exception;

    public static native String signLimitOrder(String json, String priKey) throws Exception;

    public static native String signLiquidate(String json, String priKey) throws Exception;

    public static native String signSignedOraclePrice(String json, String priKey) throws Exception;

    public static native String hashWithdraw(String json, String assetId) throws Exception;

    public static native String hashTransfer(String json) throws Exception;

    public static native String hashLimitOrder(String json) throws Exception;

    public static native String hashLiquidate(String json) throws Exception;

    public static native String hashSignedOraclePrice(String json) throws Exception;

    public static native String sign(String privateKey, String msg) throws Exception;

    public static native String privateKeyFromSeed(String seed) throws Exception;

    public static native boolean isOnCurve(String x, String y) throws Exception;

    public static native String privateKeyToPublicKeyXY(String privateKey) throws Exception;

}
