package com.okx;

public class RustJNI {
    static {
        System.loadLibrary("zkdex_sdk");
    }

    public static void main(String[] args) {
        init();
    }

    static native void init();
}
