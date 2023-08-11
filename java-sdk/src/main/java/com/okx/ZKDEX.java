package com.okx;

public class ZKDEX {
    static {
        System.loadLibrary("zkdex_sdk");
    }

    public static native boolean verifySignature(String sig_r, String sig_s, String pubKey, String msg);

    public static native String signWithdraw(String json, String assetId, String priKey);

    public static native String signTransfer(String json, String priKey);

    public static native String signLimitOrder(String json, String priKey);

    public static native String signLiquidate(String json, String priKey);

    public static native String signSignedOraclePrice(String json, String priKey);

    public static native String hashWithdraw(String json, String assetId);

    public static native String hashTransfer(String json);

    public static native String hashLimitOrder(String json);

    public static native String hashLiquidate(String json);

    public static native String hashSignedOraclePrice(String json);
}
