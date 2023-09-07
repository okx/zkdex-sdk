package com.okx;

import com.alibaba.fastjson.annotation.JSONField;

import java.util.Objects;

public class EthAddressSignature {

    private String x;
    private String y;

    private String s;

    @JSONField(name = "pk_x")
    private String pkX;

    @JSONField(name = "pk_y")
    private String pkY;

    public EthAddressSignature() {
    }

    public EthAddressSignature(String x, String y, String s, String pkX, String pkY) {
        this.x = x;
        this.y = y;
        this.s = s;
        this.pkX = pkX;
        this.pkY = pkY;
    }

    public String getX() {
        return x;
    }

    public void setX(String x) {
        this.x = x;
    }

    public String getY() {
        return y;
    }

    public void setY(String y) {
        this.y = y;
    }

    public String getS() {
        return s;
    }

    public void setS(String s) {
        this.s = s;
    }

    public String getPkX() {
        return pkX;
    }

    public void setPkX(String pkX) {
        this.pkX = pkX;
    }

    public String getPkY() {
        return pkY;
    }

    public void setPkY(String pkY) {
        this.pkY = pkY;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        EthAddressSignature that = (EthAddressSignature) o;
        return Objects.equals(x, that.x) && Objects.equals(y, that.y) && Objects.equals(s, that.s) && Objects.equals(pkX, that.pkX) && Objects.equals(pkY, that.pkY);
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, s, pkX, pkY);
    }

    @Override
    public String toString() {
        return "EthAddressSignature{" +
                "x='" + x + '\'' +
                ", y='" + y + '\'' +
                ", s='" + s + '\'' +
                ", pkX='" + pkX + '\'' +
                ", pkY='" + pkY + '\'' +
                '}';
    }
}
