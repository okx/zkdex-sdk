package com.okx;

import lombok.extern.slf4j.Slf4j;

import java.io.*;
import java.nio.file.Files;

@Slf4j
public class ZKDEX {


    private static final String VERSION = "0.1.2";

    private static final String LIB_NAME = "libzkdex_sdk";

    private static final String ARM_MAC_LIB_NAME = "arm_" + LIB_NAME + "_" + VERSION + ".dylib";
    private static final String X86_MAC_LIB_NAME = "x86_64_" + LIB_NAME + "_" + VERSION + ".dylib";

    private static final String X86_LINUX_LIB_NAME = LIB_NAME + "_" + VERSION + ".so";
    private static final String X86_WIN_LIB_NAME = LIB_NAME + "_" + VERSION + ".dll";

    static {

        String osName = System.getProperty("os.name").toLowerCase();
        String arch = System.getProperty("os.arch").toLowerCase();
        String fileName = "";
        if (osName.contains("mac")) {
            if ((arch.contains("amd64") || arch.contains("x86_64"))) {
                fileName = X86_MAC_LIB_NAME;
            } else {
                fileName = ARM_MAC_LIB_NAME;
            }
        } else if (osName.contains("nix") || osName.contains("nux") || osName.contains("aix")) {
            fileName = X86_LINUX_LIB_NAME;
        } else if (osName.contains("win")) {
            fileName = X86_WIN_LIB_NAME;
        } else {
            log.error("{}", "Unsupported operating system");
            System.exit(-1);
        }


        if (osName.contains("win")) {
            try {
                loadLib(System.getProperty("java.io.tmpdir"), fileName);
            } catch (Exception e) {
                e.printStackTrace();
                log.error("[loadLib] try load lib from {} failed: {}", System.getProperty("java.io.tmpdir"), e.toString());
                System.exit(-1);
            }
        } else {
            try {
                loadLib("/tmp", fileName);
            } catch (Exception e) {
                log.error("[loadLib] try load lib from /tmp failed: {}", e.toString());
                log.info("[loadLib] try load lib from /home/admin/zk-lib again");
                loadLib("/home/admin/zk-lib", fileName);
            }
        }
    }



    private static void loadLib(String path, String name) {

        InputStream in = null;
        try {
            in = ZKDEX.class.getResourceAsStream(name);
            String tmpPath = path;

            // check path whether created
            File fileOutDic = new File(tmpPath);
            if (!fileOutDic.exists()) {
                fileOutDic.mkdirs();
            }

            // create target file
            File fileOut = new File(tmpPath + File.separator + name);
            if (fileOut.exists()) {
                fileOut.delete();
                log.info("[loadLib] delete old lib file: {}", fileOut.getAbsolutePath());
            }

            // auto create file and copy from source to it
            Files.copy(in, fileOut.toPath());

            // load library file
            System.load(fileOut.getAbsolutePath());
        } catch (Exception e) {
            log.error("[loadLib] e: ", e.toString());
            throw new RuntimeException("loading dynamic library failed", e);
        } finally {
            if (in != null) {
                try {
                    in.close();
                } catch (IOException e) {
                    log.error("[loadLib] e: ", e.toString());
                }
            }
        }
    }

    /**
     * verify a signature
     *
     * @param sig_r   r of signature
     * @param sig_s   s of signature
     * @param pubKeyX public key x
     * @param pubKeyY public key y
     * @param msg     hash of msg
     * @return bool
     */
    public static native boolean verifySignature(String sig_r, String sig_s, String pubKeyX, String pubKeyY, String msg) throws Exception;

    /**
     * sign a Withdraw
     *
     * @param json   json of Withdraw
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signWithdraw(String json, String priKey) throws Exception;

    /**
     * sign a Transfer
     *
     * @param json   json of Transfer
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signTransfer(String json, String priKey) throws Exception;

    /**
     * sign a LimitOrder
     *
     * @param json   json of LimitOrder
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signLimitOrder(String json, String priKey) throws Exception;

    /**
     * sign a Liquidate
     *
     * @param json   json of Liquidate
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signLiquidate(String json, String priKey) throws Exception;

    /**
     * sign a SignedOraclePrice
     *
     * @param json   json of SignedOraclePrice
     * @param priKey private key
     * @return signature
     * @throws Exception
     */
    public static native String signSignedOraclePrice(String json, String priKey) throws Exception;

    /**
     * hash a Withdraw
     *
     * @param json json of Withdraw
     * @return hash
     * @throws Exception
     */
    public static native String hashWithdraw(String json) throws Exception;

    /**
     * hash a Transfer
     *
     * @param json json of Transfer
     * @return hash
     * @throws Exception
     */
    public static native String hashTransfer(String json) throws Exception;

    /**
     * hash a LimitOrder
     *
     * @param json json of LimitOrder
     * @return hash
     * @throws Exception
     */
    public static native String hashLimitOrder(String json) throws Exception;

    /**
     * hash a Liquidate signature
     *
     * @param json json of Liquidate
     * @return hash
     * @throws Exception
     */
    public static native String hashLiquidate(String json) throws Exception;

    /**
     * hash a SignedOraclePrice
     *
     * @param json json of SignedOraclePrice
     * @return hash of SignedOraclePrice
     * @throws Exception
     */
    public static native String hashSignedOraclePrice(String json) throws Exception;

    /**
     * sign a msg
     *
     * @param privateKey
     * @param msg        hash of msg
     * @return signature
     * @throws Exception
     */
    public static native String sign(String privateKey, String msg) throws Exception;

    /**
     * sign a eth address
     *
     * @param privateKey
     * @param msg        hash of msg
     * @return EthAddressSignature @see{com.okx.EthAddressSignature}
     * @throws Exception
     */
    public static native String ethSign(String privateKey, String msg) throws Exception;

    /**
     * derive a private key from a rand seed
     *
     * @param seed random string
     * @return private key string
     * @throws Exception
     */
    public static native String privateKeyFromSeed(String seed) throws Exception;

    /**
     * check public key xw is on curve
     *
     * @param x public key x
     * @param y public key y
     * @return boolean
     * @throws Exception
     */
    public static native boolean isOnCurve(String x, String y) throws Exception;

    /**
     * derive publick key from private key
     *
     * @param privateKey
     * @return public key with xy
     * @throws Exception
     */
    public static native String privateKeyToPublicKeyXY(String privateKey) throws Exception;

    /**
     * convert public key to xy
     *
     * @param publicKey
     * @return
     * @throws Exception
     */
    public static native String publicKeyToXY(String publicKey) throws Exception;

}
