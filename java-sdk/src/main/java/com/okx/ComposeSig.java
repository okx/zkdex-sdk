package com.okx;

public class ComposeSig {
    private Signature signature_a;
    private Signature signature_b;

    public void setSignature_a(Signature signature_a) {
        this.signature_a = signature_a;
    }

    public void setSignature_b(Signature signature_b) {
        this.signature_b = signature_b;
    }

    public Signature getSignature_a() {
        return signature_a;
    }

    public Signature getSignature_b() {
        return signature_b;
    }
}