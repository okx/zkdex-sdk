package com.okx;

import java.util.Objects;

public class Signature {
    private String r;
    private String s;

    public Signature() {
    }

    public Signature(String r, String s) {
        this.r = r;
        this.s = s;
    }

    public String getR() {
        return r;
    }

    public void setR(String r) {
        this.r = r;
    }

    public String getS() {
        return s;
    }

    public void setS(String s) {
        this.s = s;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Signature signature = (Signature) o;
        return Objects.equals(r, signature.r) && Objects.equals(s, signature.s);
    }

    @Override
    public int hashCode() {
        return Objects.hash(r, s);
    }

    @Override
    public String toString() {
        return "Signature{" + "r='" + r + '\'' + ", s='" + s + '\'' + '}';
    }
}
