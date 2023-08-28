package com.okx;

import java.io.*;

public class ZKDEX {

    static {

        String osName = System.getProperty("os.name").toLowerCase();
        String arch = System.getProperty("os.arch").toLowerCase();
        String  fileName = "";
       if (osName.contains("mac")) {
           if ((arch.contains("amd64") || arch.contains("x86_64"))) {
               fileName = "x86_64_libzkdex_sdk.dylib";
           }else {
               fileName = "arm_libzkdex_sdk.dylib";
           }
        } else if (osName.contains("nix") || osName.contains("nux") || osName.contains("aix")) {
            fileName = "libzkdex_sdk.so";
        } else {
            System.out.println("Unsupported operating system");
            System.exit(-1);
        }

        loadLib("/tmp", fileName);
    }

    private static void loadLib(String path, String name) {
        try {
            InputStream in = ZKDEX.class.getResourceAsStream(name);
            String tmpPath = path;

            // check path whether created
            File fileOutDic = new File(tmpPath);
            if (!fileOutDic.exists()) {
                fileOutDic.mkdirs();
            }

            // create target file
            File fileOut = new File(tmpPath + "/" + name);
            if (!fileOut.exists()) {
                fileOut.createNewFile();
            }

            // copy from source file
            BufferedInputStream bufferedInputStream = new BufferedInputStream(in);
            BufferedOutputStream bufferedOutputStream = new BufferedOutputStream(new FileOutputStream(fileOut));
            byte[] buf = new byte[4096];
            while ((bufferedInputStream.read(buf)) != -1) {
                bufferedOutputStream.write(buf);
                bufferedOutputStream.flush();
            }

            // load library file
            System.load(fileOut.getAbsolutePath());
        } catch (Exception e) {
            throw new RuntimeException("loading dynamic library failed", e);
        }
    }

    /**
     * verify a signature
     * @param sig_r r of signature
     * @param sig_s s of signature
     * @param pubKeyX public key x
     * @param pubKeyY public key y
     * @param msg hash of msg
     * @return bool
     */
    public static native boolean verifySignature(String sig_r, String sig_s, String pubKeyX,String pubKeyY, String msg);

    /**
     * sign a Withdraw
     * @param json json of Withdraw
     * @param assetId assetId
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signWithdraw(String json, String assetId, String priKey) throws Exception;

    /**
     * sign a Transfer
     * @param json json of Transfer
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signTransfer(String json, String priKey) throws Exception;

    /**
     * sign a LimitOrder
     * @param json json of LimitOrder
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signLimitOrder(String json, String priKey) throws Exception;

    /**
     * sign a Liquidate
     * @param json json of Liquidate
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signLiquidate(String json, String priKey) throws Exception;

    /**
     * sign a SignedOraclePrice
     * @param json json of SignedOraclePrice
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signSignedOraclePrice(String json, String priKey) throws Exception;

    /**
     * hash a Withdraw
     * @param json json of Withdraw
     * @param assetId assetId
     * @return hash
     * @throws Exception
     */
    public static native String hashWithdraw(String json, String assetId) throws Exception;

    /**
     * hash a Transfer
     * @param json json of Transfer
     * @return hash
     * @throws Exception
     */
    public static native String hashTransfer(String json) throws Exception;

    /**
     * hash a LimitOrder
     * @param json json of LimitOrder
     * @return hash
     * @throws Exception
     */
    public static native String hashLimitOrder(String json) throws Exception;

    /**
     * hash a Liquidate signature
     * @param json json of Liquidate
     * @return hash
     * @throws Exception
     */
    public static native String hashLiquidate(String json) throws Exception;

    /**
     * hash a SignedOraclePrice
     * @param json json of SignedOraclePrice
     * @return hash of SignedOraclePrice
     * @throws Exception
     */
    public static native String hashSignedOraclePrice(String json) throws Exception;

    /**
     * sign a msg
     * @param privateKey
     * @param msg hash of msg
     * @return signature
     * @throws Exception
     */
    public static native String sign(String privateKey, String msg) throws Exception;

    /**
     * derive a private key from a rand seed
     * @param seed random string
     * @return private key string
     * @throws Exception
     */
    public static native String privateKeyFromSeed(String seed) throws Exception;

    /**
     * check public key xw is on curve
     * @param x public key x
     * @param y public key y
     * @return boolean
     * @throws Exception
     */
    public static native boolean isOnCurve(String x, String y) throws Exception;

    /**
     * derive publick key from private key
     * @param privateKey
     * @return public key with xy
     * @throws Exception
     */
    public static native String privateKeyToPublicKeyXY(String privateKey) throws Exception;

}
