package com.okx;

import java.util.Objects;

public class Publickey {
    private String x;
    private String y;

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


    @Override
    public String toString() {
        return "Publickey{" +
                "x='" + x + '\'' +
                ", y='" + y + '\'' +
                '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Publickey publickey = (Publickey) o;
        return Objects.equals(x, publickey.x) && Objects.equals(y, publickey.y);
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
